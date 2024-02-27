{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  name = "sd-env";
  LD_LIBRARY_PATH = lib.makeLibraryPath [ gcc-unwrapped zlib libglvnd glib ];
  buildInputs = [
    protobuf3_20
	protoc-gen-rust
	protoc-gen-grpc-web
	evans # grpc client
  ];
}
