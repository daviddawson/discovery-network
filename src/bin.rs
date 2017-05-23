#[macro_use]
extern crate log;
extern crate env_logger;

use log::LogLevel;
//extern crate log4rs;
use std::time::Duration;
use std::thread;

mod discovery;

fn main() {
    env_logger::init().unwrap();
    info!("booting up");
    info!("Commencing yak shaving for {}", "simples");

    let me = discovery::InstanceDescriptor::create("AWESOME");

    let mut disco = discovery::run();
//    disco.on_ready(&|| println!("DISCO IS READY!"));
    thread::sleep(Duration::from_millis(2000));
    disco.advertise_local_service(me);

    disco.get_known_services();

    thread::sleep(Duration::from_millis(15000));
    println!("RAGGLE");

}
