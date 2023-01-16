use std::fmt;

use crate::bindings::{self, INSDictionary, INSEnumerator};

use super::{NSError, NSString};

pub struct NSDictionary {
    inner: bindings::NSDictionary,
    key_enum: bindings::NSEnumerator,
}

impl NSDictionary {
    pub fn new() -> Self {
        let inner = bindings::NSDictionary::alloc();
        let inner = unsafe {
            let ptr = <bindings::NSDictionary as INSDictionary<bindings::id, bindings::id>>::init(&inner);
            bindings::NSDictionary(ptr)
        };
        let key_enum = unsafe {
            <bindings::NSDictionary as INSDictionary<bindings::id, bindings::id>>::keyEnumerator(&inner)
        };
        Self { inner, key_enum }
    }

    pub fn into_inner(self) -> bindings::NSDictionary {
        self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let length = unsafe {
            <bindings::NSDictionary as INSDictionary<bindings::id, bindings::id>>::count(&self.inner)
        };
        length as usize
    }
}

impl Default for NSDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NSDictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let copy = NSDictionary::from(self.inner.0);
        let mut map = f.debug_map();
        for (k, v) in copy {
            let value_class = unsafe { (*v).class() };
            let value_str = match value_class.name() {
                "__NSCFString" => {
                    let str = NSString::from(v);
                    String::from(str.as_str())
                }
                "NSError" => {
                    let err = NSError::from(v);
                    format!("{:?}", err)
                }
                _ => {
                    format!("{:?} ({})", v, value_class.name())
                }
            };
            map.entry(&k, &value_str);
        }
        map.finish()
    }
}

impl From<bindings::id> for NSDictionary {
    fn from(p: bindings::id) -> Self {
        NSDictionary::from(bindings::NSDictionary(p))
    }
}

impl From<bindings::NSDictionary> for NSDictionary {
    fn from(p: bindings::NSDictionary) -> Self {
        let key_enum = unsafe {
            <bindings::NSDictionary as INSDictionary<bindings::id, bindings::id>>::keyEnumerator(&p)
        };
        NSDictionary { inner: p, key_enum }
    }
}

impl Iterator for NSDictionary {
    type Item = (NSString, bindings::id);

    fn next(&mut self) -> Option<Self::Item> {
        let key_ptr = unsafe {
            <bindings::NSEnumerator as INSEnumerator<bindings::id>>::nextObject(&self.key_enum)
        };
        if !key_ptr.is_null() {
            let value_ptr = unsafe {
                <bindings::NSDictionary as INSDictionary<bindings::id, bindings::id>>::objectForKey_(
                    &self.inner,
                    key_ptr,
                )
            };
            let key = NSString::from(key_ptr);
            let value = value_ptr;
            Some((key, value))
        } else {
            None
        }
    }
}
