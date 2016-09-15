#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;
extern crate libc;

use ffi::{
  nnp_initialize, nnp_deinitialize,
  pthreadpool_create, pthreadpool_get_threads_count, pthreadpool_destroy,
  pthreadpool_t,
};

use std::sync::{Arc};

lazy_static! {
  static ref NNPACK_CTX: Arc<NnpackContext> = Arc::new(NnpackContext::init());
}

pub mod ffi;

struct NnpackContext {
}

impl Drop for NnpackContext {
  fn drop(&mut self) {
    let status = unsafe { nnp_deinitialize() };
    if status.is_err() {
      panic!("failed to deinitialize NNPACK: {:?}", status);
    }
  }
}

impl NnpackContext {
  pub fn init() -> NnpackContext {
    println!("DEBUG: initializing nnpack...");
    let status = unsafe { nnp_initialize() };
    if status.is_err() {
      panic!("failed to initialize NNPACK: {:?}", status);
    }
    NnpackContext{}
  }
}

pub struct NnpackHandle {
  ctx:  Arc<NnpackContext>,
}

impl NnpackHandle {
  pub fn new() -> NnpackHandle {
    NnpackHandle{
      ctx: (*NNPACK_CTX).clone(),
    }
  }
}

pub struct NnpackPthreadPool {
  raw:      pthreadpool_t,
  num_thrs: usize,
}

impl Drop for NnpackPthreadPool {
  fn drop(&mut self) {
    unsafe { pthreadpool_destroy(self.raw) };
  }
}

impl NnpackPthreadPool {
  pub fn new(num_threads: usize) -> NnpackPthreadPool {
    let raw = unsafe { pthreadpool_create(num_threads) };
    assert!(!raw.is_null());
    assert_eq!(num_threads, unsafe { pthreadpool_get_threads_count(raw) });
    NnpackPthreadPool{
      raw:      raw,
      num_thrs: num_threads,
    }
  }

  pub unsafe fn as_raw(&self) -> pthreadpool_t {
    self.raw
  }
}
