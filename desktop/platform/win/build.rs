fn main() {
	let profile = std::env::var("CARGO_PROFILE").or_else(|_| std::env::var("PROFILE")).unwrap();
	println!("cargo:rustc-env=CARGO_PROFILE={profile}");

	let cef_dir = std::env::var("DEP_CEF_DLL_WRAPPER_CEF_DIR").unwrap();
	println!("cargo:rustc-env=CEF_PATH={cef_dir}");

	#[cfg(all(target_os = "windows", not(feature = "bundle")))]
	{
		let mut res = winres::WindowsResource::new();
		res.set_icon("../../assets/graphite-icon-color.ico");
		res.compile().expect("Failed to compile Windows resources");
	}
}
