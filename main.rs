#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]
#![feature(asm)]

extern crate core;
extern crate rlibc;

use rlibc::*;
use core::prelude::*;

pub const SCREEN_SIZE: i32 = 80*25;
pub const SCREEN_WIDTH: i32 = 80;

static mut term_x: i32 = 0;

#[inline(always)]
pub fn outb<T>(port: u16, val: T) {
    unsafe {
        asm!("out $1, $0" :: "{al}"(val), "{dx}"(port) :: "intel");
    }
}

#[no_stack_check]
fn putc(c : char) {
    unsafe {
        match c {
            '\n' => {
                term_x = term_x + SCREEN_WIDTH - (term_x % SCREEN_WIDTH);
            },
            _ => {
                *((0xb8000 + term_x * 2) as *mut u16) = 15 << 12 | c as u16;
                term_x = term_x + 1;
            }
        }
        if term_x > SCREEN_SIZE {
            term_x = term_x + SCREEN_WIDTH - (term_x % SCREEN_WIDTH);

        }

        //set cursor position
        outb(0x3D4, 15u16);
        outb(0x3D5, term_x as u8);
        outb(0x3D4, 14u16);
        outb(0x3D5, (term_x >> 8) as u8);
    }
}

#[no_stack_check]
fn print(line : &str) {
    for c in line.as_bytes().iter() {
        putc(*c as char);
    }
}

#[no_stack_check]
fn clear() {
    unsafe {
        memset(0xb8000 as *mut u8, 0, SCREEN_SIZE as usize);
        //for x in range(0,SCREEN_SIZE) {
        //    *((0xb8000 + x * 2) as *mut u16) = 15 << 12;
        //}
    }
}

#[no_mangle]
#[no_stack_check]
pub fn main() {
    clear();
    print("This is a test\nMORE TEST\nTESTTESTETSETESTTESTTESTETSETESTTESTTESTETSETESTTESTTESTETSETESTTESTTESTETSETESTTESTTESTETSETEST\n");
    print("TEST\n");
    print("MOAR TEST\n");
    print("A LITTLE MOAR");
    loop {}
}