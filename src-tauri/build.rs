fn main() {
  cc::Build::new()
    .include("vendor/mhycrypto")
    .cpp(true)
    .file("vendor/mhycrypto/memecrypto.cpp")
    .file("vendor/mhycrypto/metadata.cpp")
    .file("vendor/mhycrypto/metadatastringdec.cpp")
    .compile("mhycrypto");

  cc::Build::new()
    .include("vendor/mhycrypto")
    .file("vendor/mhycrypto/aes.c")
    .compile("mhycrypto-aes");

  tauri_build::build()
}
