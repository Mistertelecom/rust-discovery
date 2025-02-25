fn main() {
    // Check for Npcap SDK path in environment
    let npcap_sdk_path = match std::env::var("NPCAP_SDK_PATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("\nERROR: NPCAP_SDK_PATH environment variable not set!");
            eprintln!("To fix this:");
            eprintln!("1. Download and install Npcap SDK from https://npcap.com/#download");
            eprintln!("2. Set the NPCAP_SDK_PATH environment variable to point to the SDK installation");
            eprintln!("   Example (Windows): setx NPCAP_SDK_PATH \"C:\\Program Files\\NpcapSDK\"");
            eprintln!("3. Restart your terminal/IDE and try building again\n");
            std::process::exit(1);
        }
    };

    // Use the correct path to the library directory
    let lib_path = std::path::Path::new(&npcap_sdk_path)
        .join("Lib")
        .join("x64")
        .canonicalize()
        .expect("Failed to get canonical path")
        .to_string_lossy()
        .replace("\\\\?\\", ""); // Remove Windows extended path prefix


    // Tell cargo to link against the Npcap SDK
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=Packet");
    println!("cargo:rustc-link-lib=static=wpcap");
    
    // Print the library search path for debugging
    println!("cargo:warning=Npcap SDK lib path: {}", lib_path);



}
