extern crate net2;
extern crate mio;
extern crate bytes;
extern crate std;

use self::bytes::{Buf, MutBuf, RingBuf, SliceBuf};
use self::mio::udp::*;
use self::mio::*;
use self::mio::net::UdpSocket;
use self::mio::deprecated::{EventLoop, Handler};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;
use discovery::MulticastData;
use std::sync::{Arc, Mutex};
use discovery::InstanceDescriptor;

pub struct UdpHandler {
  rx: UdpSocket,
  rx_buf: RingBuf,
  localhost: IpAddr,
  cache: Arc<Mutex<MulticastData>>
}

impl UdpHandler {
  pub fn new(rx: UdpSocket, cache: Arc<Mutex<MulticastData>>) -> UdpHandler {
    let sock = UdpSocket::bind(&"0.0.0.0:0".parse().unwrap()).unwrap();
    UdpHandler {
      rx: rx,
      rx_buf: RingBuf::new(1024),
      localhost: sock.local_addr().unwrap().ip(),
      cache: cache.clone()
    }
  }

  fn handle_read(&mut self, event_loop: &mut EventLoop<UdpHandler>, token: Token, _: Ready) {
    match token {
      LISTENER => {
        debug!("We are receiving a datagram now...");
        let dat = unsafe { self.rx.recv_from(self.rx_buf.mut_bytes()) };
        if dat.is_ok() {
          let (bytes_read, address) = dat.unwrap();

          let str = unsafe { str::from_utf8(&self.rx_buf.mut_bytes()[0..bytes_read]).unwrap().trim() };
          println!("RECEIVED DATA {} {}", str, str.len());

          let mylock = self.cache.clone();
          let mut lock = mylock.lock().unwrap();

          //            println!("RECEIVED DATA {}", lock);
          lock.add_instance(InstanceDescriptor {
            id: str.to_string(),
            identifier: str.to_string(),
            tags: vec![],
            codecs: vec![],
            connection_urls: vec![],
          });
          println!("added a new instance {} - {}", str, lock.instances.len());
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
