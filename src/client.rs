use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::cached::CachedDate;
use crate::generated::{
    google_api,
    google_monitoring_v3::{
        self, metric_service_client::MetricServiceClient, typed_value, CreateTimeSeriesRequest,
        DeleteMetricDescriptorRequest,
    },
};
use futures::{stream::StreamExt, Stream};
use thiserror::Error;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Code,
};

#[derive(Debug, Clone)]
pub struct TypedResource {
    pub r#type: String,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricKind {
    Cumulative,
    Gauge,
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Int64,
    Double,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub value: f64,
    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub metric: TypedResource,
    pub resource: TypedResource,
    pub metric_kind: MetricKind,
    pub value_type: ValueType,
    pub points: Point,
}

fn to_timestamp(datetime: chrono::DateTime<chrono::Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_nanos() as i32,
    }
}

/// According to GCP, a metric start time can't be more than 25 hours in the past.
const DURATION_25_HOURS: Duration = Duration::from_secs(25 * 3_600);

pub struct ListMetricDescriptorsOptions {
    credential_path: Option<String>,
    filter: String,
    page_size: i32,
}

impl Default for ListMetricDescriptorsOptions {
    fn default() -> Self {
        Self {
            credential_path: None,
            filter: String::default(),
            page_size: 500,
        }
    }
}

impl ListMetricDescriptorsOptions {
    pub fn filter(self, filter: impl AsRef<str>) -> Self {
        Self {
            filter: filter.as_ref().to_string(),
            ..self
        }
    }

    pub fn page_size(self, page_size: i32) -> Self {
        Self { page_size, ..self }
    }

    pub fn credentials(self, path: impl AsRef<str>) -> Self {
        Self {
            credential_path: Some(path.as_ref().to_string()),
            ..self
        }
    }

    pub fn credentials_options(self, credential_path: Option<String>) -> Self {
        Self {
            credential_path,
            ..self
        }
    }
}

impl TimeSeries {
    fn as_wire_record(self, cached_date: &mut CachedDate) -> google_monitoring_v3::TimeSeries {
        if cached_date.elapsed() >= DURATION_25_HOURS {
            cached_date.reset();
        }

        let end_time = to_timestamp(self.points.created);
        let start_time = if self.metric_kind == MetricKind::Gauge {
            end_time.clone()
        } else {
            to_timestamp(cached_date.time())
        };

        let metric_kind = match self.metric_kind {
            MetricKind::Cumulative => {
                crate::generated::google_api::metric_descriptor::MetricKind::Cumulative
            }
            MetricKind::Gauge => crate::generated::google_api::metric_descriptor::MetricKind::Gauge,
        };

        let (value_type, unit) = match self.value_type {
            ValueType::Int64 => (
                crate::generated::google_api::metric_descriptor::ValueType::Int64,
                "INT64".to_string(),
            ),
            ValueType::Double => (
                crate::generated::google_api::metric_descriptor::ValueType::Double,
                "DOUBLE".to_string(),
            ),
        };

        let value = match self.value_type {
            ValueType::Int64 => typed_value::Value::Int64Value(self.points.value as i64),
            ValueType::Double => typed_value::Value::DoubleValue(self.points.value),
        };

        google_monitoring_v3::TimeSeries {
            metric: Some(google_api::Metric {
                r#type: self.metric.r#type,
                labels: self.metric.labels,
            }),

            resource: Some(google_api::MonitoredResource {
                r#type: self.resource.r#type,
                labels: self.resource.labels,
            }),

            metadata: None,

            metric_kind: metric_kind.into(),
            value_type: value_type.into(),

            points: vec![google_monitoring_v3::Point {
                interval: Some(google_monitoring_v3::TimeInterval {
                    end_time: Some(end_time),
                    start_time: Some(start_time),
                }),

                value: Some(google_monitoring_v3::TypedValue { value: Some(value) }),
            }],

            unit,
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unmapped gRPC error: {0}")]
    Grpc(tonic::Status),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Initialization error: {0}")]
    InitializationError(String),
}

#[derive(Debug, Clone)]
pub struct Options {
    credentials_path: Option<String>,
    batch_size: usize,
    period: Duration,
    retries: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            credentials_path: None,
            batch_size: 200,
            period: Duration::from_secs(10),
            retries: 3,
        }
    }
}

impl Options {
    pub fn credentials(self, path: impl AsRef<str>) -> Self {
        Self {
            credentials_path: Some(path.as_ref().to_string()),
            ..self
        }
    }

    pub fn credentials_options(self, credentials_path: Option<String>) -> Self {
        Self {
            credentials_path,
            ..self
        }
    }

    pub fn batch_size(self, batch_size: usize) -> Self {
        Self { batch_size, ..self }
    }

    pub fn period(self, period: Duration) -> Self {
        Self { period, ..self }
    }

    pub fn retries(self, retries: usize) -> Self {
        Self { retries, ..self }
    }
}

#[derive(Clone)]
pub struct Client {
    channel: Channel,
}

impl Client {
    pub async fn new() -> crate::Result<Self> {
        let uri = "https://monitoring.googleapis.com".parse().unwrap();
        let channel = Channel::builder(uri)
            .tls_config(ClientTlsConfig::new())
            .map_err(|e| Error::InitializationError(e.to_string()))?
            .connect()
            .await
            .map_err(|e| Error::InitializationError(e.to_string()))?;

        Ok(Self { channel })
    }

    async fn create_time_series(
        &self,
        project_id: &str,
        options: &Options,
        time_series: Vec<google_monitoring_v3::TimeSeries>,
    ) -> crate::Result<()> {
        if time_series.len() > 200 {
            return Err(Error::InvalidArgument(format!(
                "Time series list is greater than 200, got {}",
                time_series.len()
            )));
        }

        let req = CreateTimeSeriesRequest {
            name: format!("projects/{}", project_id),
            time_series,
        };

        let mut client = MetricServiceClient::with_interceptor(
            self.channel.clone(),
            tonic_ext::interceptor(&options),
        );

        if let Err(status) = client.create_time_series(tonic::Request::new(req)).await {
            return Err(Error::Grpc(status));
        }

        Ok(())
    }

    pub async fn stream_time_series<S>(&self, project_id: &str, options: &Options, mut stream: S)
    where
        S: Stream<Item = TimeSeries> + Unpin,
    {
        let mut buffer = HashMap::<String, TimeSeries>::with_capacity(options.batch_size);
        let mut last_time = Instant::now();
        let mut total_metrics = 0usize;
        let mut successes = 0usize;
        let mut failures = 0usize;
        let mut cached_date = CachedDate::new();
        let started = Instant::now();

        while let Some(series) = stream.next().await {
            total_metrics += 1;
            buffer
                .entry(series.metric.r#type.clone())
                .and_modify(|cur| match cur.metric_kind {
                    MetricKind::Cumulative => {
                        cur.points.value += series.points.value;
                    }
                    MetricKind::Gauge => {
                        cur.points.value = series.points.value;
                    }
                })
                .or_insert(series);

            if buffer.len() < options.batch_size && last_time.elapsed() < options.period {
                continue;
            }

            loop {
                if last_time.elapsed() >= options.period {
                    break;
                }

                tokio::time::sleep(Duration::from_millis(500)).await;
            }

            let series = buffer
                .drain()
                .map(|t| t.1.as_wire_record(&mut cached_date))
                .collect::<Vec<_>>();

            let mut attempts = 1usize;
            loop {
                if let Err(e) = self
                    .create_time_series(project_id, options, series.clone())
                    .await
                {
                    if let Error::Grpc(status) = &e {
                        if (status.code() == Code::Internal || status.code() == Code::Unknown)
                            && attempts < options.retries
                        {
                            tokio::time::sleep(Duration::from_millis(500)).await;
                            attempts += 1;
                            continue;
                        }
                    }

                    failures += 1;
                    error!("Error when sending time_series: {}", e);
                } else {
                    successes += 1;
                    last_time = Instant::now();
                }

                break;
            }

            let total = successes + failures;

            let success_rate = if total == 0 {
                100f64
            } else {
                (successes as f64 / total as f64) * 100f64
            };

            let metrics_processing = total_metrics as f64 / started.elapsed().as_secs_f64();

            debug!(
                "Success rate: {:.2}%, Metric processing speed: {:.2}metrics/s",
                success_rate, metrics_processing
            );
        }
    }

    pub fn list_metric_descriptors(
        &self,
        project_id: impl AsRef<str>,
        options: &ListMetricDescriptorsOptions,
    ) -> ListMetricDescriptors {
        ListMetricDescriptors::new(self.clone(), project_id.as_ref().to_string(), options)
    }

    pub async fn delete_metric_descriptor(
        &self,
        name: impl AsRef<str>,
        options: &Options,
    ) -> crate::Result<()> {
        let req = DeleteMetricDescriptorRequest {
            name: name.as_ref().to_string(),
        };

        let mut client = MetricServiceClient::with_interceptor(
            self.channel.clone(),
            tonic_ext::interceptor(&options),
        );

        if let Err(status) = client.delete_metric_descriptor(tonic::Request::new(req)).await {
            return Err(Error::Grpc(status));
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct LabelDescriptor {
    pub key: String,
    pub value_type: i32,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct MetricDescriptorMetadata {
    pub sample_period: Option<Duration>,
    pub ingest_delay: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct MetricDescriptor {
    pub name: String,
    pub r#type: String,
    pub labels: Vec<LabelDescriptor>,
    pub metric_kind: i32,
    pub value_type: i32,
    pub unit: String,
    pub description: String,
    pub display_name: String,
    pub metadata: Option<MetricDescriptorMetadata>,
    pub launch_stage: i32,
    pub monitored_resource_types: Vec<String>,
}

pub struct ListMetricDescriptors {
    credentials_path: Option<String>,
    page_size: i32,
    filter: String,
    project_id: String,
    first_time: bool,
    client: Client,
    buffer: Vec<crate::generated::google_api::MetricDescriptor>,
    next_page_token: Option<String>,
}

impl ListMetricDescriptors {
    fn new(client: Client, project_id: String, options: &ListMetricDescriptorsOptions) -> Self {
        Self {
            credentials_path: options.credential_path.clone(),
            project_id,
            first_time: true,
            page_size: options.page_size,
            filter: options.filter.to_string(),
            buffer: Default::default(),
            next_page_token: None,
            client,
        }
    }
}

impl ListMetricDescriptors {
    pub async fn next(&mut self) -> crate::Result<Option<MetricDescriptor>> {
        loop {
            if self.first_time {
                self.first_time = false;
                self.next_page_token = Some("".to_string());
            }

            if let Some(metric) = self.buffer.pop() {
                let metric = MetricDescriptor {
                    name: metric.name,
                    r#type: metric.r#type,
                    labels: metric
                        .labels
                        .into_iter()
                        .map(|l| LabelDescriptor {
                            key: l.key,
                            value_type: l.value_type,
                            description: l.description,
                        })
                        .collect::<Vec<_>>(),
                    metric_kind: metric.metric_kind,
                    value_type: metric.value_type,
                    unit: metric.unit,
                    description: metric.description,
                    display_name: metric.display_name,
                    metadata: metric.metadata.map(|m| MetricDescriptorMetadata {
                        sample_period: m
                            .sample_period
                            .map(|d| Duration::new(d.seconds as u64, d.nanos as u32)),
                        ingest_delay: m
                            .ingest_delay
                            .map(|d| Duration::new(d.seconds as u64, d.nanos as u32)),
                    }),
                    launch_stage: metric.launch_stage,
                    monitored_resource_types: metric.monitored_resource_types,
                };

                return Ok(Some(metric));
            }

            if let Some(page_token) = self.next_page_token.take() {
                // We create a shallow options as we don't need much of its properties.
                // TODO - I'll rewrite that part at some point.
                let options = Options {
                    credentials_path: self.credentials_path.clone(),
                    batch_size: 0,
                    period: Duration::from_secs(0),
                    retries: 1,
                };
                let mut client = MetricServiceClient::with_interceptor(
                    self.client.channel.clone(),
                    tonic_ext::interceptor(&options),
                );

                let req = crate::generated::google_monitoring_v3::ListMetricDescriptorsRequest {
                    name: format!("projects/{}", self.project_id),
                    filter: self.filter.clone(),
                    page_size: self.page_size,
                    page_token,
                };

                match client
                    .list_metric_descriptors(tonic::Request::new(req))
                    .await
                {
                    Err(status) => return Err(Error::Grpc(status)),
                    Ok(resp) => {
                        let resp = resp.into_inner();

                        self.buffer = resp.metric_descriptors;

                        self.next_page_token = if resp.next_page_token.is_empty() {
                            None
                        } else {
                            Some(resp.next_page_token)
                        };
                    }
                }

                continue;
            }

            return Ok(None);
        }
    }
}

mod tonic_ext {
    use tonic::{metadata::MetadataValue, Interceptor, Request, Status};

    macro_rules! map_err {
        ($res:expr) => {
            $res.map_err(|e| Status::unknown(e.to_string()))
        };
    }

    pub fn interceptor(options: &crate::Options) -> impl Into<Interceptor> {
        let mut token = gouth::Builder::new().scopes(&[
            "https://www.googleapis.com/auth/cloud-platform",
            "https://www.googleapis.com/auth/monitoring",
            "https://www.googleapis.com/auth/monitoring.write",
        ]);

        if let Some(path) = options.credentials_path.as_ref() {
            token = token.file(path);
        }

        let token = token.build().expect("Token::build()");

        move |mut req: Request<()>| {
            let token = map_err!(token.header_value())?;
            let meta = map_err!(MetadataValue::from_str(&*token))?;
            req.metadata_mut().insert("authorization", meta);
            Ok(req)
        }
    }
}
