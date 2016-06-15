use shape::*;
use std::cell::Cell;
use point2d::*;

#[derive(Debug)]
pub struct GameObject{
    ///the data that makes up the shape of the GameObject
    pub shape:Shape,
    ///the GameObjects's position on the screen in pixels
    pub pos: Cell<Point2D<f32>>,
    ///the direction that the GameObject faces in radians
    pub facing: Cell<f32>,
    ///the current velocity of the GameObject
    pub velocity: Cell<Point2D<f32>>,
}

impl GameObject{
    pub fn new(shape:Shape,pos:Point2D<f32>,facing:f32,velocity:Point2D<f32>)->GameObject{
        GameObject{
            shape : shape,
            pos:Cell::new(pos),
            facing:Cell::new(facing),
            velocity:Cell::new(velocity),
        }
    }
}

pub trait Updatable{
    fn update(&self)->bool;
}

impl Updatable for GameObject{
    fn update(&self)->bool{
        let mut p = self.pos.get();
        p+=self.velocity.get();
        self.pos.set(p);
        true
    }
}