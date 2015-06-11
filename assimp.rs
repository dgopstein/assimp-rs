#[link(name = "assimp",
	vers = "0.1",
	uuid = "9fd3d600-20b0-11e3-8224-0800200c9a66",
	author = "Tomasz Stachowiak")]

//#[comment = "Bindings and wrapper functions for AssImp."]
#[crate_type = "lib"]
#[feature(globs)]

// TODO: Document differences between GLFW and glfw-rs

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


//#[deriving(Eq, IterBytes)]
pub struct Scene {
    ptr: *mut ffi::aiScene,
}


impl Scene {
	pub fn load( filename: &str, flags: u32 ) -> Result<Scene, &str> {
		unsafe {
			filename.with_c_str (|fname| {
        let scene_ptr = ffi::aiImportFile( fname, flags );
        Ok( Scene { ptr: scene_ptr } )
				//ffi::aiImportFile( fname, flags )
				//	.to_option().map_default(Err( "aiImportFile returned null" ),
				//		| ptr | Ok(
				//			Scene { ptr: &*ptr as *mut isize }
				//		)
				//	)
			})
		}
	}
}
