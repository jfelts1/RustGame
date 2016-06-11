#[macro_use]
extern crate glium;
mod Shape;
use std::f32;

fn main() {
    use Shape::*;
    use glium::{DisplayBuild,Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let window = display.get_window();
    let window_size: Option<(u32,u32)>;
    match window{
        Some(win) => window_size = win.get_inner_size_pixels(),
        None => return,
    }
    let shape_x:f32;
    let shape_y:f32;
    match window_size{
        Some((x,y))=>{
            shape_x = (x as f32)/2.0;
            shape_y = (y as f32)/2.0;
        },
        None => return,
    }
    let shape = Shape{
        vertices: vec![Vertex::new([-0.5,-0.5]),Vertex::new([0.0,0.5]),Vertex::new([0.5,-0.5])],
        pos_x:shape_x,
        pos_y:shape_y,
    };
    let vertex_buffer = glium::VertexBuffer::new(&display,&shape.vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    uniform mat4 matrix;

    void main() {
        gl_Position = matrix*vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display,vertex_shader_src,fragment_shader_src,None).unwrap();
    let mut t: f32;
    let (mut mouse_x,mut mouse_y) = (0,0);


    loop{
        let tmp_y:f32 = shape.pos_y-(mouse_y as f32);
        let tmp_x:f32 = shape.pos_x-(mouse_x as f32);
        t = tmp_y.atan2(tmp_x);
        t = t-f32::consts::PI/ 2.0;
        /*t += 0.0002;
        if t > 2.0*3.14{
            t = 0.0;
        }*/
        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);

        let uniforms = uniform!{
        matrix:[
        [t.cos(), -t.sin(), 0.0, 0.0],
        [t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0 , 0.0, 0.0, 1.0f32],
        ]
        };

        println!("shape pos ({},{}) mouse pos ({},{}) t = {}",shape.pos_x,shape.pos_y,mouse_x,mouse_y,t);

        target.draw(&vertex_buffer,&indices,&program,&uniforms,&Default::default()).unwrap();
        target.finish().unwrap();
        for ev in display.poll_events(){
            match ev{
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(x,y)=> {
                    mouse_x=x;
                    mouse_y=y;
                },
                _=> ()
            }
        }
    }
}