fn main() {
    println!("cargo:rerun-if-changed=src/objgrad.c");
    cc::Build::new()
        .file("src/objgrad.c")
        .flag("-Wno-unused-variable")
        .compile("libobjgrad.a");
}
