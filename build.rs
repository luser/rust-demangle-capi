extern crate cheddar;

fn main() {
    cheddar::Cheddar::new().expect("could not read manifest")
        .run_build("target/include/rust_demangle.h");
}
