use crate::generated::{
    google_api,
    google_monitoring_v3::{
        self, metric_service_client::MetricServiceClient, typed_value, CreateTimeSeriesRequest,
    },
};
use thiserror::Error;
use tonic::transport::{Channel, ClientTlsConfig};

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

#[derive(Debug, Clone, Default)]
pub struct Options {
    credentials_path: Option<String>,
}

impl Options {
    pub fn credentials(self, path: impl AsRef<str>) -> Self {
        Self {
            credentials_path: Some(path.as_ref().to_string()),
        }
    }

    pub fn credentials_options(self, credentials_path: Option<String>) -> Self {
        Self { credentials_path }
    }
}

pub struct Client {
    options: Options,
    channel: Channel,
}

impl Client {
    pub async fn create_with_default(
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Client::create(Options::default()).await
    }

    pub async fn create(
        options: Options,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = "https://monitoring.googleapis.com".parse().unwrap();
        let channel = Channel::builder(uri)
            .tls_config(ClientTlsConfig::new())?
            .connect()
            .await?;

        Ok(Self { options, channel })
    }

    pub async fn create_time_series(
        &self,
        project_id: &str,
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
            tonic_ext::interceptor(&self.options),
        );

        if let Err(status) = client.create_time_series(tonic::Request::new(req)).await {
            return Err(Error::Grpc(status));
        }

        Ok(())
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
