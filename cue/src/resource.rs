#[derive(Debug)]
pub(crate) struct Resource(usize);

impl Resource {
    pub(crate) unsafe fn from_raw(res: usize) -> Self {
        Resource(res)
    }

    pub(crate) fn as_raw(&self) -> usize {
        self.0
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            cue_sys::cue_free(self.0);
        }
    }
}
