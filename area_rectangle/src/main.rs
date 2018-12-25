#[derive(Debug)]
struct Point(u32, u32);

#[derive(Debug)]
struct Rectangle {
	p1: Point,
	p2: Point,
	p3: Point,
	p4: Point,
}

fn get_length(rect: &Rectangle) -> u32 {
	rect.p2.0 - rect.p1.0
}

fn get_width(rect: &Rectangle) -> u32 {
	rect.p3.1 - rect.p3.0
}

impl Rectangle {
	// a method
	fn area(&self) -> u32 {
		(self.p2.0 - self.p1.0) * self.p3.1 - self.p3.0
	}
	// an associative function (no reference to self)
	fn square(origin: &Point, size: u32) -> Rectangle {
		Rectangle {
			p1: Point(origin.0, origin.1),
			p2: Point(origin.0 + size, origin.0),
			p3: Point(origin.0, origin.1 + size),
			p4: Point(origin.0 + size, origin.1 + size),
		}
	}
}



fn main() {
    println!("Hello, world!");
    let rect = Rectangle{
    	p1: Point(0, 0),
    	p2: Point(5, 0),
    	p3: Point(0, 10),
    	p4: Point(5, 10),
    };
    println!("Rectangle: {:#?}", rect);

    let length = get_length(&rect);
    let width = get_width(&rect);
    println!("Area: {}", length*width);

    println!("Area found using a struct method: {}", rect.area());
    let origin = Point(0, 0);
    let square = Rectangle::square(&origin, 5);
    println!("Area of my new square {:#?} is {}", square, square.area());


}
