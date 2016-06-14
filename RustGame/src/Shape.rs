use std::cell::Cell;
use std::ops::*;
use point2d::*;

#[derive(Copy,Clone,Debug)]
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

///A type that hold information about a shape
#[derive(Debug)]
pub struct Shape<T:Clone + Copy + Add + Sub + Mul + Div>{
    ///the vertices that make up the shape's shape
    pub vertices: Vec<Vertex>,
    ///the shape's position on the screen in pixels
    pub pos: Cell<Point2D<T>>,
    ///the direction that the shape faces in radians
    pub facing: Cell<f32>,
}
