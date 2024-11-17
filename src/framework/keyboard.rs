//! Lightweight keypress detection library.
//!
//! Blocks can access keypresses from the Keyboard object.

extern crate filedescriptor;
extern crate log;
extern crate termios;

use std::{
    io::{self, Read},
    os::fd::AsRawFd,
    time,
};
use termios::*;

/// The max number of chars that can be read in a single cycle.
const BUFSIZE: usize = 64;

/// Struct which handles keyboard presses from the terminal.
pub struct Keyboard {
    /// The keys that were pressed since the last refresh() call.
    pub keys: Vec<char>,
}

impl Keyboard {
    /// Construct a new keyboard attached to stdin.
    pub fn new() -> Result<Self, String> {
        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();

        let mut termios = log::unwrap_abort_str!(termios::Termios::from_fd(fd));
        // Turn off canonical mode to read in char by char.
        termios.c_lflag &= !ICANON;
        // Echo characters to the terminal.
        termios.c_lflag |= ECHO;

        log::unwrap_abort_str!(tcsetattr(fd, TCSANOW, &termios));

        Ok(Keyboard {
            keys: Vec::<char>::with_capacity(5),
        })
    }

    /// Reset the keyboard.
    ///
    /// This clears all existing keypresses and then buffers any new ones.
    pub fn reset(&mut self) {
        self.keys.clear();

        // Poll stdin for the number of bytes ready to be read.
        let pollfd = filedescriptor::pollfd {
            fd: io::stdin().as_raw_fd(),
            events: filedescriptor::POLLIN,
            revents: 0,
        };
        let duration = time::Duration::new(0, 0);
        let len = match filedescriptor::poll(&mut [pollfd], Some(duration)) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to poll stdin for keypress: {}", e.to_string());
                return;
            }
        };

        // If there are any keypresses to be read, load them in.
        if len > 0 {
            let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];

            let count = match io::stdin().read(&mut buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to read bytes from stdin: {}", e.to_string());
                    return;
                }
            };

            for i in 0..count {
                self.keys.push(buf[i] as char);
            }
        }
    }
}
