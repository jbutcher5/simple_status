fn main(){
    // Link X11 lib at compile-time

    println!("cargo:rustc-link-lib=X11");
}
