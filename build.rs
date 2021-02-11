extern crate cmake;
extern crate curl;
extern crate num_cpus;

use curl::easy::Easy;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::thread;

/// Build from https://wasm-stat.us that we are known to work with.
#[cfg(target_os="linux")]
mod config {
    pub const DOWLOAD_WASM: bool = true;
    pub const WASM_BUILD: &'static str = "14533";
    pub const OS_NAME: &'static str = "linux";
}

#[cfg(target_os="macos")]
mod config {
    pub const DOWLOAD_WASM: bool = true;
    pub const WASM_BUILD: &'static str = "2670";
    pub const OS_NAME: &'static str = "mac";
}

#[cfg(not(any(target_os="linux", target_os="macos")))]
mod config {
    pub const DOWLOAD_WASM: bool = false;
    pub const WASM_BUILD: &'static str = "";
    pub const OS_NAME: &'static str = "";
}

use config::WASM_BUILD;

fn main() {
    let cmake = thread::spawn(|| {
        if !Path::new("binaryen/.git").exists() {
            Command::new("git")
                .args(&["submodule", "update", "--init"])
                .status()
                .expect("error updating submodules");
        }
        cmake::Config::new("binaryen")
            .define("BUILD_STATIC_LIB", "ON")
            .build_arg(format!("-j{}", num_cpus::get() + 2))
            .build()
    });

    let toolchain = thread::spawn(|| {
        if config::DOWLOAD_WASM {
            update_wasm_toolchain();
        }
    });

    let dst = cmake.join().unwrap();
    toolchain.join().expect("Error downloading Wasm toolchain");

    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=binaryen");
    println!("cargo:rustc-link-lib=static=passes");
    println!("cargo:rustc-link-lib=static=support");
    println!("cargo:rustc-link-lib=static=emscripten-optimizer");
    println!("cargo:rustc-link-lib=static=asmjs");
    println!("cargo:rustc-link-lib=static=wasm");
    println!("cargo:rustc-link-lib=static=ast");

    print_deps(Path::new("binaryen"));
}

fn print_deps(path: &Path) {
    for e in path.read_dir().unwrap().filter_map(|e| e.ok()) {
        let file_type = e.file_type().unwrap();
        if file_type.is_dir() {
            print_deps(&e.path());
        } else {
            println!("cargo:rerun-if-changed={}", e.path().display());
        }
    }
}

/// Downloads the wasm toolchain from https://wasm-stat.us/ if necessary.
fn update_wasm_toolchain() {
    const WASM_INSTALL_VER: &'static str = ".wasm-install-ver";

    // Check if the right version is already in .wasm-install-ver
    if let Ok(mut file) = File::open(WASM_INSTALL_VER) {
        let mut contents = String::new();
        if let Ok(_) = file.read_to_string(&mut contents) {
            if WASM_BUILD == contents.trim() {
                return;
            }
        }
    }

    // If we got here, we need to update.
    const TMP_FILE: &'static str = ".wasm-install.tbz2";

    let url = wasm_url();
    let url = url.as_str();

    fetch_url(url, TMP_FILE);
    Command::new("tar")
        .args(&["xjf", TMP_FILE])
        .status()
        .and_then(|_| File::create(WASM_INSTALL_VER))
        .and_then(|mut file| writeln!(file, "{}", WASM_BUILD))
        .expect("error downloading wasm toolchain");
}

fn fetch_url(url: &str, output: &str) {
    File::create(output).and_then(|mut file| {
        let mut curl = Easy::new();
        curl.url(url).expect("Error setting url");
        curl.write_function(move |data| Ok(file.write(data).expect("Error writing data")))
            .expect("Error setting write function");
        curl.perform().expect("Error downloading archive");
        Ok(())
    }).expect("Could not open output file");
}

fn wasm_url() -> String {
    format!("https://storage.googleapis.com/wasm-llvm/builds/{}/{}/wasm-binaries-{}.tbz2",
            config::OS_NAME,
            config::WASM_BUILD,
            config::WASM_BUILD)
}
