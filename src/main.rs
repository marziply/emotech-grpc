pub mod client;
pub mod server;

use clap::{Parser, Subcommand};
use client::send_data;
use server::start_server;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct App {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Debug, Subcommand)]
enum Sender {
  #[clap(subcommand)]
  r#Send(Data),
}

#[derive(Debug, Subcommand)]
enum Data {
  r#String { data: String },
  Number { data: i32 },
  File { path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum Command {
  Client {
    #[clap(subcommand)]
    data: Sender,
  },
  Server,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let App { command } = App::parse();

  match command {
    Command::Client { data } => send_data(data),
    Command::Server => start_server().await?,
  };

  Ok(())
}
