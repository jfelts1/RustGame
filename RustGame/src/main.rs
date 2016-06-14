#[macro_use]
extern crate glium;
use std::cell::Cell;
mod Shape;
mod keyboard;
mod point2d;
use std::f32;
use point2d::*;

//these are temporary until I have file based config coded
const WINDOW_WIDTH:u32 = 1280;
const WINDOW_HEIGHT:u32 = 720;
const ACCELL:f32 = 10.0;

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
    let win_size:Point2D<f32>;
    match window_size{
        Some((x,y))=>{
            win_size = Point2D::new(x as f32,y as f32);
        },
        None => return,
    }

    let shape = Shape{
        vertices: vec![Vertex::new([-0.025,-0.025]),Vertex::new([0.0,0.025]),Vertex::new([0.025,-0.025])],
        pos:Cell::new(Point2D::new(0.0,0.0)),
        facing:Cell::new(0.0),
    };
    let vertex_buffer = glium::VertexBuffer::new(&display,&shape.vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    uniform mat4 rot_matrix;
    uniform mat4 trans_matrix;

    void main() {
        mat4 tmp = trans_matrix*rot_matrix;
        gl_Position = tmp*vec4(position, 0.0, 1.0);
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
    let mut mouse_pos:Point2D<i32> = Point2D::new(0,0);
    let mut tmp:Point2D<f32> = Point2D::new(0.0,0.0);
    loop{
        tmp.y = shape.pos.get().y+win_size.y/2.0-(mouse_pos.y as f32);
        tmp.x = shape.pos.get().x+win_size.x/2.0-(mouse_pos.x as f32);
        t = tmp.y.atan2(tmp.x);
        t = t-f32::consts::PI/ 2.0;
        //flip the rotation direction to track the mouse correctly
        t=-t;

        shape.facing.set(t);

        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);

        let rot_matrix=[
        [t.cos(), t.sin(), 0.0, 0.0],
        [-t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
        ];

        let shape_x = (shape.pos.get().x/win_size.x);
        let shape_y = (shape.pos.get().y/win_size.y);
        let trans_matrix = [
        [1.0,0.0,0.0,0.0],
        [0.0,1.0,0.0,0.0],
        [0.0,0.0,1.0,0.0],
        [shape_x,shape_y,0.0,1.0]];

        println!("shape pos {:?} tmp {:?} shape_x {} shape_y {} mouse pos {:?} win_size {:?} shape facing {} ",shape.pos,tmp,shape_x,shape_y,mouse_pos,win_size,shape.facing.get());

        target.draw(&vertex_buffer,&indices,&program,&uniform!{rot_matrix:rot_matrix,trans_matrix:trans_matrix},&Default::default()).unwrap();
        target.finish().unwrap();
        use glium::glutin::*;
        use keyboard::UserInput::*;
        for ev in display.poll_events(){
            match ev{
                Event::Closed => return,
                Event::MouseMoved(x,y)=> {
                    mouse_pos.x = x;
                    mouse_pos.y = y;
                },
                Event::KeyboardInput(state,v,key_code)=>{
                    let input = keyboard::handle_keyboard(state,v,key_code);
                    let mut pos = shape.pos.get();
                    match input{
                        MoveForward=>{
                            let rot = shape.facing.get() + f32::consts::PI/2.0;
                            let (y,x) = rot.sin_cos();
                            pos.y = pos.y-ACCELL*y;
                            pos.x = pos.x-ACCELL*x;
                            shape.pos.set(pos);
                        },
                        MoveBackward=>{
                            let rot = shape.facing.get()+f32::consts::PI/2.0;
                            let (y,x) = rot.sin_cos();
                            pos.y = pos.y+ACCELL*y;
                            pos.x = pos.x+ACCELL*x;
                            shape.pos.set(pos);
                        },
                        MoveLeft=>{
                            let (y,x) = shape.facing.get().sin_cos();
                            pos.y = pos.y-ACCELL*y;
                            pos.x = pos.x-ACCELL*x;
                            shape.pos.set(pos);
                        },
                        MoveRight=>{
                            let (y,x) = shape.facing.get().sin_cos();
                            pos.y = pos.y+ACCELL*y;
                            pos.x = pos.x+ACCELL*x;
                            shape.pos.set(pos);
                        },
                        _=>{}
                    }
                },
                _=> ()
            }
        }
    }
}