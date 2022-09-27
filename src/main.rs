use tonic::{transport::Server, Request, Response, Status};

use hello::{HelloRequest, HelloResponse, hello_service_server::{HelloServiceServer, HelloService}};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct HelloServiceImplementation {}

#[tonic::async_trait]
impl HelloService for HelloServiceImplementation {
  async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
    let r = request.into_inner();

    Ok(Response::new(hello::HelloResponse {
        message: format!("Hello, {}!", r.name)
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:80".parse().unwrap();
    let hello_service = HelloServiceImplementation::default();

    println!("Starting server");

    let server = Server::builder().add_service(HelloServiceServer::new(hello_service))
        .serve(address);

    server
        .await?;

    Ok(())
}

