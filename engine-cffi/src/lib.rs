use std::char::from_u32_unchecked;
use std::mem::MaybeUninit;

mod ffi {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::KimeInputResultType;

#[derive(Clone, Copy, Debug)]
pub struct InputResult {
    pub ty: KimeInputResultType,
    pub char1: char,
    pub char2: char,
}

pub struct InputEngine {
    engine: *mut ffi::KimeInputEngine,
}

impl InputEngine {
    pub fn new() -> Self {
        Self {
            engine: unsafe { ffi::kime_engine_new() },
        }
    }

    pub fn press_key(&mut self, config: &Config, hardware_code: u16, state: u32) -> InputResult {
        let ret =
            unsafe { ffi::kime_engine_press_key(self.engine, config.config, hardware_code, state) };

        unsafe {
            InputResult {
                ty: ret.ty,
                char1: from_u32_unchecked(ret.char1),
                char2: from_u32_unchecked(ret.char2),
            }
        }
    }

    pub fn reset(&mut self) -> Option<char> {
        unsafe {
            match ffi::kime_engine_reset(self.engine) {
                0 => None,
                n => Some(from_u32_unchecked(n)),
            }
        }
    }
}

impl Drop for InputEngine {
    fn drop(&mut self) {
        unsafe {
            ffi::kime_engine_delete(self.engine);
        }
    }
}

pub struct Config {
    config: *mut ffi::KimeConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            config: unsafe { ffi::kime_config_load() },
        }
    }

    pub fn xim_font_name(&self) -> &str {
        unsafe {
            let mut ptr = MaybeUninit::uninit();
            let mut len = MaybeUninit::uninit();
            ffi::kime_config_xim_preedit_font(self.config, ptr.as_mut_ptr(), len.as_mut_ptr());
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                ptr.assume_init(),
                len.assume_init(),
            ))
        }
    }
    
    pub fn gtk_commit_english(&self) -> bool {
        unsafe {
            ffi::kime_config_gtk_commit_english(self.config) != 0
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            ffi::kime_config_delete(self.config);
        }
    }
}
