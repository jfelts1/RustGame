use std::cell::Cell;
use std::ops::*;

#[derive(Copy,Clone)]
pub struct Vertex{
    position: [f32;2],
}

implement_vertex!(Vertex, position);

impl Vertex{
    pub fn new(position:[f32;2])->Vertex{
        Vertex{
            position : position,
        }
    }
}

///A mutable type to hold Point data
pub struct Point2D<T:Clone + Copy + Add + Sub + Mul + Div + Rem>{
    pub x : Cell<T>,
    pub y : Cell<T>,
}

impl<T:Clone + Copy + Add + Sub + Mul + Div + Rem> Point2D<T>{
    pub fn new(x:T,y:T)->Point2D<T>{
        Point2D{
            x : Cell::new(x),
            y : Cell::new(y),
        }
    }
}

///An immutable type to hold Point data
pub struct Point2DImmutable<T:Add + Sub + Mul + Div + Rem>{
    pub x: T,
    pub y: T,
}

impl<T:Add + Sub + Mul + Div + Rem> Point2DImmutable<T>{
    pub fn new(x:T,y:T)->Point2DImmutable<T>{
        Point2DImmutable{
            x: x,
            y: y,
        }
    }
}

///A type that hold information about a shape
pub struct Shape<T:Clone + Copy + Add + Sub + Mul + Div + Rem>{
    ///the vertices that make up the shape's shape
    pub vertices: Vec<Vertex>,
    ///the shape's position on the screen in pixels
    pub pos: Point2D<T>,
    ///the direction that the shape faces in radians
    pub facing: Cell<f32>,
}
