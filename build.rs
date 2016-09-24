use std::env;
use std::fs::{create_dir_all};
use std::path::{PathBuf};
use std::process::{Command};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();
  //let target_triple = env::var("TARGET").unwrap();

  let mut venv_path = PathBuf::from(&out_dir);
  venv_path.push("env");
  if !venv_path.exists() {
    Command::new("virtualenv")
      .current_dir(&out_dir)
      .arg("-p").arg("python2.7")
      .arg(venv_path.to_str().unwrap())
      .status().unwrap();
  }

  let mut python = PathBuf::from(&venv_path);
  python.push("bin");
  python.push("python");
  let mut pip = PathBuf::from(&venv_path);
  pip.push("bin");
  pip.push("pip");

  let mut ninja = PathBuf::from(&out_dir);
  ninja.push("ninja-build");
  ninja.push("ninja");
  if !ninja.exists() {
    let mut ninja_src_path = PathBuf::from(&manifest_dir);
    ninja_src_path.push("ninja");
    let mut ninja_build_path = PathBuf::from(&out_dir);
    ninja_build_path.push("ninja-build");
    create_dir_all(&ninja_build_path).ok();
    let mut ninja_config_path = PathBuf::from(&ninja_src_path);
    ninja_config_path.push("configure.py");
    Command::new(python.to_str().unwrap())
      .current_dir(ninja_build_path.to_str().unwrap())
      .arg(ninja_config_path.to_str().unwrap())
      .arg("--bootstrap")
      .status().unwrap();
    /*Command::new("cp")
      .arg(ninja_build_path.join("ninja"))
      .arg(ninja.to_str().unwrap())
      .status().unwrap();*/
    Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install")
      .arg("--upgrade")
      .arg("ninja-syntax")
      .status().unwrap();
  }

  let mut peachpy_build_path = PathBuf::from(&out_dir);
  peachpy_build_path.push("PeachPy-build");
  if !peachpy_build_path.exists() {
    let mut peachpy_src_path = PathBuf::from(&manifest_dir);
    peachpy_src_path.push("PeachPy");
    Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(peachpy_src_path.to_str().unwrap())
      .arg(peachpy_build_path.to_str().unwrap())
      .status().unwrap();
    let mut peachpy_req_path = PathBuf::from(&peachpy_build_path);
    peachpy_req_path.push("requirements.txt");
    Command::new(pip.to_str().unwrap())
      .current_dir(&out_dir)
      .arg("install")
      .arg("--upgrade")
      .arg("-r")
      .arg(peachpy_req_path.to_str().unwrap())
      .status().unwrap();
    let mut peachpy_setup_path = PathBuf::from(&peachpy_build_path);
    peachpy_setup_path.push("setup.py");
    Command::new(python.to_str().unwrap())
      .current_dir(&peachpy_build_path)
      .arg(peachpy_setup_path.to_str().unwrap())
      .arg("generate")
      .status().unwrap();
    Command::new(pip.to_str().unwrap())
      .current_dir(peachpy_build_path.to_str().unwrap())
      .arg("install")
      .arg("--upgrade")
      .arg(peachpy_build_path.to_str().unwrap())
      .status().unwrap();
  }

  let mut native_lib_path = PathBuf::from(&out_dir);
  native_lib_path.push("libnnpack_native.a");
  if !native_lib_path.exists() {
    let mut nnpack_src_path = PathBuf::from(&manifest_dir);
    nnpack_src_path.push("NNPACK");
    let mut nnpack_build_path = PathBuf::from(&out_dir);
    nnpack_build_path.push("NNPACK-build");
    //create_dir_all(&nnpack_build_path).ok();
    Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(nnpack_src_path.to_str().unwrap())
      .arg(nnpack_build_path.to_str().unwrap())
      .status().unwrap();
    let mut nnpack_config_path = PathBuf::from(&nnpack_build_path);
    nnpack_config_path.push("configure.py");
    Command::new(python.to_str().unwrap())
      .current_dir(nnpack_build_path.to_str().unwrap())
      .arg(nnpack_config_path.to_str().unwrap())
      .status().unwrap();
    Command::new(ninja.to_str().unwrap())
      .current_dir(nnpack_build_path.to_str().unwrap())
      .status().unwrap();
    let mut nnpack_lib_path = PathBuf::from(&nnpack_build_path);
    nnpack_lib_path.push("lib");
    nnpack_lib_path.push("libnnpack.a");
    Command::new("cp")
      .current_dir(&out_dir)
      .arg(nnpack_lib_path.to_str().unwrap())
      .arg(native_lib_path.to_str().unwrap())
      .status().unwrap();
  }

  /*let mut artifacts_path = PathBuf::from(&manifest_dir);
  artifacts_path.push(&format!("artifacts.{}", target_triple));
  create_dir_all(&artifacts_path).ok();

  let mut native_lib_path = PathBuf::from(&artifacts_path);
  native_lib_path.push("libnnpack_native.a");
  if !native_lib_path.exists() {
    unimplemented!();
  }*/

  //println!("cargo:rustc-link-lib=static=nnpack_native");
  //println!("cargo:rustc-link-search=native={}", artifacts_path.to_str().unwrap());
  println!("cargo:rustc-link-search=native={}", out_dir);
}
