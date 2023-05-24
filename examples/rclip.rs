extern crate x11_rs as x11;
extern crate x11_sys as xlib;
extern crate clap;
extern crate core;

use clap::{arg, Command};
use core::fmt;
use std::thread;
use std::time::Duration;
use std::ffi::CStr;
use std::convert::TryInto;
use x11::{Display, Event, Window, GC};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DebugLevelType {
    /// No debug output
    None,
    /// Only output informational debug stuff
    Information,
    /// More information and debug data
    Verbose,
    /// Very detailed and very verbose amount of debug information
    Detailed,
}

impl std::convert::From<u8> for DebugLevelType {
    /// Convert a u8 (typically from a CLI argument) into the appropriate
    /// debug enum. Use "x as u8".
    fn from(orig: u8) -> Self {
        match orig {
            0 => DebugLevelType::None,
            1 => DebugLevelType::Information,
            2 => DebugLevelType::Verbose,
            3 => DebugLevelType::Detailed,
            _ => DebugLevelType::None,
        }
    }
}

impl fmt::Display for DebugLevelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugLevelType::None => write!(f, "None"),
            DebugLevelType::Information => write!(f, "Information"),
            DebugLevelType::Verbose => write!(f, "Verbose"),
            DebugLevelType::Detailed => write!(f, "Detailed"),
        }
    }
}

pub struct Args {
    /// Debug
    debug: u8,
}

impl Default for Args {
    /// Default instance of Args as recommended by Clippy
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    /// Create a new instance of the arguments to the program
    pub fn new() -> Self {
        let matches = Command::new("bft")
            .version("1.0")
            .author("J Axtell <jonaxtell@codethink.co.uk>")
            .about("Messes about with the X11 clipboard")
            .arg(
                arg!(-d --debug "Debug. Multiple occurrences will increase verbosity")
                    .required(false)
                    .action(clap::ArgAction::Count),
            )
            .get_matches();

        // Check debug arg first since it's used for outputting other arg statuses
        let debug = matches.get_count("debug");
        if <u8 as Into<DebugLevelType>>::into(debug) > DebugLevelType::Information {
            println!("Debug is {:?}", debug);
        }

        Args {
            debug,
        }
    }

    pub fn debug(&self) -> DebugLevelType {
        self.debug.into()
    }

    pub fn debug_on(&self) -> bool {
        self.debug != 0
    }
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {

    let display = match Display::open() {
        Ok(d) => { d },
        Err(e) => { if args.debug_on() { println!("Error: {:?}", e); } return Err(e.into()); }
    };

    let window = match Window::create_simple(&display, 1, 1) {
        Ok(w) =>  { w },
        Err(e) => { if args.debug_on() { println!("Error: {:?}", e); }return Err(e.into()); }
    };

    let _graphic_context = match GC::create(&window)  {
        Ok(gc) =>  { gc },
        Err(e) => { if args.debug_on() { println!("Error: {:?}", e); }return Err(e.into()); }
    };

    window.set_title("xclip example");
    window.show();


    loop {
        match window.check_event() {
            Some(Event::Key(code)) => {
                let sym = unsafe { xlib::XKeycodeToKeysym( display.raw, code.try_into().unwrap(), 0 ) };
                let str = unsafe { xlib::XKeysymToString(sym) };
                println!("key pressed: {} is {}", code, unsafe { CStr::from_ptr(str).to_str().unwrap().to_owned() } );
                if code == 9 { return Ok(()) }  // Escape
            }
            Some(Event::Delete) => {
                println!("Window is closed!");
                return Ok(());
            }
            _ => {
                display.sync();
                window.show();
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

}

fn main() {
    let args = &Args::new();
    if args.debug_on() { println!("Debugging level is {} [{}]", args.debug(), args.debug() as u8); }
    match run(args) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {}", e);
            std::process::exit(1)
        }
    }
    std::process::exit(0)
}