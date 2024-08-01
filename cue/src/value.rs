use std::fmt::{Debug, Display, Formatter, Result};
use std::ptr;
use std::slice;

use cue_sys;

use crate::{Context, Error};

#[derive(Debug)]
pub struct Value {
    ctx: Context,
    res: usize
}

impl Value {
    pub(crate) unsafe fn with_context(ctx: Context, res: usize) -> Self {
        Value {
            ctx: ctx,
            res: res,
        }
    }
}

impl Value {
    fn to_json(&self) -> String {
        let mut buf_ptr = ptr::null_mut();
        let mut len: usize = 0;

        unsafe {
            let err = cue_sys::cue_dec_json(self.res, &mut buf_ptr, &mut len);
            if err != 0 {
                return Error::from_res(err).to_string()
            }

            let slice = slice::from_raw_parts(buf_ptr as *const u8, len);
            let s = String::from_utf8_lossy(slice).into_owned();
            cue_sys::libc_free(buf_ptr);

            s
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO: print CUE syntax, not JSON.
        write!(f, "{}", self.to_json())
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            cue_sys::cue_free(self.res);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn to_json() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "1");
        assert_eq!(v.unwrap().to_json(), "1");

        let v = crate::compile(&ctx, "{ x: 1 }");
        assert_eq!(v.unwrap().to_json(), "{\"x\":1}");
    }
}
