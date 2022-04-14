use sdl2::{render::{RenderTarget, Canvas}, rect::Rect, gfx::primitives::DrawRenderer, pixels::Color};
use vecmat::{Vector, Complex};


use sdl2::gfx;

pub fn draw_line<R: RenderTarget>(
    canvas: &mut Canvas<R>, 
    pos0: Vector<f64, 2>, 
    pos1: Vector<f64, 2>,
    color: Color
) {
    canvas.line(pos0.x() as i16, pos0.y() as i16, pos1.x() as i16, pos1.y() as i16, color).unwrap();
}

pub fn draw_rotated_rect<R: RenderTarget>(
    canvas: &mut Canvas<R>, 
    center: Vector<f64, 2>, 
    size: Vector<f64, 2>,
    rotor: Complex<f64>,
    fill: Option<Color>,
    stroke: Option<Color>
) {


    let lt: Vector<f64, 2> = center + (rotor * Complex::from([-size.x() / 2., -size.y() / 2.])).into();
    let lb: Vector<f64, 2> = center + (rotor * Complex::from([-size.x() / 2., size.y() / 2.])).into();
    let rt: Vector<f64, 2> = center + (rotor * Complex::from([ size.x() / 2., -size.y() / 2.])).into();
    let rb: Vector<f64, 2> = center + (rotor * Complex::from([ size.x() / 2., size.y() / 2.])).into();

    if let Some(color) = fill {
        canvas.filled_polygon(
            &[lt.x() as i16, rt.x() as i16, rb.x() as i16, lb.x() as i16], 
            &[lt.y() as i16, rt.y() as i16, rb.y() as i16, lb.y() as i16], 
            color
        ).unwrap();
    }

    if let Some(color) = stroke {
        draw_line(canvas, lt, rt, color);
        draw_line(canvas, rt, rb, color);
        draw_line(canvas, rb, lb, color);
        draw_line(canvas, lb, lt, color);
    }
}