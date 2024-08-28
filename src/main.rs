
mod query_service;

use tonic::transport::Server;
use crate::query_service::query::query_service_server::QueryServiceServer;
use tokio;
use tonic_reflection::server::Builder;
use std::env;
use std::path::Path;
use dotenvy::from_path;
// Import the generated code from query_service.rs
use crate::query_service::MyQueryService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    // Retrieve the necessary values from environment variables
    let ip = env::var("QUERY_DOMAIN").expect("Missing 'domain' environment variable");
    let port = env::var("QUERY_PORT").expect("Missing 'port' environment variable");
    let addr = format!("{}:{}", ip, port).parse().unwrap();

    let query_service = MyQueryService::default();

    println!("QueryService listening on {}", addr);

    let descriptor_set = include_bytes!(concat!(env!("OUT_DIR"), "/query.bin"));
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(descriptor_set)
        .build_v1()?;

    Server::builder()
        .add_service(QueryServiceServer::new(query_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
