use cmake;

fn main() {
    let proj_output = cmake::Config::new("anotherLib").build();
    println!("cargo:rustc-link-search=native={}", proj_output.display());
    println!("cargo:rustc-link-lib=static=MyMathLib");
    println!("cargo:rust-link-lib=static=MyMathLib2");
}
