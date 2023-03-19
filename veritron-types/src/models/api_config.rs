use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ApiConfig {
    #[serde(default = "default_api_rest")]
    pub rest: Option<RestApiOptions>,

    #[serde(default = "default_api_grpc")]
    pub grpc: Option<GrpcApiOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_app_grpc")]
    pub app_grpc: Option<GrpcApiOptions>,
}

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct RestApiOptions {
    #[serde(default = "default_rest_port")]
    pub port: u32,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_cors")]
    pub cors: bool,
}

#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct GrpcApiOptions {
    #[serde(default = "default_grpc_port")]
    pub port: u32,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_cors")]
    pub cors: bool,
    #[serde(default = "default_enable_web")]
    pub web: bool,
}

fn default_app_grpc_port() -> u32 {
    50053
}
fn default_app_grpc_host() -> String {
    "0.0.0.0".to_owned()
}

pub(crate) fn default_app_grpc() -> Option<GrpcApiOptions> {
    Some(GrpcApiOptions {
        port: default_app_grpc_port(),
        host: default_app_grpc_host(),
        cors: false,
        web: false,
    })
}
pub(crate) fn default_api_rest() -> Option<RestApiOptions> {
    Some(RestApiOptions {
        port: default_rest_port(),
        host: default_host(),
        cors: default_cors(),
    })
}
pub(crate) fn default_api_grpc() -> Option<GrpcApiOptions> {
    Some(GrpcApiOptions {
        port: default_grpc_port(),
        host: default_host(),
        cors: default_cors(),
        web: default_enable_web(),
    })
}
fn default_grpc_port() -> u32 {
    50051
}
fn default_rest_port() -> u32 {
    8080
}
fn default_enable_web() -> bool {
    true
}
fn default_cors() -> bool {
    true
}

fn default_host() -> String {
    "0.0.0.0".to_owned()
}
pub fn default_api_config() -> ApiConfig {
    ApiConfig {
        rest: default_api_rest(),
        grpc: default_api_grpc(),
        app_grpc: default_app_grpc(),
    }
}
