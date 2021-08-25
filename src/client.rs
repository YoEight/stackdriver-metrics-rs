use thiserror::Error;
use tonic::{ transport::{Channel, ClientTlsConfig}, service::interceptor::InterceptedService };
use crate::generated::google_monitoring_v3::metric_service_client::MetricServiceClient;

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
    pub int64_value: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Serie<'a> {
    pub metric: TypedResource,
    pub resource: TypedResource,
    pub metric_kind: MetricKind,
    pub value_type: ValueType,
    pub points: &'a [Point],
}

#[derive(Debug, Clone)]
pub struct Series<'a> {
    time_series: &'a [Serie<'a>],
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unmapped gRPC error: {0}")]
    Grpc(tonic::Status),
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    credentials_path: Option<String>,
}

struct Auth {
    token: gouth::Token,
}

impl tonic::service::Interceptor for Auth {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        match self.token.header_value() {
            Err(e) => Err(tonic::Status::invalid_argument(format!(
                "Invalid token: {}",
                e
            ))),
            Ok(value) => match value.parse::<tonic::metadata::AsciiMetadataValue>() {
                Err(e) => Err(tonic::Status::invalid_argument(format!(
                    "Invalid header value: {}",
                    e
                ))),
                Ok(value) => {
                    req.metadata_mut().insert("authorization", value);

                    Ok(req)
                }
            },
        }
    }
}

pub struct Client {
    client: MetricServiceClient<InterceptedService<Channel, Auth>>,
}

impl Client {
    pub async fn create_with_default(
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Client::create(Options::default()).await
    }

    pub async fn create(
        options: Options,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut token = gouth::Builder::new().scopes(&[
            "https://www.googleapis.com/auth/cloud-platform,",
            "https://www.googleapis.com/auth/monitoring,",
            "https://www.googleapis.com/auth/monitoring.read,",
            "https://www.googleapis.com/auth/monitoring.write",
        ]);

        if let Some(path) = options.credentials_path.as_ref() {
            token = token.file(path);
        }

        let auth = Auth {
            token: token.build()?,
        };

        let uri = "https://monitoring.googleapis.com".parse().unwrap();
        let channel = Channel::builder(uri)
            .tls_config(ClientTlsConfig::new())?
            .connect()
            .await?;

        let client = MetricServiceClient::with_interceptor(channel, auth);

        Ok(Self {
            client,
        })
    }

    pub async fn create_time_series<'a>(&'a self, series: Series<'a>) -> crate::Result<()> {

        Ok(())
    }
}
