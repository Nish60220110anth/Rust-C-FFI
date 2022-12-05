#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::c_char;

#[link(name = "MyMathLib", kind = "static")]
extern "C" {
    // fn AddFloat(a1: f32, a2: f32) -> f32;
    fn FFIPrintTest() -> ();
    fn FFIScanTest() -> i32;
    fn FFPrintTestWithArg(a: *const c_char) -> ();
}

#[link(name = "MyMathLib2", kind= "static")]
extern "C" {
    fn TestFunc() -> ();
}

fn main() {
    println!("Hello from Rust\nThis is the sample code to test FFI in rust");
    // let res = unsafe { AddFloat(4.53, 7.65) };
    unsafe {
        FFIPrintTest();
    }
    let gotInt = unsafe {
        FFIScanTest()
    };
    println!("Got Int {}",gotInt);
    unsafe {
        TestFunc();
    }
}
