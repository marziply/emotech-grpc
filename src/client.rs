use crate::server::service::service_client::ServiceClient;
use crate::Data;
use std::error::Error;
use tonic::Request;

const ADDRESS: &str = "http://0.0.0.0:50051";

pub async fn send_data(input: Data) -> Result<(), Box<dyn Error>> {
  let mut client = ServiceClient::connect(ADDRESS).await?;
  let req = Request::new(input.into());
  let res = client.send_data(req).await?;

  println!("{res:#?}");

  Ok(())
}
