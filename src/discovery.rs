extern crate net2;
extern crate mio;
extern crate bytes;
extern crate std;
extern crate uuid;

use self::uuid::{Uuid, UuidVersion};
use std::fmt;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::ops::Drop;
use std::str;
use self::bytes::{Buf, MutBuf, RingBuf, SliceBuf};
use self::mio::udp::*;
use self::mio::*;
use self::mio::net::UdpSocket;
use self::mio::deprecated::{EventLoop, Handler};
use self::net2::UdpBuilder;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct UdpHandler {
  rx: UdpSocket,
  rx_buf: RingBuf,
  localhost: IpAddr
}

impl UdpHandler {
  fn new(rx: UdpSocket) -> UdpHandler {
    let sock = UdpSocket::bind(&"0.0.0.0:0".parse().unwrap()).unwrap();
    UdpHandler {
      rx: rx,
      rx_buf: RingBuf::new(1024),
      localhost: sock.local_addr().unwrap().ip()
    }
  }

  fn handle_read(&mut self, event_loop: &mut EventLoop<UdpHandler>, token: Token, _: Ready) {
    match token {
      LISTENER => {
        debug!("We are receiving a datagram now...");
        unsafe {
          let dat = self.rx.recv_from(self.rx_buf.mut_bytes());
          if (dat.is_ok()) {
            let val = dat.unwrap();
            println!("RECEIVED DATA {}", str::from_utf8(self.rx_buf.mut_bytes()).unwrap())
          }
        }
      }
      _ => ()
    }
  }
}

impl Handler for UdpHandler {
  type Timeout = usize;
  type Message = ();

  fn ready(&mut self, event_loop: &mut EventLoop<UdpHandler>, token: Token, events: Ready) {
    if events.is_readable() {
      println!("readables");
      self.handle_read(event_loop, token, events);
    }
  }
}

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
  pub name: &'static str,
}

#[repr(C)]
pub struct MulticastDiscovery {
  pub name: &'static str,
  pub lock: Arc<Mutex<MulticastData>>
}

pub fn run() -> MulticastDiscovery {
  let data = Arc::new(Mutex::new(MulticastData { name: "awesome" }));

  let multi = MulticastDiscovery::create(data);
  multi
}

impl MulticastDiscovery {
  pub fn create(data: Arc<Mutex<MulticastData>>) -> MulticastDiscovery {
    let threaddata = data.clone();
    let senddata = data.clone();

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
          let dat = senddata.lock().unwrap();
          let msg = format!("{} {}", dat.name, address);
          let mut buf = SliceBuf::wrap(msg.as_bytes());

          println!("Sending....{}", addr);
          let cnt = tx.send_to(buf.bytes(), &"227.1.1.100:7776".parse().unwrap())
            .unwrap();
        }
        //                buf.advance(cnt);
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
      event_loop.run(&mut UdpHandler::new(rx)).unwrap();
    });

    return MulticastDiscovery { lock: data, name: "Happy" };
  }

  pub fn on_ready<F>(&self, func: F)
    where F: Fn() + Send + Sync + 'static {
    let function = Arc::new(func);
    thread::spawn(move || {
      thread::sleep(Duration::from_millis(600));
      function();
    });
  }
  pub fn advertise_local_service(&mut self, descriptor: &InstanceDescriptor) {
    //        println!("Got 0, {:?}", descriptor.tags);
    let data = self.lock.clone();
    println!("Got 1");
    let mut dat = data.lock().unwrap();
    println!("Got 2");
    dat.name = "HELLO WORLD";
    println!("Advertising a local instance {}", descriptor.get_identifier());
  }

  pub fn shutdown(&mut self) {
    println!("Internal shutdown has been called");
  }

  pub fn get_known_services(&mut self) /* TODO -> [ServiceDescriptor] */ {
    //        []
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
    disco.advertise_local_service({});
    let mut disco2 = run();
    let mut disco3 = run();

    thread::sleep(Duration::from_millis(1500));


    assert!(*data);
  }
  //
  //  #[test]
  //  fn get_known_services_returns_all() {
  //    assert!(1i32 == 3i32);
  //  }
  //
  //  #[test]
  //  fn expires_cache_5s() {
  //    assert!(1i32 == 3i32);
  //  }
}
