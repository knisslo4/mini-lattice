use std::net::SocketAddr;
use std::sync::Arc;
use std::pin::Pin;

use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use arrow_flight::flight_service_server::FlightServiceServer;
use arrow_flight::flight_service_server::FlightService;
use arrow_flight::{FlightData, Ticket, Result as FlightResult, SchemaResult, Action, PutResult, FlightInfo};
use tonic::IntoStreamingRequest;
use tonic::{transport::Server, Request, Response, Status};
use arrow_flight::utils::flight_data_to_arrow_batch;

use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::empty;

struct MyFlightService {}

#[tonic::async_trait]
impl FlightService for MyFlightService {
    type HandshakeStream = Pin<Box<dyn IntoStreamingRequest::Stream<Item = Result<FlightData, Status>> + Send + Sync + 'static>>;
    type ListFlightsStream = Pin<Box<dyn IntoStreamingRequest::Stream<Item = Result<FlightInfo, Status>> + Send + Sync + 'static>>;
    type DoPutStream = Pin<Box<dyn IntoStreamingRequest::Stream<Item = Result<PutResult, Status>> + Send + Sync + 'static>>;
    type DoExchangeStream = Pin<Box<dyn IntoStreamingRequest::Stream<Item = Result<FlightData, Status>> + Send + Sync + 'static>>;
    type DoActionStream = Pin<Box<dyn IntoStreamingRequest::Stream<Item = Result<FlightResult, Status>> + Send + Sync + 'static>>;
    type ListActionsStream = Pin<Box<dyn IntoStreamingRequest::Stream<Item = Result<Action, Status>> + Send + Sync + 'static>>;
    // streaming type - saying our server will get FlightData messages back
    type DoGetStream = ReceiverStream<Result<FlightData, Status>>;

    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("name", DataType::Utf8, false),
        ]));

        // this is a test batch of data
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int32Array::from(vec![1, 2, 3])),
                Arc::new(StringArray::from(vec!["A", "B", "C"])),
            ],
        ).unwrap();

        let (schema_flight_data, batch_flight_data) = 
            flight_data_to_arrow_batch(&batch, &schema, false)
            .map_err(|e| Status::internal(e.to_string()))?;

        let (tx, rx) = tokio::sync::mpsc::channel(10);
        tx.send(Ok(schema_flight_data)).await.unwrap();
        for data in batch_flight_data {
            tx.send(Ok(data)).await.unwrap();
        }

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    // we need all of them for this to work
    async fn handshake(
        &self,
        _: Request<tonic::Streaming<FlightData>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        Ok(Response::new(Box::pin(empty())))
    }

    async fn list_flights(
        &self,
        _: Request<()>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        Ok(Response::new(Box::pin(empty())))
    }

    async fn get_flight_info(
        &self,
        _: Request<arrow_flight::FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        Err(Status::unimplemented("get_flight_info not implemented"))
    }

    async fn poll_flight_info(
        &self,
        _: Request<arrow_flight::FlightInfo>,
    ) -> Result<Response<FlightInfo>, Status> {
        Err(Status::unimplemented("poll_flight_info not implemented"))
    }

    async fn get_schema(
        &self,
        _: Request<arrow_flight::FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        Err(Status::unimplemented("get_schema not implemented"))
    }

    async fn do_put(
        &self,
        _: Request<tonic::Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        Ok(Response::new(Box::pin(empty())))
    }

    async fn do_exchange(
        &self,
        _: Request<tonic::Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Ok(Response::new(Box::pin(empty())))
    }

    async fn do_action(
        &self,
        _: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        Ok(Response::new(Box::pin(empty())))
    }

    async fn list_actions(
        &self,
        _: Request<()>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        Ok(Response::new(Box::pin(empty())))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let service = MyFlightService {};

    println!("Flight Service listening on {}", addr);
    
    Server::builder()
        .add_service(FlightServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
