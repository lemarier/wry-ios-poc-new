fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("-ios") {
        println!("cargo:rustc-link-lib=framework=UIKit");
        println!("cargo:rustc-link-lib=framework=WebKit");
    }
}
