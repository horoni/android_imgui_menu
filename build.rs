fn main() {
	println!("cargo:rustc-link-lib=static=c++_static");
	println!("cargo:rustc-link-lib=GLESv3");
	println!("cargo:rustc-link-lib=EGL");
	println!("cargo:rustc-link-lib=android");
	println!("cargo:rustc-link-lib=log");
}
