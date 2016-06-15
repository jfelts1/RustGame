use point2d::*;
use std::f32;

pub fn move_forward(pos:&mut Point2D<f32>,accelleration:f32,rotation:f32){
    let rot = rotation + f32::consts::PI/2.0;
    let (y,x) = rot.sin_cos();
    pos.y -= accelleration*y;
    pos.x += accelleration*x;
}

pub fn move_backward(pos:&mut Point2D<f32>,accelleration:f32,rotation:f32){
    let rot = rotation+f32::consts::PI/2.0;
    let (y,x) = rot.sin_cos();
    pos.y += accelleration*y;
    pos.x -= accelleration*x;
}

pub fn move_left(pos:&mut Point2D<f32>,accelleration:f32,rotation:f32){
    let (y,x) = rotation.sin_cos();
    pos.y += accelleration*y;
    pos.x -= accelleration*x;
}

pub fn move_right(pos:&mut Point2D<f32>,acelleration:f32,rotation:f32){
    let (y,x) = rotation.sin_cos();
    pos.y -= acelleration*y;
    pos.x += acelleration*x;
}
