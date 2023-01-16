use std::fmt;
use crate::bindings::{self, NSString_NSStringExtensionMethods, NSUTF8StringEncoding};

pub struct NSString(bindings::NSString);
impl NSString {
    pub fn new(value: &str) -> Self {
        let ptr = unsafe {
            let str_ptr = value.as_ptr() as *const std::os::raw::c_void;
            let inner = bindings::NSString::alloc();
            inner.initWithBytes_length_encoding_(str_ptr, value.len() as u64, NSUTF8StringEncoding)
        };
        let inner = bindings::NSString(ptr);
        Self(inner)
    }

    pub fn as_str(&self) -> &str {
        let len = self.len();
        let slice = unsafe {
            let bytes = self.0.UTF8String() as *const u8;
            std::slice::from_raw_parts(bytes, len)
        };
        std::str::from_utf8(slice).unwrap()
    }

    pub fn into_inner(self) -> bindings::NSString {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { self.0.lengthOfBytesUsingEncoding_(NSUTF8StringEncoding) as usize }
    }
}

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<bindings::id> for NSString {
    fn from(p: bindings::id) -> Self {
        NSString::from(bindings::NSString(p))
    }
}

impl From<bindings::NSString> for NSString {
    fn from(p: bindings::NSString) -> Self {
        NSString(p)
    }
}

#[cfg(test)]
mod tests {
    use super::NSString;

    #[test]
    fn new_works() {
        let _ = NSString::new("this is a test string");
    }

    #[test]
    fn as_str_works() {
        let test_str = "this is a test string";
        let nsstr = NSString::new(test_str);
        assert_eq!(test_str, nsstr.as_str());

        let test_str = "this is a test string with an emoji ⛔";
        let nsstr = NSString::new(test_str);
        assert_eq!(test_str, nsstr.as_str());
    }

    #[test]
    fn len_works() {
        let test_str = "this is a test string";
        let nsstr = NSString::new(test_str);
        assert_eq!(test_str.len(), nsstr.len());

        let test_str = "this is a test string with an emoji ⛔";
        let nsstr = NSString::new(test_str);
        assert_eq!(test_str.len(), nsstr.len());
    }
}
