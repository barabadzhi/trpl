use std::io::Error;
use std::fmt::Arguments;

// type aliases
type Kilometers = i32;

// fn takes_long_type(f: Box<FnOnce() + Send + 'static>) {}

// fn returns_long_type() -> Box<FnOnce() + Send + 'static> {
//     unimplemented!()
// }

#[allow(dead_code)]
#[allow(unused_variables)]
fn takes_long_type(t: Thunk) {}

#[allow(dead_code)]
fn returns_long_type() -> Thunk {
    unimplemented!()
}

type Thunk = Box<FnOnce() + Send + 'static>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<(), Error>;
}

// the Never type
#[allow(dead_code)]
fn bar() -> ! {
    unimplemented!()
}

// Sized trait
#[allow(dead_code)]
#[allow(unused_variables)]
fn generic<T: ?Sized>(t: &T) {}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    #[allow(unused_variables)]
    // let f: Box<FnOnce() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk = Box::new(|| println!("hi"));

    // dynamically sized types
    #[allow(unused_variables)]
    let s1: &str = "Hello there!";

    #[allow(unused_variables)]
    let s2: &str = "How's it going?";
}
