use std::rc::Rc;
use std::cell::RefCell;

use cue_sys;

#[derive(Debug)]
pub struct Context {
    pub(crate) res: Rc<RefCell<usize>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            res: Rc::new(RefCell::new(unsafe { cue_sys::cue_newctx() })),
        }
    }

    pub(crate) fn res(&self) -> usize {
        *self.res.borrow()
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
                cue_sys::cue_free(self.res());
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
        assert_ne!(ctx.res(), 0);
    }

    #[test]
    fn multiple() {
        let ctx0 = Context::new();
        let ctx1 = Context::new();
        assert_ne!(ctx0.res(), ctx1.res());
    }
}
