#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub mod errors;
pub mod utils;

use libc::{c_char, c_void};
use libloading::Symbol;
use std::ffi::CString;
use utils::CoreOption;
include!("bindings.rs");

pub struct MeoAssistant {
    lib: libloading::Library,
    handle: Option<AsstHandle>,
}

unsafe extern "C" fn asst_create_ex_cb(_id: i32, raw_ptr: *const i8, void_ptr: *mut c_void) {
    let cb: &mut &mut dyn FnMut(&str) = unsafe { &mut *(void_ptr as *mut _) };
    let content = unsafe {
        let content_bytes = CString::from_raw(raw_ptr as *mut i8)
            .as_bytes_with_nul()
            .to_vec();
        String::from_utf8(content_bytes).unwrap_or(String::from("{\"error\": \"Parse failed\"}"))
    };
    cb(content.as_str());
}

impl MeoAssistant {
    fn load_lib(path: &str) -> Self {
        let lib = unsafe {
            libloading::Library::new(path).expect("Error occurred while loading library: ")
        };

        Self { lib, handle: None }
    }

    pub fn load_resource(&self, path: &str) -> Result<(), errors::Error> {
        let result = unsafe {
            let path_cstr = CString::new(path).expect("CString::new failed");
            let func = self
                .lib
                .get::<Symbol<extern "C" fn(*const c_char) -> u8>>(b"AsstLoadResource\0")?;
            func(path_cstr.as_ptr())
        };
        if result != 0 {
            Ok(())
        } else {
            Err(crate::errors::Error::ResourceLoadFailed)
        }
    }
    pub fn set_static_option(&self, key: i32, value: &str) -> Result<(), errors::Error> {
        let result = unsafe {
            let func = self
                .lib
                .get::<Symbol<extern "C" fn(i32, *const c_char) -> u8>>(b"AsstSetStaticOption\0")?;
            let key_cstr = CString::new(value).expect("CString::new failed");
            func(key, key_cstr.as_ptr())
        };
        if result != 0 {
            Ok(())
        } else {
            Err(crate::errors::Error::ResourceLoadFailed)
        }
    }

    pub fn create(&mut self) -> Result<(), errors::Error> {
        let result = unsafe {
            let func = self
                .lib
                .get::<Symbol<extern "C" fn() -> AsstHandle>>(b"AsstCreate")?;
            func()
        };
        if result.is_null() {
            Err(errors::Error::AsstCreateFailed)
        } else {
            Ok(self.handle = Some(result))
        }
    }
    pub fn create_with_callback<Callback>(
        &mut self,
        callback: Callback,
    ) -> Result<(), errors::Error>
    where
        Callback: FnMut(&str) + Send + Sync,
    {
        let result =
            unsafe {
                let cb = Box::new(callback);
                let leaked_cb = Box::leak(cb);
                let func = self.lib.get::<Symbol<
                    extern "C" fn(AsstApiCallback, *mut c_void) -> AsstHandle,
                >>(b"AsstCreateEx\0")?;
                func(Some(asst_create_ex_cb), leaked_cb as *mut _ as *mut c_void)
            };
        if result.is_null() {
            Err(errors::Error::AsstCreateFailed)
        } else {
            Ok(self.handle = Some(result))
        }
    }

    pub fn destroy(&self) -> Result<(), errors::Error> {
        unsafe {
            let func = self
                .lib
                .get::<Symbol<extern "C" fn() -> c_void>>(b"AsstDestroy\0")?;
            func();
        }
        Ok(())
    }

    pub fn set_instance_option(&self, opt: CoreOption) -> Result<(), errors::Error> {
        let result = unsafe {
            let func = self
                .lib
                .get::<Symbol<extern "C" fn(i32, *const c_char) -> u8>>(b"AsstSetStaticOption\0")?;

            func(opt.key(), opt.value_cstr().as_ptr())
        };
        if result != 0 {
            Ok(())
        } else {
            Err(crate::errors::Error::SetCoreOptionFailed)
        }
    }
}
