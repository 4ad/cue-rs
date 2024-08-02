use std::rc::Rc;

use cue_sys;

use crate::Value;

#[derive(Debug)]
pub struct Context {
    pub(crate) res: Rc<usize>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            res: Rc::new(unsafe { cue_sys::cue_newctx() }),
        }
    }

    pub fn top(&self) -> Value {
        unsafe {
            let res = cue_sys::cue_top(*self.res);
            Value::with_context(self.clone(), res)
        }
    }

    pub fn bottom(&self) -> Value {
        unsafe {
            let res = cue_sys::cue_top(*self.res);
            Value::with_context(self.clone(), res)
        }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            res: Rc::clone(&self.res),
        }
    }
}

impl Drop for Context {
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
    use super::*;

    #[test]
    fn new() {
        let ctx = Context::new();
        assert_ne!(*ctx.res, 0);
    }

    #[test]
    fn multiple() {
        let ctx0 = Context::new();
        let ctx1 = Context::new();
        assert_ne!(*ctx0.res, *ctx1.res);
    }
}
