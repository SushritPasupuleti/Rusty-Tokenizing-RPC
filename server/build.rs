use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // tonic_build::compile_protos("./proto/token.proto")?;
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("token_descriptor.bin")) // <- For Reflection
        .compile(&["proto/token.proto"], &["proto"])?;
    Ok(())
}
