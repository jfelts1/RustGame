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
pub struct Shape{
    ///the vertices that make up the shape's shape
    pub vertices: Vec<Vertex>,
}

