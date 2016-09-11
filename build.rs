use std::env;
use std::fs::{create_dir_all};
use std::path::{PathBuf};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let target_triple = env::var("TARGET").unwrap();

  let mut artifacts_path = PathBuf::from(&manifest_dir);
  artifacts_path.push(&format!("artifacts.{}", target_triple));
  create_dir_all(&artifacts_path).ok();

  let mut native_lib_path = PathBuf::from(&artifacts_path);
  native_lib_path.push("libnnpack_native.a");
  if !native_lib_path.exists() {
    unimplemented!();
  }

  println!("cargo:rustc-link-search=native={}", artifacts_path.to_str().unwrap());
  //println!("cargo:rustc-link-lib=static=nnpack_native");
}
