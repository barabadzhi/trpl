// function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    // if let
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for
    let v = vec![1, 2, 3];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let
    #[allow(unused_variables)]
    let x = 5;

    #[allow(unused_variables)]
    let (x, y, z) = (1, 2, 3);

    // function parameters
    let point = (3, 5);
    print_coordinates(&point);

    // refutability
    if let Some(x) = Some(10) {
        println!("{}", x);
    }

    // irrefutable if-let pattern
    //
    // if let x = 5 {
    //     println!("{}", x);
    // }

    // pattern syntax

    //literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),        
    }

    // named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(5) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges with ...
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),        
    }

    // destructuring
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    println!("sum_of_squares: {}", sum_of_squares);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!(
        "(feet, inches); {{x, y}} : ({}, {}); {{{}, {}}}",
        feet,
        inches,
        x,
        y
    );

    // ignoring values
    let x = Some(5);

    match x {
        Some(_) => println!("got a Some and I don't care what's inside"),
        None => (),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // ignoring unused variables
    let _x = 5;
    // let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s);

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    // ignoring remaining parts of a value
    #[allow(dead_code)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // match numbers {
    //     (.., second, ..) => println!("Some numbers: {}", second),
    // }

    // ref and ref mut
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    // match guards
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3...7 } => {
            println!("Found an id in range: {}", id);
        }
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}
