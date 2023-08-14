use std::net::SocketAddr;
use tonic::transport::Server;
use service::{calc_program::calc_program_server::CalcProgramServer, CalcProgramService};

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "[::1]:8080".parse().unwrap();
    let mpc_service: CalcProgramService = CalcProgramService::default();

    Server::builder()
        .add_service(CalcProgramServer::new(mpc_service))
        .serve(address)
        .await?;
    Ok(())
}