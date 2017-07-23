use std::ops::Add;
use std::fmt;

// operator overloading
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 + (other.0 * 1000)))
    }
}

// associated types vs generics
trait GGraph<Node, Edge> {}

trait AGraph {
    type Node;
    type Edge;
}

// fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
//     unimplemented!()
// }

// fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
//     unimplemented!()
// }

// fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
//     unimplemented!()
// }

// trait objects with associated types
#[allow(dead_code)]
#[allow(unused_variables)]
fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
    unimplemented!()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn traverse(graph: &AGraph<Node = usize, Edge = (usize, usize)>) {}

// syntax for disambiguation
trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("Baz's impl of Foo");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Baz's impl of Bar");
    }
}

impl Baz {
    fn f(&self) {
        println!("Baz's impl");
    }
}

// supertraits
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

// newtype pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let b = Baz;

    b.f();
    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("w = {}", w);
}
