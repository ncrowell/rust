// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod a {
    struct Foo;
    impl Foo { pub fn new() {} }
    enum Bar {}
    impl Bar { pub fn new() {} }
}

fn main() {
    a::Foo::new();
    //~^ ERROR: static method `new` is inaccessible
    //~^^ NOTE: struct `Foo` is private
    a::Bar::new();
    //~^ ERROR: static method `new` is inaccessible
    //~^^ NOTE: enum `Bar` is private
}
