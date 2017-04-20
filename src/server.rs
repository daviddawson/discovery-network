extern crate mio;

use self::mio::*;

pub struct MulticastServer {
    pub identifier: *mut c_char,
    pub tags: Vec<&'static str>,
    pub codecs: Vec<&'static str>,
    pub connection_urls: Vec<&'static str>
}

impl MulticastServer {
    pub fn create(identifier: &str) -> ServiceDescriptor {
        ServiceDescriptor {
            identifier: (CString::new("AWESOME").unwrap().into_raw()),
            tags: vec!["h", "b"],
            codecs: vec!["h", "b"],
            connection_urls: vec!["h", "b"],
        }
    }
    pub fn get_identifier(&self) -> String {
        unsafe {
            return CString::from_raw(self.identifier).into_string().unwrap();
        }
    }
}