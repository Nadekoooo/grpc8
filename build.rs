fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)                      // Generate server code (optional for server-side)
        .compile(
            &["proto/services.proto"],          // File .proto yang ingin dikompilasi
            &["proto"],                         // Direktori tempat file .proto disimpan
        )?;
    Ok(())
}
