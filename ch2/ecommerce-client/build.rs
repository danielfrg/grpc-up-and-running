fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("../proto/product_info.proto")?;
    tonic_build::configure()
        .build_server(false)
        // .out_dir("./gen")
        .compile(&["product_info.proto"], &["../proto/"])?;
    Ok(())
}
