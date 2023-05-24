extern crate rand;
extern crate x11_rs as x11;
extern crate x11_sys as xlib;

use x11::{Display, Event, Window, GC};
use x11::shm::ShmImage;
use std::convert::TryInto;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::ffi::CStr;

fn main() {

    let display = match Display::open() {
        Ok(d) => { d },
        Err(e) => { println!("Error: {:?}", e); return; }
    };

    let window = match Window::create(&display, 1024, 768) {
        Ok(w) =>  { w },
        Err(e) => { println!("Error: {:?}", e); return; }
    };

    let context = match GC::create(&window)  {
        Ok(gc) =>  { gc },
        Err(e) => { println!("Error: {:?}", e); return; }
    };

    window.set_title("xshm example");
    window.show();

    let mut img = match ShmImage::create(&display, display.width(), display.height()) {
        Ok(img) => { img },
        Err(e) => { println!("Error {:?}", e); return; }
    };
    let mut rng = rand::thread_rng();

    loop {
        match window.check_event() {
            Some(Event::Key(code)) => {
                let sym = unsafe { xlib::XKeycodeToKeysym( display.raw, code.try_into().unwrap(), 0 ) };
                let str = unsafe { xlib::XKeysymToString(sym) };
                println!("key pressed: {} is {}", code, unsafe { CStr::from_ptr(str).to_str().unwrap().to_owned() } );
                return;
            }
            Some(Event::Delete) => {
                println!("Window is closed!");
                return;
            }
            _ => {
                let x = rng.gen_range(0..(img.width() - 1));
                let y = rng.gen_range(0..(img.height() - 1));
                let c = rng.gen_range(0..0x00FFFFFF);
                img.put_pixel(x, y, c);
                img.put_image(&window, &context, 0, 0);
                // display.sync();
                // window.show();
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}
