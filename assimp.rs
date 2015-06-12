/// Bindings and wrapper functions for AssImp.

extern crate libc;
extern crate c_str;

use c_str::{FromCStr, ToCStr};
use std::ptr;
use std::str;
//use std::vec;

// re-export constants
pub use consts::*;

pub mod ffi;
pub mod consts;
//mod private;


use std::ops::Deref;
impl Deref for Scene {
  type Target = *mut ffi::aiScene;

  fn deref<'a>(&'a self) -> &'a *mut ffi::aiScene {
    &self.ptr
  }
}

pub struct Scene {
    pub ptr: *mut ffi::aiScene,
}


impl Scene {
	pub fn load( filename: &str, flags: u32 ) -> Result<*mut ffi::aiScene, String> {
    unsafe {
      let c_filename = std::ffi::CString::new(filename);
      let maybe_scene = ffi::aiImportFile( c_filename.unwrap().as_ptr(), flags );
      let err_msg = format!("Could not load file: {}", filename);

      maybe_scene.ok_or(err_msg)
    }
	}
}
