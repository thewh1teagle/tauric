use std::env;
use std::process::Command;

fn macos_link_search_path() -> Option<String> {
  let output = Command::new("clang").arg("--print-search-dirs").output().ok()?;
  if !output.status.success() {
      println!("failed to run 'clang --print-search-dirs', continuing without a link search path");
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  for line in stdout.lines() {
      if line.contains("libraries: =") {
          let path = line.split('=').nth(1)?;
          return Some(format!("{}/lib/darwin", path));
      }
  }

  println!("failed to determine link search path, continuing without it");
  None
}

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;

    tauri_build::build();
    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file("tauri.h");

      if target.contains("apple") {
        // On (older) OSX we need to link against the clang runtime,
        // which is hidden in some non-default path.
        //
        // More details at https://github.com/alexcrichton/curl-rust/issues/279.
        if let Some(path) = macos_link_search_path() {
            println!("cargo:rustc-link-lib=clang_rt.osx");
            println!("cargo:rustc-link-search={}", path);
        }
    }
}

