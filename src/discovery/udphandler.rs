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


pub struct UdpHandler {
  rx: UdpSocket,
  rx_buf: RingBuf,
  localhost: IpAddr
}

impl UdpHandler {
  pub fn new(rx: UdpSocket) -> UdpHandler {
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
