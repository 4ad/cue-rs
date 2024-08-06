use std::cmp::Eq;
use std::convert::{From, TryFrom};
use std::ffi;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::slice;

use cue_sys;

use crate::{Context, Error, Kind, Resource};

#[derive(Debug, Clone)]
pub struct Value {
    ctx: Context,
    res: Rc<Resource>,
}

impl Value {
    pub(crate) unsafe fn from_resource_with_context(ctx: Context, res: Resource) -> Self {
        Value {
            ctx,
            res: Rc::new(res),
        }
    }

    pub(crate) unsafe fn from_raw_with_context(ctx: Context, res: usize) -> Self {
        Value::from_resource_with_context(ctx, Resource::from_raw(res))
    }

    pub(crate) fn as_raw(&self) -> usize {
        self.res.as_raw()
    }

    pub fn unify(&self, other: &Self) -> Self {
        unsafe {
            let res = cue_sys::cue_unify(self.as_raw(), other.as_raw());
            Self::from_raw_with_context(self.ctx.clone(), res)
        }
    }

    pub fn top() -> Self {
        Context::new().top()
    }

    pub fn bottom() -> Self {
        Context::new().top()
    }

    pub fn kind(&self) -> Kind {
        unsafe {
            let kind = cue_sys::cue_concrete_kind(self.as_raw());
            Kind::from(kind)
        }
    }

    pub fn incomplete_kind(&self) -> Kind {
        unsafe {
            let kind = cue_sys::cue_incomplete_kind(self.as_raw());
            Kind::from(kind)
        }
    }
}

impl Value {
    fn to_json(&self) -> String {
        let mut buf_ptr = ptr::null_mut();
        let mut len: usize = 0;

        unsafe {
            let err = cue_sys::cue_dec_json(self.as_raw(), &mut buf_ptr, &mut len);
            if err != 0 {
                return Error::from_raw(err).to_string();
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
            let res = cue_sys::cue_from_bool(ctx.as_raw(), item);
            Self::from_raw_with_context(ctx, res)
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut res = false;
        unsafe {
            let err = cue_sys::cue_dec_bool(value.as_raw(), &mut res);
            if err != 0 {
                return Err(Error::from_raw(err));
            }
        }
        Ok(res)
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
            let res = cue_sys::cue_from_int64(ctx.as_raw(), item);
            Self::from_raw_with_context(ctx, res)
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut res: i64 = 0;
        unsafe {
            let err = cue_sys::cue_dec_int64(value.as_raw(), &mut res);
            if err != 0 {
                return Err(Error::from_raw(err));
            }
        }
        Ok(res)
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
            let res = cue_sys::cue_from_uint64(ctx.as_raw(), item);
            Self::from_raw_with_context(ctx, res)
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut res: u64 = 0;
        unsafe {
            let err = cue_sys::cue_dec_uint64(value.as_raw(), &mut res);
            if err != 0 {
                return Err(Error::from_raw(err));
            }
        }
        Ok(res)
    }
}

impl From<&str> for Value {
    fn from(item: &str) -> Self {
        let ctx = Context::new();
        let str_ptr = CString::new(item).unwrap().into_raw();
        unsafe {
            let res = cue_sys::cue_from_string(ctx.as_raw(), str_ptr);
            Self::from_raw_with_context(ctx, res)
        }
    }
}

impl From<String> for Value {
    fn from(item: String) -> Self {
        Value::from(item.as_str())
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut buf_ptr = ptr::null_mut();
        unsafe {
            let err = cue_sys::cue_dec_string(value.as_raw(), &mut buf_ptr);

            if err != 0 {
                return Err(Error::from_raw(err));
            }

            let s = CStr::from_ptr(buf_ptr).to_string_lossy().into_owned();
            cue_sys::libc_free(buf_ptr as *mut ffi::c_void);

            Ok(s)
        }
    }
}

impl From<f32> for Value {
    fn from(item: f32) -> Self {
        Value::from(item as f64)
    }
}

impl From<f64> for Value {
    fn from(item: f64) -> Self {
        let ctx = Context::new();
        unsafe {
            let res = cue_sys::cue_from_double(ctx.as_raw(), item);
            Self::from_raw_with_context(ctx, res)
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let mut res: f64 = 0.0;
        unsafe {
            let err = cue_sys::cue_dec_double(value.as_raw(), &mut res);
            if err != 0 {
                return Err(Error::from_raw(err));
            }
        }
        Ok(res)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        unsafe { cue_sys::cue_is_equal(self.as_raw(), other.as_raw()) }
    }
}
impl Eq for Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: print CUE syntax, not JSON.
        write!(f, "{}", self.to_json())
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
    fn from_int() {
        let v = Value::from(1);
        assert_eq!(v.to_json(), "1");

        let v = Value::from(-1);
        assert_eq!(v.to_json(), "-1");

        let n: i64 = 1234567890111213;
        let v = Value::from(n);
        assert_eq!(v.to_json(), "1234567890111213");
    }

    #[test]
    fn from_unsigned() {
        let u: u64 = 0xdeadbeef;
        let v = Value::from(u);
        assert_eq!(v.to_json(), "3735928559");
    }

    #[test]
    fn to_bool() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "true").unwrap();
        assert_eq!(bool::try_from(v).unwrap(), true);

        let v = crate::compile(&ctx, "1").unwrap();
        assert_eq!(
            bool::try_from(v).unwrap_err().to_string(),
            "cannot use value 1 (type int) as bool"
        );
    }

    #[test]
    fn to_int() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "1").unwrap();
        assert_eq!(i64::try_from(v).unwrap(), 1);

        let v = crate::compile(&ctx, "true").unwrap();
        assert_eq!(
            i64::try_from(v).unwrap_err().to_string(),
            "cannot use value true (type bool) as int"
        );
    }

    #[test]
    fn to_unsigned() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "0xdeadbeef").unwrap();
        assert_eq!(u64::try_from(v).unwrap(), 0xdeadbeef);

        let v = crate::compile(&ctx, "true").unwrap();
        assert_eq!(
            u64::try_from(v).unwrap_err().to_string(),
            "cannot use value true (type bool) as int"
        );
    }

    #[test]
    fn eq() {
        let v0 = Value::from(true);
        let v1 = Value::from(true);
        assert_eq!(v0, v1);

        let v0 = Value::from(1);
        let v1 = Value::from(2);
        assert_ne!(v0, v1);

        let v0 = Value::from(true);
        let v1 = Value::from(1);
        assert_ne!(v0, v1);
    }

    #[test]
    fn unify() {
        let ctx = Context::new();

        let v0 = crate::compile(&ctx, "int").unwrap();
        let v1 = crate::compile(&ctx, "1").unwrap();
        let r = v0.unify(&v1);
        assert_eq!(r, v1);
    }

    #[test]
    fn from_string() {
        let v = Value::from("hello");
        assert_eq!(v.to_json(), "\"hello\"");

        let s = String::from("world");
        let v = Value::from(s);
        assert_eq!(v.to_json(), "\"world\"");
    }

    #[test]
    fn to_string() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "\"hello\"").unwrap();
        assert_eq!(String::try_from(v).unwrap(), "hello");

        let v = crate::compile(&ctx, "int").unwrap();
        assert_eq!(
            String::try_from(v).unwrap_err().to_string(),
            "cannot use value int (type int) as string"
        );
    }

    #[test]
    fn from_float() {
        let v = Value::from(1.2);
        assert_eq!(v.to_json(), "1.2");

        let v = Value::from(-1.2);
        assert_eq!(v.to_json(), "-1.2");

        let n: f32 = 1.2345000505447388;
        let v = Value::from(n);
        assert_eq!(v.to_json(), "1.2345000505447388");
    }

    #[test]
    fn to_float() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "1.0").unwrap();
        assert_eq!(f64::try_from(v).unwrap(), 1.0);

        let v = crate::compile(&ctx, "true").unwrap();
        assert_eq!(
            f64::try_from(v).unwrap_err().to_string(),
            "cannot use value true (type bool) as number"
        );
    }

    #[test]
    fn kind() {
        let ctx = Context::new();

        let v = crate::compile(&ctx, "int").unwrap();
        assert_eq!(v.incomplete_kind(), Kind::Int);

        let v = crate::compile(&ctx, "1").unwrap();
        assert_eq!(v.kind(), Kind::Int);

        let v = crate::compile(&ctx, "float64").unwrap();
        assert_eq!(v.incomplete_kind(), Kind::Number);

        let v = crate::compile(&ctx, "1.0").unwrap();
        assert_eq!(v.kind(), Kind::Float);

        let v = crate::compile(&ctx, "string").unwrap();
        assert_eq!(v.incomplete_kind(), Kind::String);

        let v = crate::compile(&ctx, "\"hello\"").unwrap();
        assert_eq!(v.kind(), Kind::String);

        let v = crate::compile(&ctx, "bool").unwrap();
        assert_eq!(v.incomplete_kind(), Kind::Bool);

        let v = crate::compile(&ctx, "true").unwrap();
        assert_eq!(v.kind(), Kind::Bool);

        let v = crate::compile(&ctx, "null").unwrap();
        assert_eq!(v.kind(), Kind::Null);

        let v = crate::compile(&ctx, "[int, bool]").unwrap();
        assert_eq!(v.incomplete_kind(), Kind::List);

        let v = crate::compile(&ctx, "{ x: int }").unwrap();
        assert_eq!(v.incomplete_kind(), Kind::Struct);

        let v = crate::compile(&ctx, "{ x: 1 }").unwrap();
        assert_eq!(v.kind(), Kind::Struct);
    }
}
