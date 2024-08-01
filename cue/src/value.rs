use cue_sys;

use crate::Context;

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

impl Drop for Value {
    fn drop(&mut self) {
        unsafe {
            cue_sys::cue_free(self.res);
        }
    }
}
