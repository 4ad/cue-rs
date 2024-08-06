use std::rc::Rc;

use cue_sys;

use crate::{Resource, Value};

#[derive(Debug, Clone)]
pub struct Context {
    res: Rc<Resource>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            res: Rc::new(unsafe { Resource::from_raw(cue_sys::cue_newctx()) }),
        }
    }

    pub(crate) fn as_raw(&self) -> usize {
        self.res.as_raw()
    }

    pub fn top(&self) -> Value {
        unsafe {
            let res = cue_sys::cue_top(self.as_raw());
            Value::from_raw_with_context(self.clone(), res)
        }
    }

    pub fn bottom(&self) -> Value {
        unsafe {
            let res = cue_sys::cue_top(self.as_raw());
            Value::from_raw_with_context(self.clone(), res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let ctx = Context::new();
        assert_ne!(ctx.as_raw(), 0);
    }

    #[test]
    fn multiple() {
        let ctx0 = Context::new();
        let ctx1 = Context::new();
        assert_ne!(ctx0.as_raw(), ctx1.as_raw());
    }
}
