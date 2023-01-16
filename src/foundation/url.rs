use crate::bindings::{self, INSURL};

use super::NSString;

pub struct NSURL(bindings::NSURL);
impl NSURL {
    pub fn new(path: &str) -> Self {
        let str = NSString::new(path);
        let inner = unsafe { bindings::NSURL::fileURLWithPath_(str.into_inner()) };
        Self(inner)
    }

    pub fn into_inner(self) -> bindings::NSURL {
        self.0
    }
}
