use crate::server::service::data_request::Input;
use crate::server::service::service_client::ServiceClient;
use crate::server::service::DataRequest;
use crate::Data;
use std::error::Error;
use tonic::Request;

const ADDRESS: &str = "http://0.0.0.0:50051";

pub async fn send_data(data: Data) -> Result<(), Box<dyn Error>> {
  let mut client = ServiceClient::connect(ADDRESS).await?;
  let req = Request::new(DataRequest {
    input: Some(Input::StringData(String::new())),
  });
  let res = client.send_data(req).await?;

  println!("{res:#?}");

  Ok(())
}
