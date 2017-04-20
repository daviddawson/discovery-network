extern crate libc;
#[macro_use]
extern crate log;

use std::thread;
use std::string;
mod discovery;
use discovery::MulticastDiscovery;

#[repr(C)]
pub struct OnReady {
    callback: extern fn() -> bool
}

#[no_mangle]
pub extern fn get_known_services(target: *mut MulticastDiscovery) {
    unsafe {
        (*target).get_known_services();
    }
}

#[no_mangle]
pub extern fn advertise_local_service(target: *mut MulticastDiscovery, descriptor: *mut discovery::ServiceDescriptor) {
    unsafe {
        (*target).advertise_local_service(&*descriptor);
    }
}

#[no_mangle]
pub extern fn on_ready(target: *mut MulticastDiscovery, call: OnReady) {
    println!("I'm called from C");
    unsafe {
        (*target).on_ready(|| {
            println!("EXTERNAL ON READY, calling exec.....");
            (call.callback)();
        })
    }
}

#[no_mangle]
pub extern fn create(name: *const u8) -> *mut MulticastDiscovery {
    let mut mydisco = Box::new(discovery::run());
    return &mut *mydisco;
}

#[no_mangle]
pub extern fn shutdown(target: *mut MulticastDiscovery) {
    unsafe {
        (*target).shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

    }
}