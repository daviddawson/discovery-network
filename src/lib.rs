extern crate libc;
#[macro_use]
extern crate log;

use std::thread;
use std::string;
mod discovery;
use discovery::MulticastDiscovery;
use std::ffi::CString;
use std::ffi::CStr;
use self::libc::c_char;
use self::libc::size_t;
use std::iter::Iterator;

#[repr(C)]
pub struct ServiceDescriptor {
  pub identifier: *mut c_char,
  pub tags: *const *const c_char,
  pub codecs: *const *const c_char,
  pub connection_urls: *const *const c_char,
  pub tags_length: size_t,
  pub codecs_length: size_t,
  pub connection_urls_length: size_t
}

//#[repr(C)]
//pub struct OnReady {
//    callback: extern fn() -> bool
//}
//
//#[no_mangle]
//pub extern fn get_known_services(target: *mut MulticastDiscovery) {
//    unsafe {
//        (*target).get_known_services();
//    }
//}
//
#[no_mangle]
pub extern fn advertise_local_service(target: *mut MulticastDiscovery, descriptor:ServiceDescriptor) {
  println!("Advertising? {}", descriptor.tags_length);

  let desc = unsafe { discovery::ServiceDescriptor {
    identifier: &CStr::from_ptr(descriptor.identifier).to_str().unwrap(),
    tags: array_to_vec(descriptor.tags, descriptor.tags_length),
    codecs: array_to_vec(descriptor.codecs, descriptor.codecs_length),
    connection_urls: array_to_vec(descriptor.connection_urls, descriptor.connection_urls_length),
  } };
}

fn array_to_vec<'a>(vals: *const *const c_char, len: size_t) -> Vec<&'a str>
{

  let arr = unsafe {
    std::slice::from_raw_parts(vals, len as usize)
      .iter().map(|tag| {
      let val = CStr::from_ptr((*tag)).to_str();
      return val.unwrap();
    })
  };

  let mut vector = Vec::new();
  for i in arr {
    println!("{}", i);
    vector.push(i);
  }
  vector
}
//
//#[no_mangle]
//pub extern fn advertise_local_service_old(target: *mut MulticastDiscovery,
//                                      tags: *const *const c_char, length: size_t) {
//
//  println!("Advertising? {}", length);
//
//  let tagslice = unsafe {
//    std::slice::from_raw_parts(tags as *const *const c_char, length as usize)
//      .iter().map( | tag | CStr::from_ptr((*tag)).to_str().unwrap())
//  };
//
//  for x in tagslice {
//    println!("{}", x);
//  }
//
////  println!("Got 0, {:?}", tags);
//
//
////  let v = unsafe { Vec::<T>::from_raw_parts(ptr, length, capacity) };
//
////  mytarget.advertise_local_service()
//
////    unsafe {
////  unsafe { (*target).advertise_local_service(&discovery::ServiceDescriptor {
////    identifier: (CString::new("AWESOME1234").unwrap().into_raw()),
////    numberOfTags: 2,
////    tags: tags,
////    codecs: vec![],
////    connection_urls: vec![],
////  }); }
//}
//
//#[no_mangle]
//pub extern fn on_ready(target: *mut MulticastDiscovery, call: OnReady) {
//    println!("I'm called from C");
//    unsafe {
//        (*target).on_ready(|| {
//            println!("EXTERNAL ON READY, calling exec.....");
//            (call.callback)();
//        })
//    }
//}
//
#[no_mangle]
pub extern fn create(name: *const u8) -> *mut MulticastDiscovery {
    Box::into_raw(Box::new(discovery::run()))
}
//
//#[no_mangle]
//pub extern fn shutdown(target: *mut MulticastDiscovery) {
//    unsafe {
//        (*target).shutdown();
//        drop(target);
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

    }
}
