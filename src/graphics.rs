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
    color: Color
) {


    let lt: Vector<f64, 2> = center + (rotor * Complex::from([-size.x() / 2., -size.y() / 2.])).into();
    let lb: Vector<f64, 2> = center + (rotor * Complex::from([-size.x() / 2., size.y() / 2.])).into();
    let rt: Vector<f64, 2> = center + (rotor * Complex::from([ size.x() / 2., -size.y() / 2.])).into();
    let rb: Vector<f64, 2> = center + (rotor * Complex::from([ size.x() / 2., size.y() / 2.])).into();


    draw_line(canvas, lt.into(), rt.into(), color);
    draw_line(canvas, rt.into(), rb.into(), color);
    draw_line(canvas, rb.into(), lb.into(), color);
    draw_line(canvas, lb.into(), lt.into(), color);
}