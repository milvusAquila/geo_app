#[derive(Debug)]
pub struct Couleur(u8, u8, u8);

pub trait Shape {
    fn move_shape(self: &mut Self, x: f64, y: f64);
    fn define_shape(self: &mut Self, x: f64, y: f64);
}

#[derive(Debug, Clone, Copy)]
pub struct Point {pub x: f64, pub y: f64}
impl Point {
    pub fn new(x: f64, y: f64) -> Point {Point{x, y}}
}
impl Shape for Point {
    fn move_shape(self: &mut Point, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    fn define_shape(self: &mut Point, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

/* 


#[derive(Debug, Clone, Copy)]
pub struct Segment {a: Point, b: Point}
impl Segment {
    pub fn new(a: Point, b: Point) -> Segment {Segment{a, b}}
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {a: Point, b: Point}
impl Vector {
    pub fn new(a: Point, b: Point) -> Vector {Vector{a, b}}
}
#[derive(Debug, Clone, Copy)]
pub struct Ray {a: Point, b: Point}
impl Ray {
    pub fn new(a: Point, b: Point) -> Ray {Ray{a, b}}
}




#[derive(Debug, Clone, Copy)]
pub struct Parralelogram {v: Vector, a: Point}
impl Parralelogram {
    pub fn new(v: Vector, a: Point) -> Parralelogram {
        Parralelogram {v: v, a: a}
    }
    pub fn area(&self) -> f64 {
        64.
    }
    pub fn move_Parralelogram(&mut self) {

    }
}


 */