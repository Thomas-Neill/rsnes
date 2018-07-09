fn main() {
    println!("cargo:rustc-link-search=./libraries");
    println!("cargo:rustc-link-lib=apu");
    println!("cargo:rustc-link-lib=stdc++");
}
