# Emotech GRPC Exercise

A single binary application that can serve two separate processes to
exchange various datatypes.

## Usage

### Server

```sh
cargo run -- server
```

### Client

```sh
cargo run -- client send string "hello world!"
cargo run -- client send number 123
cargo run -- client send file ./data.bin
```

### Generate random data file

```sh
./gen.sh
```

## Test

```sh
cargo test
```
