#[macro_use]
extern crate glium;
use std::cell::Cell;
mod Shape;
mod keyboard;
use std::f32;

//these are temporary until I have file based config coded
const WINDOW_WIDTH:u32 = 1280;
const WINDOW_HEIGHT:u32 = 720;

fn main() {
    use Shape::*;
    use glium::{Api,Version,DisplayBuild,Surface};
    let display = glium::glutin::WindowBuilder::new()
    .with_vsync()
    .with_dimensions(WINDOW_WIDTH,WINDOW_HEIGHT)
    .build_glium()
    .unwrap();
    let window = display.get_window();
    let window_size: Option<(u32,u32)>;

    let version = *display.get_opengl_version();
    let api = match version {
        Version(Api::Gl, _, _) => "OpenGL",
        Version(Api::GlEs, _, _) => "OpenGL ES"
    };
    println!("{} context renderer: {}", api, display.get_opengl_renderer_string());
    println!("{} context vendor: {}", api, display.get_opengl_vendor_string());
    println!("{} context version: {}", api, display.get_opengl_version_string());

    match window{
        Some(win) => window_size = win.get_inner_size_pixels(),
        None => return,
    }

    //get the window size in an easily accessible form
    let win_size:Point2DImmutable<f32>;
    match window_size{
        Some((x,y))=>{
            win_size = Point2DImmutable::new(x as f32,y as f32);
        },
        None => return,
    }

    let shape = Shape{
        vertices: vec![Vertex::new([-0.25,-0.25]),Vertex::new([0.0,0.25]),Vertex::new([0.25,-0.25])],
        pos:Point2D::new(win_size.x/2.0,win_size.y/2.0),
        facing:Cell::new(0.0),
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
        let tmp_y:f32 = shape.pos.y.get()-(mouse_y as f32);
        let tmp_x:f32 = shape.pos.x.get()-(mouse_x as f32);
        t = tmp_y.atan2(tmp_x);
        t = t-f32::consts::PI/ 2.0;

        shape.facing.set(t);

        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);
        //
        let shape_x = shape.pos.x.get()/win_size.x - 0.5;
        let shape_y = shape.pos.y.get()/win_size.y - 0.5;
        let uniforms = uniform!{
        matrix:[
        [t.cos(), -t.sin(), 0.0, 0.0],
        [t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [shape_x, shape_y, 0.0, 1.0f32],
        ]
        };

        //println!("shape pos ({},{}) mouse pos ({},{}) shape facing {}",shape.pos.x.get(),shape.pos.y.get(),mouse_x,mouse_y,shape.facing.get());

        target.draw(&vertex_buffer,&indices,&program,&uniforms,&Default::default()).unwrap();
        target.finish().unwrap();
        use glium::glutin::*;
        for ev in display.poll_events(){
            match ev{
                Event::Closed => return,
                Event::MouseMoved(x,y)=> {
                    mouse_x=x;
                    mouse_y=y;
                },
                Event::KeyboardInput(state,v,key_code)=>{
                    keyboard::handle_keyboard(state,v,key_code);
                },
                _=> ()
            }
        }
    }
}