fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Android 16KB page size support (arm64 and x86_64).
    // See https://developer.android.com/guide/practices/page-sizes
    if target_os == "android" && (target_arch == "aarch64" || target_arch == "x86_64") {
        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
    }
}
