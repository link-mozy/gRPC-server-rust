use std::net::SocketAddr;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use service::{calc_program::calc_program_server::CalcProgramServer, CalcProgramService};

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "[::1]:8080".parse().unwrap();
    let calc_service: CalcProgramService = CalcProgramService::default();

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(["http://127.0.0.1".parse()?,"http://localhost:3000".parse()?]);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(CalcProgramServer::new(calc_service))
        .serve(address)
        .await?;
    Ok(())
}