extern crate cc;

fn main() {
    cc::Build::new()
        .file("ext/skein/c_skein.c")
        .compile("libskein.a");
}
