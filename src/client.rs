use crate::server::service::service_client::ServiceClient;
use crate::Data;
use std::error::Error;
use tonic::codegen::CompressionEncoding;
use tonic::Request;

const ADDRESS: &str = "http://0.0.0.0:50051";

pub async fn send_data(input: Data) -> Result<(), Box<dyn Error>> {
  let client = ServiceClient::connect(ADDRESS).await?;
  let req = Request::new(input.into());

  println!("Sending data");

  let res = client
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip)
    .send_data(req)
    .await?;

  println!("Data sent: {res:#?}");

  Ok(())
}
