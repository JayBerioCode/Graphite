#![cfg_attr(feature = "app", windows_subsystem = "windows")]

#[cfg(feature = "app")]
fn main() {
	graphite_desktop::start();
}

#[cfg(feature = "bundle")]
mod bundle;
#[cfg(feature = "bundle")]
fn main() {
	bundle::main().unwrap();
}
