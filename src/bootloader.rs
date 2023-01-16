use crate::bindings::{ self, IVZLinuxBootLoader };
use crate::foundation::{NSString, NSURL};

pub struct VZBootLoader(bindings::VZBootLoader);
impl VZBootLoader {
    pub fn into_inner(self) -> bindings::VZBootLoader {
        self.0
    }
}

impl From<bindings::VZLinuxBootLoader> for VZBootLoader {
    fn from(p: bindings::VZLinuxBootLoader) -> Self {
        Self(p.into())
    }
}

impl From<VZLinuxBootLoader> for VZBootLoader {
    fn from(p: VZLinuxBootLoader) -> Self {
        Self::from(p.into_inner())
    }
}

pub struct VZLinuxBootLoader(bindings::VZLinuxBootLoader);
impl VZLinuxBootLoader {
    pub fn new(kernel_path: &str) -> Self {
        let inner = bindings::VZLinuxBootLoader::alloc();
        let path = NSURL::new(kernel_path);
        let inner = unsafe {
            let ptr = inner.initWithKernelURL_(path.into_inner());
            bindings::VZLinuxBootLoader(ptr)
        };
        Self(inner)
    }

    pub fn into_inner(self) -> bindings::VZLinuxBootLoader {
        self.0
    }

    pub fn with_command_line(self, command_line: &str) -> Self {
        let cmd_line = NSString::new(command_line);
        unsafe {
            self.0.setCommandLine_(cmd_line.into_inner());
        };
        self
    }

    pub fn with_initial_ramdisk_path(self, path: &str) -> Self {
        let path = NSURL::new(path);
        unsafe {
            self.0.setInitialRamdiskURL_(path.into_inner());
        };
        self
    }

    pub fn with_kernel_path(self, path: &str) -> Self {
        let path = NSURL::new(path);
        unsafe {
            self.0.setKernelURL_(path.into_inner());
        };
        self
    }
}

impl From<bindings::VZLinuxBootLoader> for VZLinuxBootLoader {
    fn from(p: bindings::VZLinuxBootLoader) -> Self {
        Self(p)
    }
}

#[cfg(test)]
mod tests {
    use super::{VZBootLoader, VZLinuxBootLoader};

    #[test]
    fn new_works() {
        let kernel_path = "/Users/user/Documents/vmlinuz";
        let _ = VZLinuxBootLoader::new(kernel_path);
    }

    #[test]
    fn with_command_line_works() {
        let kernel_path = "/Users/user/Documents/vmlinuz";
        let command_line = "console=hvc0";
        let _ = VZLinuxBootLoader::new(kernel_path).with_command_line(command_line);
    }

    #[test]
    fn with_kernel_path_works() {
        let kernel_path_a = "/Users/user/Documents/A";
        let kernel_path_b = "/Users/user/Documents/B";
        let _ = VZLinuxBootLoader::new(kernel_path_a).with_kernel_path(kernel_path_b);
    }

    #[test]
    fn with_initial_ramdisk_path_works() {
        let kernel_path = "/Users/user/Documents/vmlinuz";
        let initrd_path = "/Users/user/Documents/initrd.img";
        let _ = VZLinuxBootLoader::new(kernel_path).with_initial_ramdisk_path(initrd_path);
    }

    #[test]
    fn into_vzbootloader_works() {
        let command_line = "console=hvc0";
        let kernel_path = "/Users/user/Documents/vmlinuz";
        let initrd_path = "/Users/user/Documents/initrd.img";
        let bootloader = VZLinuxBootLoader::new(kernel_path)
            .with_command_line(command_line)
            .with_initial_ramdisk_path(initrd_path);
        let _: VZBootLoader = bootloader.into();
    }
}
