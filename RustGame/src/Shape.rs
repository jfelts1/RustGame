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

pub struct Shape{
    pub vertices: Vec<Vertex>,
    pub pos_x:f32,
    pub pos_y:f32,
}
