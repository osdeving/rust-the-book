#![allow(unused, dead_code)]

use std::ops::Add;

fn add<T>(a: T, b: T) -> T where T: Add<Output = T> {
    a+b
}

fn max_of<T: std::cmp::PartialOrd>(v: &[T]) -> &T {
    let mut max = &v[0];

    for i in v {
        if i > max {
            max = i;
        }
    }

    max
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}


enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// some generic parameters are declared with impl
// and some are declared with the method definition
impl<T, U> Point2<T, U> {
    fn mixup<T2, U2>(self, other: Point2<T2, U2>) -> Point2<T, U2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let sum = add(a, b);

    println!("Sum of {a} and {b} is {sum}");

    let v = &vec![1,2,3,100, 20];
    let max = max_of(v);

    println!("max value is: {max}");

    let pt = Point{x:2u32,  y:4u32};
    let pt = Point{x:1.2f32, y:3.3f32};
    //let pt = Point{x:10u8, y:1.5f32}; // T cannot be u8 and f32 at same time

    // Point2 accepts T and U where each one get a type
    let pt = Point2{x:10u8, y:1.5f32}; // T is u8, U is f32
    let pt = Point2{x:1.5f32, y:10u8}; // T is f32, U is u8
    let pt = Point2{x: 10u8, y:10u8}; // T is u8, U is u8
    let pt = Point2{x: 10f32, y:10f32}; // T is f32, U is f32

    let pt = Point{x:232u32,  y:4u32};
    println!("pt.x is {} pt.y is {}", pt.x(), pt.y());

    let pt = Point {x:3.0, y:4.0};
    println!("Distance between {} and {} is {}", pt.x(), pt.y(), pt.distance());

    let pt = Point {x:3, y:4};

    // dont compile. distance is implemented just for Point which type is f32
    //let distance = pt.distance();

    let pt1 = Point2 {x:10, y:20};
    let pt2 = Point2{x:'c', y:"Hello"};
    let pt3 = pt1.mixup(pt2);

    println!("pt3.x is {} pt3.y is {}", pt3.x, pt3.y);
}