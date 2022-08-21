fn main() {
    println!("cargo:rerun-if-changed=src/objgrad.c");
    cc::Build::new()
        .file("src/objgrad.c")
        .compile("libobjgrad.a");
}
