use std::time::{Instant, Duration};

use rand::{RngCore, Rng};
use sdl2::{render::{RenderTarget, Canvas}, pixels::Color};
use vecmat::Vector;

use crate::{rect::Rect, graphics::draw_line};





pub struct FireVE {
    instant: Instant,
    interval_in: Duration,
    interval_out: Duration,
    is_in: bool,
    p0: Vector<f64, 2>,
    p1: Vector<f64, 2>
}

impl FireVE {
    pub fn new(
        interval_in: Duration,
        interval_out: Duration,
    ) -> Self {
        Self {
            instant: Instant::now(),
            interval_in: interval_in,
            interval_out: interval_out,
            is_in: false,
            p0: Vector::default(),
            p1: Vector::default()
        }
    }

    pub fn paint<T: RenderTarget, R: RngCore>(&mut self, canvas: &mut Canvas<T>, rng: &mut R, source_rect: Rect, target_rect: Rect, color: Color) {
        if self.is_in {
            draw_line(canvas, self.p0, self.p1, color);
        }


        let now = Instant::now();
        if self.is_in {
            if (now - self.instant) > self.interval_in {
                self.is_in = false;
                self.instant = now;
            }
        } else {
            if (now - self.instant) > self.interval_out {
                self.is_in = true;
                self.p0 = source_rect.random_point(rng);
                self.p1 = target_rect.random_point(rng);    
    
                self.instant = now;
            }
        }

    }
}


pub struct MultiFireVE {
    fires: Vec<FireVE>
}

impl MultiFireVE {
    pub fn new<R: RngCore>(
        rng: &mut R,
        interval_in: Duration,
        interval_out: Duration,
        interval_delta: Duration,
        count: u32
    ) -> Self {
        Self {
            fires: (0..count).map(|_| {
                let id = rng.gen_range((interval_in - interval_delta)..(interval_in + interval_delta));
                let od = rng.gen_range((interval_out - interval_delta)..(interval_out + interval_delta));
    
                FireVE::new(id, od)
            }).collect()
        }
    }

    pub fn paint<T: RenderTarget, R: RngCore>(&mut self, canvas: &mut Canvas<T>, rng: &mut R, source_rect: Rect, target_rect: Rect, color: Color) {

        for f in &mut self.fires {
            f.paint(canvas, rng, source_rect, target_rect, color)
        }
    }

}

