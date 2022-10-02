use std::env;
use std::path::PathBuf;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("my_descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .build_server(true)
        .compile(&["proto/hello.proto"], &["proto/"])?;

    Ok(())
}