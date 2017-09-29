extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("ext/skein/c_skein.c")
        .compile("libskein.a");
}
