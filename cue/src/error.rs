use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter, Result};

use cue_sys;

#[derive(Debug)]
pub struct Error {
    res: usize,
}

impl Error {
    pub(crate) unsafe fn from_res(res: usize) -> Self {
        Error {
            res: res,
        }
    }
}

impl Error {
    fn to_string(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(cue_sys::cue_error_string(self.res));
            c_str.to_str().unwrap().to_owned()
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            cue_sys::cue_free(self.res);
        }
    }
}