#[macro_use]
extern crate glium;

mod shape;
mod keyboard;
mod point2d;
mod utils;
mod game_object_types;

use std::f32;
use point2d::*;
use utils::physics;
use utils::game;
use game_object_types::game_object::{GameObject,Updatable};

//these are temporary until I have file based config coded
const WINDOW_WIDTH:u32 = 1280;
const WINDOW_HEIGHT:u32 = 720;

fn main() {
    use shape::*;
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

    let ship = GameObject::new(Shape{vertices:vec![Vertex::new([-0.025,-0.025]),Vertex::new([0.0,0.025]),Vertex::new([0.025,-0.025])]},
                               Point2D::new(0.0,0.0),
                               0.0,
                               Point2D::new(0.0,0.0));

    let vertex_buffer = glium::VertexBuffer::new(&display,&ship.shape.vertices).unwrap();
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
    let mut mouse_shape_pos_diff:Point2D<f32> = Point2D::new(0.0,0.0);
    //this is the shapes position based on the actual screen coordinates in pixels with (0,0) being in the top left of the viewport
    let mut shape_pos:Point2D<f32> = Point2D::new(0.0,0.0);

    //main game loop
    loop{
        if !game::handle_user_input(&display,&mut mouse_pos,&ship) {
            //exit program
            return;
        }

        mouse_shape_pos_diff.y = ship.pos.get().y+win_size.y/2.0;
        shape_pos.y = mouse_shape_pos_diff.y;
        mouse_shape_pos_diff.y = mouse_shape_pos_diff.y-(mouse_pos.y as f32);

        mouse_shape_pos_diff.x = ship.pos.get().x+win_size.x/2.0;
        shape_pos.x = mouse_shape_pos_diff.x;
        mouse_shape_pos_diff.x = mouse_shape_pos_diff.x-(mouse_pos.x as f32);

        t = mouse_shape_pos_diff.y.atan2(mouse_shape_pos_diff.x);
        t = t-f32::consts::PI/ 2.0;
        //flip the rotation direction to track the mouse correctly
        t=-t;

        ship.facing.set(t);
        ship.update();

        //render
        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);

        let rot_matrix=[
        [t.cos(), t.sin(), 0.0, 0.0],
        [-t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
        ];

        //multiply by two so that the position units match the screen size
        let shape_x = ship.pos.get().x*2.0/win_size.x;
        //the sign on the y coordinate is flipped so that the shapes position is in the same coordinate as the mouse position
        let shape_y = -(ship.pos.get().y*2.0/win_size.y);
        let trans_matrix = [
        [1.0,0.0,0.0,0.0],
        [0.0,1.0,0.0,0.0],
        [0.0,0.0,1.0,0.0],
        [shape_x,shape_y,0.0,1.0]];

        //println!("shape pos {:?} tmp {:?} shape_x {} shape_y {} mouse pos {:?} win_size {:?} shape facing {} ",shape_pos,mouse_shape_pos_diff,shape_x,shape_y,mouse_pos,win_size,ship.facing.get());

        target.draw(&vertex_buffer,&indices,&program,&uniform!{rot_matrix:rot_matrix,trans_matrix:trans_matrix},&Default::default()).unwrap();
        target.finish().unwrap();

    }
}