#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let l1 = rectangle.bottom_right.x - rectangle.top_left.x;
    let l2 = rectangle.top_left.y - rectangle.bottom_right.y;

    l1 * l2
}

/*
    Add a function square which takes a Point and a f32 as arguments,
  and returns a Rectangle with its top left corner on the point, and a
  width and height corresponding to the f32.
*/

fn square(point: Point, width: f32, height: f32) -> Rectangle {
    let bottom_right = Point {
        x: point.x + width,
        y: point.y - height,
    };

    Rectangle {
        top_left: point,
        bottom_right: bottom_right,
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.3 };

    println!("point coordinates: ({}, {})", point.x, point.y);
    println!(
        "point coordinates: ({}, {})",
        another_point.x, another_point.y
    );

    let rectangle = Rectangle {
        top_left: point,
        bottom_right: another_point,
    };

    let area = rect_area(rectangle);

    println!("area: ({})", area);

    let new_rectangle = square(Point { x: 0.0, y: 1.0 }, 2.0, 1.0);

    let new_area = rect_area(new_rectangle);

    println!("new_area: ({})", new_area);
}
