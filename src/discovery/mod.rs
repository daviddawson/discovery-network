#![feature(box_syntax)]

extern crate std;
extern crate uuid;
extern crate net2;
extern crate mio;
extern crate bytes;

mod udphandler;

use std::collections::HashMap;
use self::uuid::{Uuid, UuidVersion};
use std::fmt;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::ops::Drop;
use std::str;
use self::mio::udp::*;
use self::mio::*;
use self::mio::deprecated::{EventLoop, Handler};
use self::udphandler::UdpHandler;
use std::net::{SocketAddr};
use self::net2::UdpBuilder;
use self::mio::net::UdpSocket;
use self::bytes::{Buf, MutBuf, RingBuf, SliceBuf};


#[derive(Clone, Debug)]
pub struct InstanceDescriptor {
  pub id: String,
  pub identifier: String,
  pub tags: Vec<String>,
  pub codecs: Vec<String>,
  pub connection_urls: Vec<String>
}

impl InstanceDescriptor {
  pub fn create(identifier: &str) -> InstanceDescriptor {
    let id = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();

    InstanceDescriptor {
      id: id,
      identifier: identifier.to_owned(),
      tags: vec![],
      codecs: vec![],
      connection_urls: vec![],
    }
  }
  pub fn get_identifier(&self) -> String {
    return self.identifier.clone()
  }
}


pub struct MulticastData {
  pub local: Option<InstanceDescriptor>,
  pub instances: Vec<InstanceDescriptor>
}

impl MulticastData {
  pub fn add_instance(&mut self, descriptor: InstanceDescriptor) {
    self.instances.push(descriptor);
  }
}

#[repr(C)]
pub struct MulticastDiscovery {
  pub lock: Arc<Mutex<MulticastData>>
}

pub fn run() -> MulticastDiscovery {
  let data = Arc::new(Mutex::new(MulticastData { local: None, instances: Vec::new() }));

  let multi = MulticastDiscovery::create(data);
  multi
}

impl MulticastDiscovery {
  pub fn create(data: Arc<Mutex<MulticastData>>) -> MulticastDiscovery {
    let threaddata = data.clone();
    let senddata = data.clone();
    let insertdata = data.clone();


    let address: SocketAddr = "0.0.0.0:7776".parse().unwrap();

    let builder = UdpBuilder::new_v4().unwrap();
    builder.reuse_address(true).unwrap();
    let sock = builder.bind(&address).unwrap();

    let rx = UdpSocket::from_socket(sock).unwrap();
    let addr = rx.local_addr().unwrap();

    //sender
    thread::spawn(move || {
      let any = "0.0.0.0:0".parse().unwrap();
      let tx = UdpSocket::bind(&any).unwrap();

      let localhost = tx.local_addr().unwrap();
      loop {
        {
          let mut dat = senddata.lock().unwrap();

          match dat.local {
            Some(ref x) => {
//              let msg = format!("{} {}", x.identifier, address);
              let msg = format!("{}", x.identifier);
              let buf = SliceBuf::wrap(msg.as_bytes());

              println!("Sending....{}", msg);
              let cnt = tx.send_to(buf.bytes(), &"227.1.1.100:7776".parse().unwrap())
                .unwrap();
            }
            None => {

            }
          }
        }
        thread::sleep(Duration::from_millis(500));
      }
    });

    //receiver
    thread::spawn(move || {
      const LISTENER: Token = Token(0);

      let mut event_loop = EventLoop::new().unwrap();


      println!("Joining group 227.1.1.100");
      let any = "0.0.0.0".parse().unwrap();
      rx.join_multicast_v4(&"227.1.1.100".parse().unwrap(), &any).unwrap();

      println!("Registering LISTENER");
      event_loop.register(&rx, LISTENER, Ready::readable(), PollOpt::edge()).unwrap();

      println!("Starting event loop to test with...");
      event_loop.run(&mut UdpHandler::new(rx, insertdata)).unwrap();
    });

    return MulticastDiscovery { lock: data.clone() };
  }

  pub fn on_ready<F>(&self, func: F)
    where F: Fn() + Send + Sync + 'static {
    let function = Arc::new(func);
    thread::spawn(move || {
      thread::sleep(Duration::from_millis(600));
      function();
    });
  }
  pub fn advertise_local_service(&mut self, descriptor: InstanceDescriptor) {
    let data = self.lock.clone();
    println!("Got 1");
    let mut dat = data.lock().unwrap();
    println!("Got 2");
    dat.local = Some(descriptor.clone());
    println!("Advertising a local instance {}", descriptor.get_identifier());
  }

  pub fn shutdown(&mut self) {
    println!("Internal shutdown has been called");
  }

  pub fn get_known_services(&mut self) -> Vec<InstanceDescriptor> {
    let mut dat = self.lock.lock().unwrap();
    dat.instances.clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn calls_on_ready_on_start() {
    let mut disco = run();

    let result = Arc::new(Mutex::new(false));
    let result2 = result.clone();

    disco.on_ready(move || {
      let mut value = result2.lock().unwrap();
      *value = true;
    });

    thread::sleep(Duration::from_millis(1500));

    let mut data = result.lock().unwrap();
    assert!(*data);
  }

  #[test]
  fn on_advertise_will_appear_in_remote() {
    let mut disco = run();

    let me = InstanceDescriptor {
      id: "my-id".to_string(),
      identifier: "my-identinf".to_string(),
      tags: vec![],
      codecs: vec![],
      connection_urls: vec![]
    };

    disco.advertise_local_service(me);

    let mut disco2 = run();

    thread::sleep(Duration::from_millis(1500));

    let instances = disco2.get_known_services();

    assert!(instances.len() > 1);

    println!("Name is {}", instances[0].identifier);

    assert_eq!(instances[0].identifier.trim(), "my-identinf");
  }

  #[test]
  fn get_known_services_returns_all() {
    let mut multi = MulticastData {
      local: Some(InstanceDescriptor {
        id: "my-id".to_string(),
        identifier: "my-identinf".to_string(),
        tags: vec![],
        codecs: vec![],
        connection_urls: vec![]
      }),
      instances: Vec::new()
    };

    multi.add_instance(InstanceDescriptor {
      id: "simple".to_string(),
      identifier: "hello".to_string(),
      tags: vec![],
      codecs: vec![],
      connection_urls: vec![]
    });

    assert!(multi.instances.len() == 1);
  }
  //
  //  #[test]
  //  fn expires_cache_5s() {
  //    assert!(1i32 == 3i32);
  //  }
}
