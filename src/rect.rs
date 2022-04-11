use rand::{Rng, RngCore};
use vecmat::Vector;




#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: Vector<f64, 2>,
    pub size: Vector<f64, 2>
}

impl Rect {
    pub fn from_center(center: Vector<f64, 2>, size: Vector<f64, 2>) -> Self {
        Self { pos: center - size / 2., size }
    }

    pub fn random_point<R: RngCore>(&self, r: &mut R) -> Vector<f64, 2> {
        let x = r.gen_range(self.pos.x()..(self.pos.x() + self.size.x()));
        let y = r.gen_range(self.pos.y()..(self.pos.y() + self.size.y()));
        Vector::from([x, y])
    }
}