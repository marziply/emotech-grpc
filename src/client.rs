use crate::server::service::service_client::ServiceClient;
use crate::server::ADDRESS;
use crate::Data;
use std::error::Error;
use tonic::codegen::CompressionEncoding;
use tonic::Request;

pub async fn send_data(input: Data) -> Result<(), Box<dyn Error>> {
  let addr = format!("http://{ADDRESS}");
  let client = ServiceClient::connect(addr).await?;
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

// I have decided to omit these due to time constraints. One could argue
// that these tests are redundant anyway with the tests in the server
// module.
// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[tokio::test]
//   async fn send_string() {}
//
//   #[tokio::test]
//   async fn send_number() {}
//
//   #[tokio::test]
//   async fn send_file() {}
// }
