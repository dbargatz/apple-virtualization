mod bindings {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(clippy::all)]

    include!("bindings.rs");
}

use bindings::IVZVirtualMachine;

mod bootloader;
mod config;
pub mod foundation;
mod vm;

pub use bootloader::VZLinuxBootLoader;
pub use config::VZVirtualMachineConfiguration;
pub use vm::VZVirtualMachine;

pub fn supported() -> bool {
    unsafe { bindings::VZVirtualMachine::isSupported() }
}

#[cfg(test)]
mod tests {
    use super::supported;

    #[test]
    fn virtualization_supported() {
        assert!(supported())
    }
}
