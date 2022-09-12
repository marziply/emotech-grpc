use clap::{Parser, Subcommand};
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

fn main() {
  let app = App::parse();

  println!("{app:#?}");
}
