use tonic::{transport::Server, Request, Response, Status};

use futures::Stream;
use std::{pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use hello::{
    HelloRequest,
    HelloResponse,
    HealthCheckRequest,
    HealthCheckResponse,
    health_check_response::ServingStatus,
    hello_service_server::{
        HelloServiceServer, 
        HelloService,
    },
};

pub mod hello {
    tonic::include_proto!("hello");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct HelloServiceImplementation {
}

#[tonic::async_trait]
impl HelloService for HelloServiceImplementation {
  type WatchStream = ResponseStream;  

  async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
    let r = request.into_inner();

    Ok(Response::new(hello::HelloResponse {
        message: format!("Hello, {}!", r.name)
    }))
  }
  
  async fn check(&self, request: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
    println!("Health check");
    println!("\tclient connected from: {:?}", request.remote_addr());

    Ok(Response::new(hello::HealthCheckResponse {
        status: ServingStatus::Serving as i32,
    }))
  }

  async fn watch(&self, request: Request<HealthCheckRequest>) -> Result<Response<ResponseStream>, Status> {
    println!("Health watch");
    println!("Client connected from: {:?}", request.remote_addr());

    let repeat = std::iter::repeat(hello::HealthCheckResponse {
        status: ServingStatus::Serving as i32,
    });

    let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(2000)));

    let (tx, rx) = mpsc::channel(128);
    tokio::spawn(async move {
        while let Some(item) = stream.next().await {
            match tx.send(Result::<_, Status>::Ok(item)).await {
                Ok(_) => {
                    // item (server response) was queued to be send to client
                }
                Err(_item) => {
                    // output_stream was build from rx and both are dropped
                    break;
                }
            }
        }
        println!("Client disconnected from health watch: {:?}", request.remote_addr());
    });

    let output_stream = ReceiverStream::new(rx);

    Ok(Response::new(
        Box::pin(output_stream) as Self::WatchStream
    ))
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

