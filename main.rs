#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]

extern crate core;

use core::prelude::*;

#[allow(dead_code)]
enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

struct Terminal {
    x : i32,
    y : i32,
    bgColor : u16
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            x: 0,
            y: 0,
            bgColor: Color::White as u16
        }
    }
    pub fn clear(&mut self, color : Color) {
        self.bgColor = color as u16;
        self.x = 0;
        self.y = 0;
        for x in range(0, 80*25) {
            Terminal::put_char(self.bgColor, ' ', x);
        }
    }
    pub fn print(&mut self, line : &str) {
        for c in line.chars() {
            match c {
                '\n' => { self.y = self.y + 1; self.x = 0; },
                _ => {
                    Terminal::put_char(self.bgColor, c, self.x + self.y * 80);
                    self.x = self.x + 1;
                    if (self.x > 80) {
                        self.y = self.y + 1;
                        self.x = 0;
                    }
                }
            }
        }
    }
    fn put_char(background:u16, value : char, x : i32) {
        unsafe {
            *((0xb8000 + x * 2) as *mut u16) = (background << 12) | (value as u16);
        }
    }
}

#[no_mangle]
#[no_stack_check]
pub fn main() {
    let mut t = Terminal::new();
    t.clear(Color::LightBlue);
    t.print("Hello, world!\nThis is a test of\nmultiline shenanigans.");
}