use tonic::transport::Server;
use query::query_server::QueryServer;
use tokio;

// Import the query_service module
mod query_service;

// Import the generated code from query_service.rs
use query_service::QueryService;

pub mod query {
    tonic::include_proto!("query");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let query_service = QueryService::default();

    println!("QueryService listening on {}", addr);

    Server::builder()
        .add_service(QueryServer::new(query_service))
        .serve(addr)
        .await?;

    Ok(())
}
