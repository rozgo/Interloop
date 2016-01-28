#![feature(rustc_private)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ws;

use ws::{listen, CloseCode, Message, Handshake, Sender, Handler, Result};
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub extern "C" fn launch(on_bytes : extern "C" fn(i32, *const u8) -> i32) {
    env_logger::init().unwrap();

    let mut incr_id = 0;

    struct Client {
        id: u32,
        ws: Sender,
        interop: extern "C" fn(i32, *const u8) -> i32
    }

    struct Server {
        me: Client,
        clients: Rc<RefCell<Vec<Client>>>
    }

    impl Handler for Server {

        fn on_open(&mut self, _: Handshake) -> Result<()> {
            let ref client = self.me;
            self.clients.borrow_mut().push(Client {id: client.id, ws: client.ws.clone(), interop: client.interop});
            let c = self.clients.borrow().len();
            debug!("on_open num_of_clients:{}", c);
            self.me.ws.send(format!("Num of clients: {}", c))
        }

        fn on_message(&mut self, msg: Message) -> Result<()> {
            debug!("on_message id:{} msg:'{}'", self.me.id, msg);
            let clients = self.clients.borrow();
            for x in 0..clients.len() {
                clients[x].ws.send(msg.clone()).unwrap();
            }
            let xs: [u8; 5] = [1, 2, 3, 4, 5];
            (self.me.interop)(xs.len() as i32, xs.as_ptr());
            Ok(())
        }

        fn on_close(&mut self, _: CloseCode, _: &str) {
            debug!("on_close id:{}", self.me.id);
            let mut clients = self.clients.borrow_mut();
            let index = clients.iter().position(|n| n.id == self.me.id);
            match index {
                Some (index) => {clients.remove(index);},
                None => {},
            }
        }
    }

    let clients = Rc::new(RefCell::new(Vec::new()));

    listen("127.0.0.1:8080", |out| {
        incr_id = incr_id + 1;
        Server { me: Client {id: incr_id, ws: out, interop: on_bytes}, clients: clients.clone() }
    }).unwrap()
}

#[no_mangle]
pub extern "C" fn test_hello_world() {
    println!("Hello world from rust - #2")
}

#[no_mangle]
pub extern "C" fn test_get_num() -> i32 {
    52
}

#[no_mangle]
pub extern "C" fn test_get_array(func : extern "C" fn(i32, *const u8) -> i32) {
    let xs: [u8; 5] = [1, 2, 3, 4, 5];
    func(xs.len() as i32, xs.as_ptr());
}

#[no_mangle]
pub extern "C" fn test_combine(func : extern "C" fn(i32,i32) -> i32) -> i32 {
    func(2,3)
}

#[no_mangle]
pub extern "C" fn test_send_string(c_buf : *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = str::from_utf8(buf).unwrap();
    let str_buf: String = str_slice.to_owned();
    println!("Rust received a string: {}", str_buf)
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
