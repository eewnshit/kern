pub mod square;
pub mod line;
pub mod image;

pub trait PhysicalObject {
    fn draw(&self);
    fn is_colliding(&self);
}