use std::ffi::CString;
use std::ptr;

use cue_sys;

use crate::{Context, Error, Value};

pub fn compile(ctx: &Context, input: &str) -> Result<Value, Error> {
    let mut val = 0;
    let val_ptr = &mut val as *mut usize;
    let null_ptr = ptr::null_mut();
    let str_ptr = CString::new(input).unwrap().into_raw();

    let result = unsafe {
        let err = cue_sys::cue_compile_string(ctx.res(), str_ptr, null_ptr, val_ptr);
        if err != 0 {
            Err(Error::from_res(err))
        } else {
            Ok(Value::with_context(ctx.clone(), val))
        }
    };

    unsafe {
        // free allocated C memory.
        let _ = CString::from_raw(str_ptr);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let ctx = Context::new();
        let v = compile(&ctx, "");
        assert!(v.is_ok());
    }

    #[test]
    fn basic() {
        let ctx = Context::new();
        let v = compile(&ctx, "int");
        assert!(v.is_ok());
    }

    #[test]
    fn error() {
        let ctx = Context::new();
        let v = compile(&ctx, "<@@");

        match v {
            Ok(_) => panic!("expected error"),
            Err(e) => assert_eq!(e.to_string(), "invalid attribute: expected '('"),
        }
    }
}
