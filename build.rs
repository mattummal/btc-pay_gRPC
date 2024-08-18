fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)  // Optional: Generates server code
        .build_client(true)  // Optional: Generates client code
        .compile(
            &["proto/payment/payment.proto"],
            &["proto/payment"],
        )?;
    Ok(())
}
