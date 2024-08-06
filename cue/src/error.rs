use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;

use cue_sys;

use crate::Resource;

#[derive(Debug, Clone)]
pub struct Error {
    res: Rc<Resource>,
}

impl Error {
    pub(crate) unsafe fn from_resource(res: Resource) -> Self {
        Error { res: Rc::new(res) }
    }

    pub(crate) unsafe fn from_raw(res: usize) -> Self {
        Error::from_resource(Resource::from_raw(res))
    }

    pub(crate) fn as_raw(&self) -> usize {
        self.res.as_raw()
    }
}

impl Error {
    fn to_string(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr(cue_sys::cue_error_string(self.as_raw()));
            c_str.to_str().unwrap().to_owned()
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}
