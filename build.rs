
fn main() -> std::io::Result<()> {
    // Path to the proto files
    let proto_root = "proto";

    // Path to the output file descriptor set
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let query_descriptor_path = std::path::Path::new(&out_dir).join("query_descriptor.bin");

    // Compile query.proto
    tonic_build::configure()
        .file_descriptor_set_path(&query_descriptor_path)
        .compile(
            &["proto/query.proto"], // Path to your proto file
            &[proto_root],            // Include directory for proto files
        )?;

    println!("cargo:rerun-if-changed=proto/query.proto");

    Ok(())
}

