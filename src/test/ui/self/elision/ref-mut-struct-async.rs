// edition:2018

#![feature(async_await)]

#![feature(arbitrary_self_types)]
#![allow(non_snake_case)]

use std::pin::Pin;

struct Struct { }

impl Struct {
    // Test using `&mut Struct` explicitly:

    async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }
}

fn main() { }
