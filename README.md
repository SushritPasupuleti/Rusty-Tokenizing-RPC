# Rusty Tokenizing RPC

A Rust based gRPC server using Tonic which exposes a Tokenizer that splits a string into words and returns the tokens.

Useful when you have a pipeline that is constanly being hit with new data/requests, that can quickly overwhelm the throughput of the service due to poor GC performance of Python.

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

With `grpcurl`:

with authentication:

```bash
grpcurl -H "Authorization: Bearer <token>" -emit-defaults -plaintext -d '{"token": "Hello World!"}' localhost:8080 token.Token.ProcessToken
```

without authentication:

```bash
grpcurl -plaintext -d '{"token": "Hello World!"}' localhost:8080 token.Token.ProcessToken
```
