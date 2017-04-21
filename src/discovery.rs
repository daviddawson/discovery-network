extern crate libc;
extern crate net2;
extern crate mio;
//#[macro_use]
//extern crate log;

use self::libc::c_char;
use std::ffi::CString;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::ops::Drop;

use self::mio::*;
use self::mio::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub trait Callme {
    fn callback(&self);
}

#[repr(C)]
pub struct ServiceDescriptor {
    pub identifier: *mut c_char,
    pub tags: Vec<&'static str>,
    pub codecs: Vec<&'static str>,
    pub connection_urls: Vec<&'static str>
}

impl ServiceDescriptor {
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


pub struct MulticastData {
    pub name: &'static str,
}

#[repr(C)]
pub struct MulticastDiscovery {
    pub name: &'static str,
    pub lock: Arc<Mutex<MulticastData>>
}

impl Callme for MulticastDiscovery {
    fn callback(&self) {
        println!("callback on trait");
    }
}

pub fn run() -> MulticastDiscovery{

    let data = Arc::new(Mutex::new(MulticastData { name: "awesome"}));

    let multi = MulticastDiscovery::create(data);
    multi
}

impl MulticastDiscovery {
    pub fn create(data: Arc<Mutex<MulticastData>>) -> MulticastDiscovery {

        let threaddata = data.clone();
        /**

        use a mutex in the discovery
        when updatuing the discovery, do so via a message onto a channel to a thread in the lib.
        whenever reading from the discovery, use mutex.
        all data should be in a substruct within the mutex
        **/
        println!("SIGGLE");
//        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            {
                let data = threaddata.lock().unwrap();
                println!("Data is {}", data.name);
            }
            thread::sleep(Duration::from_millis(2000));
            {
                let data = threaddata.lock().unwrap();
                println!("Data is {}", data.name);
            }
            println!("HAPPY");
            let mut socket = "239.255.0.0:7777".parse().unwrap();
            let udp = Box::new(UdpSocket::bind(&socket).unwrap());
            let boxed = &udp;
            println!("WOOT!");
//            tx.send(some_expensive_computation(init_val));
        });
//        rx

//        let udp = UdpBuilder::new_v4().unwrap();
//        let socket = udp.reuse_address(true).unwrap().bind(("0.0.0.0", 4445));
////          tcp.only_v6(false).unwrap();
//
//        socket.join_multicast_v4("224.1.7.8".parse(), "0.0.0.0".parse());

//        let sock = UdpSocket::bind("0.0.0.0:2345").unwrap();
//        let local_addr = Ipv4Addr::new(0,0,0,0);
//        let multicast_addr = Ipv4Addr::new(239, 255, 0, 1);
//
//        sock.join_multicast_v4(&multicast_addr, &local_addr).unwrap();

//        let mut stream = tcp.connect("127.0.0.1:80").unwrap();

        return MulticastDiscovery { lock:data, name:"Happy" };
    }

    pub fn on_ready<F>(& mut self, arg: F)
        where F: Fn() {
        info!("CALLED ATTACHED METHOD");
//        println!("CALLED A ATTACHED METHOD!");
        arg();
    }
    pub fn advertise_local_service(& mut self, descriptor: &ServiceDescriptor) {
        println!("Got 0, {}", self.name);
        let data = self.lock.clone();
        println!("Got 1");
        let mut dat = data.lock().unwrap();
        println!("Got 2");
        dat.name = "HELLO WORLD";
        println!("Advertising a local instance {}", descriptor.get_identifier());
    }

    pub fn shutdown(& mut self) {
        println!("Internal shutdown has been called");

    }

    pub fn get_known_services(& mut self) /* TODO -> [ServiceDescriptor] */ {

//        []
    }
}