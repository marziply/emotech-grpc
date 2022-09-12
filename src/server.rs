use self::service::data_request::Input;
use self::service::data_response::Output;
use self::service::service_server::{Service, ServiceServer};
use self::service::{DataRequest, DataResponse};
use std::error::Error;
use std::net::SocketAddr;
use tonic::codegen::CompressionEncoding;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub const ADDRESS: &str = "0.0.0.0:50051";

pub mod service {
  tonic::include_proto!("emotech_service");
}

#[derive(Debug, Default)]
struct DataService;

#[tonic::async_trait]
impl Service for DataService {
  async fn send_data(
    &self,
    req: Request<DataRequest>,
  ) -> Result<Response<DataResponse>, Status> {
    let DataRequest { input } = req.get_ref();
    let output = match input.clone().unwrap() {
      Input::StringData(v) => Output::StringData(v),
      Input::NumberData(v) => Output::NumberData(v),
      Input::FileData(v) => Output::FileData(v),
    };
    let res = DataResponse {
      ok: true,
      output: Some(output),
    };

    match input.clone() {
      Some(data) => match data {
        Input::FileData(buf) => println!("Received file: {} bytes", buf.len()),
        _ => println!("Received data: {data:?}"),
      },
      None => println!("Error receiving data, corrupt input"),
    };

    Ok(Response::new(res))
  }
}

pub async fn start_server() -> Result<(), Box<dyn Error>> {
  let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
  let service = ServiceServer::new(DataService::default())
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

  println!("Server listening on {addr:#?}");

  Server::builder()
    .add_service(service)
    .serve(addr)
    .await?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::service::data_request::Input;
  use super::service::data_response::Output;
  use super::service::service_client::ServiceClient;
  use super::service::service_server::ServiceServer;
  use super::service::{DataRequest, DataResponse};
  use super::{DataService, ADDRESS};
  use futures::future::BoxFuture;
  use futures::FutureExt;
  use std::time::Duration;
  use tokio::spawn;
  use tokio::sync::oneshot::channel;
  use tokio::time::sleep;
  use tonic::transport::{Channel, Server};
  use tonic::Request;

  async fn init() -> (
    impl FnOnce() -> BoxFuture<'static, ()>,
    ServiceClient<Channel>,
  ) {
    let (tx, rx) = channel::<()>();
    let addr = ADDRESS.parse().unwrap();
    let svc = ServiceServer::new(DataService::default());
    let handle = spawn(async move {
      Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, rx.map(drop))
        .await
        .unwrap();
    });
    let done = || {
      let run = async {
        tx.send(()).unwrap();
        handle.await.unwrap();
      };

      run.boxed()
    };

    sleep(Duration::from_millis(100)).await;

    let addr = format!("http://{ADDRESS}");
    let client = ServiceClient::connect(addr)
      .await
      .unwrap();

    (done, client)
  }

  async fn test_request(data: Input) -> DataResponse {
    let (done, mut client) = init().await;
    let req = Request::new(DataRequest { input: Some(data) });
    let res = client
      .send_data(req)
      .await
      .unwrap()
      .into_inner();

    done();

    res
  }

  #[tokio::test]
  async fn receive_string() {
    let data = String::from("Hello world!");
    let input = Input::StringData(data.clone());
    let DataResponse { ok, output } = test_request(input).await;

    assert!(ok);
    assert_eq!(output.unwrap(), Output::StringData(data.clone()));
  }

  #[tokio::test]
  async fn receive_number() {
    let data = 123;
    let input = Input::NumberData(data);
    let DataResponse { ok, output } = test_request(input).await;

    assert!(ok);
    assert_eq!(output.unwrap(), Output::NumberData(data));
  }

  #[tokio::test]
  async fn receive_file() {
    let data = vec![1, 2, 3, 4, 5];
    let input = Input::FileData(data.clone());
    let DataResponse { ok, output } = test_request(input).await;

    assert!(ok);
    assert_eq!(output.unwrap(), Output::FileData(data));
  }
}
