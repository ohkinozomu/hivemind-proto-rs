fn main() {
    println!("Starting build script...");
    let result = prost_build::compile_protos(
        &["src/protos/dht.proto", "src/protos/runtime.proto", "src/protos/auth.proto"],
        &["src/protos"]
    );
    match result {
        Ok(_) => println!("Protos compiled successfully."),
        Err(e) => println!("Failed to compile protos: {:?}", e),
    }
}
