use glium::glutin::*;
use glium::backend::glutin_backend::*;
use point2d::*;
use game_object_types::game_object::{GameObject,Updatable};
#[path="../keyboard.rs"]
use keyboard;
use utils::physics;

//these are temporary until I have file based config coded
const ACCELL:f32 = 1.0;

pub fn user_input(display:&GlutinFacade,mouse_pos:&mut Point2D<i32>,ship:&GameObject){
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
                    let mut vel = ship.velocity.get();
                    match input{
                        MoveForward=>{
                            physics::move_forward(&mut vel,ACCELL,ship.facing.get());
                            ship.velocity.set(vel);
                        },
                        MoveBackward=>{
                            physics::move_backward(&mut vel,ACCELL,ship.facing.get());
                            ship.velocity.set(vel);
                        },
                        MoveLeft=>{
                            physics::move_left(&mut vel,ACCELL,ship.facing.get());
                            ship.velocity.set(vel);
                        },
                        MoveRight=>{
                            physics::move_right(&mut vel,ACCELL,ship.facing.get());
                            ship.velocity.set(vel);
                        },
                        _=>{}
                    }
                },
                _=> ()
            }
        }
}