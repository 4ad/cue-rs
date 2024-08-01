use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ptr;
use std::rc::Rc;
use std::slice;

use cue_sys;

use crate::{Context, Error};

#[derive(Debug)]
pub struct Value {
    ctx: Context,
    res: Rc<usize>
}

impl Value {
    pub(crate) unsafe fn with_context(ctx: Context, res: usize) -> Self {
        Value {
            ctx: ctx,
            res: Rc::new(res),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Value {
            ctx: self.ctx.clone(),
            res: Rc::clone(&self.res),
        }
    }
}

impl Value {
    fn to_json(&self) -> String {
        let mut buf_ptr = ptr::null_mut();
        let mut len: usize = 0;

        unsafe {
            let err = cue_sys::cue_dec_json(*self.res, &mut buf_ptr, &mut len);
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

impl From<bool> for Value {
    fn from(item: bool) -> Self {
        let ctx = Context::new();
        unsafe {
            let res = cue_sys::cue_from_bool(*ctx.res, item);
            Self::with_context(ctx, res)
        }
    }
}

impl From<i8> for Value {
    fn from(item: i8) -> Self {
        Value::from(item as i64)
    }
}

impl From<i16> for Value {
    fn from(item: i16) -> Self {
        Value::from(item as i64)
    }
}

impl From<i32> for Value {
    fn from(item: i32) -> Self {
        Value::from(item as i64)
    }
}

impl From<i64> for Value {
    fn from(item: i64) -> Self {
        let ctx = Context::new();
        unsafe {
            let res = cue_sys::cue_from_int64(*ctx.res, item);
            Self::with_context(ctx, res)
        }
    }
}

impl From<u8> for Value {
    fn from(item: u8) -> Self {
        Value::from(item as u64)
    }
}

impl From<u16> for Value {
    fn from(item: u16) -> Self {
        Value::from(item as u64)
    }
}

impl From<u32> for Value {
    fn from(item: u32) -> Self {
        Value::from(item as u64)
    }
}

impl From<u64> for Value {
    fn from(item: u64) -> Self {
        let ctx = Context::new();
        unsafe {
            let res = cue_sys::cue_from_uint64(*ctx.res, item);
            Self::with_context(ctx, res)
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
        if Rc::strong_count(&self.res) == 1 {
            unsafe {
                cue_sys::cue_free(*self.res);
            }
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

    #[test]
    fn from_bool() {
        let v = Value::from(true);
        assert_eq!(v.to_json(), "true");

        let v = Value::from(false);
        assert_eq!(v.to_json(), "false");
    }

    #[test]
    fn from_i64() {
        let v = Value::from(1);
        assert_eq!(v.to_json(), "1");

        let v = Value::from(-1);
        assert_eq!(v.to_json(), "-1");

        let n: i64 = 1234567890111213;
        let v = Value::from(n);
        assert_eq!(v.to_json(), "1234567890111213");
    }

    #[test]
    fn from_u64() {
        let u: u64 = 0xdeadbeef;
        let v = Value::from(u);
        assert_eq!(v.to_json(), "3735928559");
    }
}
