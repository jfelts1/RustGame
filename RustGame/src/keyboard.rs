extern crate glium;

//these are temporary until I have file based config coded
pub const KEY_FORWARD: glium::glutin::VirtualKeyCode = glium::glutin::VirtualKeyCode::W;
pub const KEY_BACKWARD: glium::glutin::VirtualKeyCode = glium::glutin::VirtualKeyCode::S;
pub const KEY_LEFT: glium::glutin::VirtualKeyCode = glium::glutin::VirtualKeyCode::A;
pub const KEY_RIGHT: glium::glutin::VirtualKeyCode = glium::glutin::VirtualKeyCode::D;

pub enum UserInput{
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    None,
}

pub fn handle_keyboard(state:glium::glutin::ElementState,v:u8,key_code:Option<glium::glutin::VirtualKeyCode>) -> UserInput{
    use glium::glutin::*;

    match state{
        _=>{}
        //ElementState::Pressed=>print!("Pressed "),
        //ElementState::Released=>print!("Released "),
    }

    match key_code{
        Some(kc)=>{
            if kc == KEY_FORWARD{
                //println!("forward");
                return UserInput::MoveForward;
            }
            if kc == KEY_BACKWARD{
                //println!("backward");
                return UserInput::MoveBackward;
            }
            if kc == KEY_LEFT{
                //println!("left");
                return UserInput::MoveLeft;
            }
            if kc == KEY_RIGHT{
                //println!("right");
                return UserInput::MoveRight;
            }
        },
        None => return UserInput::None,
    }
    return UserInput::None;
}
