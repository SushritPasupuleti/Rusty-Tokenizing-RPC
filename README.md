# RustyRPC

A Rust based gRPC server using Tonic.

## Setup

For NixOS users, you can use the provided `shell.nix` file to get a shell with all the required dependencies.

```bash
nix-shell
```

## Running the server

```bash
cd server
cargo run
```

## Development

Edit the `/proto` directory to add or remove services and messages. The protobuf files are compiled into Rust code using the `tonic-build` crate, whose behaviour can be altered in `./server/build.rs`.

## Usage

Use `evans` or `grpcurl` to interact with the server.

```bash
evans -p proto/<proto_file>.proto -p 8080
```

```bash
grpcurl -plaintext -d '{"name": "world"}' localhost:8080 <service>.<method>
```
