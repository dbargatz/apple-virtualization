use crate::bindings;

pub struct DispatchQueue {
    inner: bindings::NSObject,
}

impl DispatchQueue {
    pub fn new(name: &str) -> Self {
        let label_cstr = name.as_ptr() as *const std::os::raw::c_char;
        let null_attrs = bindings::NSObject(0 as bindings::id);
        let queue = unsafe { bindings::dispatch_queue_create(label_cstr, null_attrs) };

        Self { inner: queue }
    }

    fn _create_block<F>(closure: F) -> *mut std::os::raw::c_void
    where
        F: Fn() + 'static,
    {
        let dispatch_block = block::ConcreteBlock::new(closure);
        let dispatch_block = dispatch_block.copy();
        let dispatch_block: &block::Block<(), ()> = &dispatch_block;
        let dispatch_block_ptr: *mut std::os::raw::c_void =
            dispatch_block as *const _ as *mut std::os::raw::c_void;

        dispatch_block_ptr
    }

    pub fn as_object(&self) -> bindings::NSObject {
        self.inner
    }

    pub async fn dispatch_async<F>(&self, closure: F)
    where
        F: Fn() + 'static,
    {
        let block_ptr = Self::_create_block(closure);
        unsafe {
            bindings::dispatch_async(self.inner, block_ptr);
        }
    }

    pub fn dispatch_sync<F>(&self, closure: F)
    where
        F: Fn() + 'static,
    {
        let block_ptr = Self::_create_block(closure);
        unsafe {
            bindings::dispatch_sync(self.inner, block_ptr);
        }
    }
}
