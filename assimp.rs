/// Bindings and wrapper functions for AssImp.

extern crate libc;
extern crate c_str;

// re-export constants
pub use consts::*;

pub mod ffi;
pub mod consts;

impl Scene {
	pub fn load( filename: &str, flags: u32 ) -> Result<&ffi::aiScene, String> {
    let c_filename = std::ffi::CString::new(filename);
    let err_msg = format!("Could not load file: {}", filename);

    unsafe {
      let scene_ptr = ffi::aiImportFile( c_filename.unwrap().as_ptr(), flags );

      if scene_ptr.is_null() {
        Err(err_msg)
      } else {
        Ok(&*scene_ptr)
      }
    }
	}
}
