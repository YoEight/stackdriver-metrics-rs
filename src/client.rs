use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::generated::{
    google_api,
    google_monitoring_v3::{
        self, metric_service_client::MetricServiceClient, typed_value, CreateTimeSeriesRequest,
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

#[derive(Debug, Clone, Copy)]
pub enum MetricKind {
    Cumulative,
    Gauge,
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Int64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub interval: Interval,
    pub value: PointValue,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy)]
pub struct PointValue {
    pub int64_value: i64,
}

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub metric: TypedResource,
    pub resource: TypedResource,
    pub metric_kind: MetricKind,
    pub value_type: ValueType,
    pub points: Point,
}

fn to_timestamp(datetime: &chrono::DateTime<chrono::Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_nanos() as i32,
    }
}

impl TimeSeries {
    fn as_wire_record(self) -> google_monitoring_v3::TimeSeries {
        let start_time = if let Some(start_time) = self.points.interval.start_time.as_ref() {
            Some(to_timestamp(start_time))
        } else {
            None
        };

        let end_time = Some(to_timestamp(&self.points.interval.end_time));
        let metric_kind = match self.metric_kind {
            MetricKind::Cumulative => 3,
            MetricKind::Gauge => 1,
        };

        let value_type = match self.value_type {
            ValueType::Int64 => 2,
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

            metric_kind,
            value_type,

            points: vec![google_monitoring_v3::Point {
                interval: Some(google_monitoring_v3::TimeInterval {
                    end_time,
                    start_time,
                }),

                value: Some(google_monitoring_v3::TypedValue {
                    value: Some(typed_value::Value::Int64Value(
                        self.points.value.int64_value,
                    )),
                }),
            }],

            unit: "INT64".to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unmapped gRPC error: {0}")]
    Grpc(tonic::Status),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
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

pub struct Client {
    channel: Channel,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = "https://monitoring.googleapis.com".parse().unwrap();
        let channel = Channel::builder(uri)
            .tls_config(ClientTlsConfig::new())?
            .connect()
            .await?;

        Ok(Self { channel })
    }

    pub async fn create_time_series(
        &self,
        project_id: &str,
        options: &Options,
        series: Vec<TimeSeries>,
    ) -> crate::Result<()> {
        if series.len() > 200 {
            return Err(Error::InvalidArgument(format!(
                "Time series list is greater than 200, got {}",
                series.len()
            )));
        }

        let time_series = series
            .into_iter()
            .map(|s| s.as_wire_record())
            .collect::<Vec<google_monitoring_v3::TimeSeries>>();

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
        let started = Instant::now();

        while let Some(series) = stream.next().await {
            total_metrics += 1;
            buffer.insert(series.metric.r#type.clone(), series);

            if buffer.len() < options.batch_size && last_time.elapsed() < options.period {
                continue;
            }

            loop {
                if last_time.elapsed() >= options.period {
                    break;
                }

                tokio::time::sleep(Duration::from_millis(500)).await;
            }

            let series = buffer.drain().map(|t| t.1).collect::<Vec<_>>();

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
