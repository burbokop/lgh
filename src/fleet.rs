use std::{cell::{RefCell, RefMut, Ref}, rc::{Rc, Weak}, borrow::BorrowMut, time::Duration};

use sdl2::{render::{RenderTarget, Canvas}, gfx::primitives::DrawRenderer, event::Event, pixels::Color, mouse::MouseButton, ttf::Font, video::Window};
use vecmat::{Vector, Complex, prelude::Dot};

use crate::{admiral::Admiral, graphics::{draw_rotated_rect, draw_line}, scene::Scene, visual_effects::MultiFireVE, rect::Rect, side::Side, math::{map_range_f64, contrast_color}};

use sdl2::gfx;


///


#[derive(Debug)]
struct Parent {
    pub childs: Vec<Rc<RefCell<Child>>>
}

impl Parent {
    fn update(&self) {
        for c in self.childs.iter() {
            let cc: Rc<RefCell<Child>> = c.clone();

            let mut borrowed: RefMut<Child> = cc.as_ref().borrow_mut();
            borrowed.update(self);
        }
    }
}

#[derive(Default, Debug)]
struct Child {
    id: String,
    target: Option<Weak<RefCell<Child>>>
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("drop: {:?}", self.id);
    }
}

impl Child {
    fn update(&mut self, parent: &Parent) {
        for c in parent.childs.iter() {
            
            match c.as_ref().try_borrow() {
                Ok(borrowed) => {
                    if borrowed.id == "C" && self.id != borrowed.id {
                        let w = Rc::downgrade(&c);

                        //*self.target.borrow_mut() = Some(w);
                        break;
                    }
                }
                Err(_) => {}
            };
        }

        //parent.childs.iter().find(|x: &RefCell<Rc<Child>>| -> bool {
        //    let a = x.borrow();
//
        //    false
        //});
    }

    fn new(id: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Child{ id: id, target: None }))
    }
}


///



pub enum Formation {
    Triangle,
    Rect
}


pub struct Rotation {
    pub rotor: Complex<f64>,
    pub radians: f64,    
}

impl Rotation {
    pub fn new(radians: f64) -> Self {
        Self { rotor: Complex::from([ radians.cos(), radians.sin() ]), radians: radians }
    }
    pub fn from_cos(cos: f64) -> Self {
        let sin = (1. - (cos * cos)).sqrt();

        Self { rotor: Complex::from([cos, sin]), radians: cos.acos() }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::new(0.)
    }
}


pub struct Fleet {
    pub side: Rc<RefCell<Side>>,
    pub admiral: Admiral,
    pub ship_count: f64,
    pub fighting_spirit: f64, // from 0 to 1
    pub max_vel: f64,
    pub fire_range: f64,

    pub pos: Vector<f64, 2>,
    vel: Vector<f64, 2>,
    acc: Vector<f64, 2>,
    rot: Rotation,
    
    formation: Formation,
    pub dst_pos: Option<Vector<f64, 2>>,
    pub target: Option<Weak<RefCell<Fleet>>>,
    firing: bool,
    pub group: usize,

    fireVE: MultiFireVE
}




fn eFunction(x: f64, c: f64) -> f64 {
    if c != 0. {
        let y2 = 1. - (x * x) / (c * c);
        if y2 >= 0. {
            return y2.sqrt();
        }
    }
    return 0.;
}


impl Fleet {
    pub fn new(
        side: Rc<RefCell<Side>>,
        admiral: Admiral, 
        ship_count: u32,
        fighting_spirit: f64,
        pos: Vector<f64, 2>,
        max_vel: f64,
        fire_range: f64
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            side: side,
            admiral: admiral,
            ship_count: ship_count as f64,
            fighting_spirit: fighting_spirit,
            pos: pos,
            vel: Vector::from([0., 0.]),
            max_vel: max_vel,
            acc: Vector::from([0., 0.]),
            rot: Rotation::default(),
            formation: Formation::Rect,
            dst_pos: None,
            target: None,
            group: 0,
            fire_range: fire_range,
            firing: false,
            fireVE: MultiFireVE::new(&mut rand::thread_rng(), Duration::from_millis(100), Duration::from_millis(2000), Duration::from_millis(50), ship_count / 100)
        }))
    }

    pub fn size_multiplier(&self) -> f64 {
        self.ship_count as f64 * 0.01
    }

    pub fn in_bounding_box(&self, pos: Vector<f64, 2>) -> bool {
        let a = pos - self.pos;
        let s = self.size();

        a.x() < s.x() / 2. &&
        a.y() < s.y() / 2. &&
        a.x() > -s.x() / 2. &&
        a.y() > -s.y() / 2.

    }

    pub fn size(&self) -> Vector<f64, 2> {
        Vector::from([1. * self.size_multiplier(), 1.5 * self.size_multiplier()])
    }

    pub fn rect(&self) -> Rect {
        Rect::from_center(self.pos, self.size())
    }

    pub fn side(&self) -> &Rc<RefCell<Side>> {
        &self.side
    }

    pub fn is_enemy(&self, other: &Fleet) -> bool {
        self.side.as_ptr() != other.side.as_ptr()
    }

    pub fn strength(&self) -> f64 {
        self.ship_count
        * self.admiral.fighting_spirit
        * map_range_f64(self.admiral.skill_level, 0., 1., 0.5, 1.)
        * map_range_f64(self.admiral.will_to_die, 0., 1., 0.9, 1.)
    }

    pub fn divide_strenght(&self, other: &Fleet) -> f64 {
        self.strength() / other.strength()
    }

    pub fn dst_to(&self, other: &Fleet) -> f64 {
        (self.pos - other.pos).length()
    }

    pub fn paint(&mut self, canvas: &mut Canvas<Window>, scene: &Scene, font: &Font,  selected: bool) {
        let stroke_color = if selected { Some(Color::RGB(255, 20, 0)) } else { None };

        //draw_rotated_rect(canvas, self.pos, self.size(), Complex::from([1., 0.]), Color::RGB(255, 180, 255));

        let side_color = self.side.as_ref().borrow().color;
        let contrast_side_color = contrast_color(side_color);

        draw_rotated_rect(canvas, self.pos, self.size(), self.rot.rotor, Some(side_color), stroke_color);



        if let Some(dst_pos) = self.dst_pos {
            draw_line(canvas, self.pos, dst_pos, Color::RGB(255, 180, 255));
            canvas.circle(dst_pos.x() as i16, dst_pos.y() as i16, 4, Color::RGB(255, 180, 255)).unwrap();
        }


        if let Some(target) = &self.target {
            if let Some(target) = target.upgrade() {
                let b = target.borrow();
                draw_line(canvas, self.pos, b.pos, Color::RGB(255, 0, 0));
                canvas.circle(b.pos.x() as i16, b.pos.y() as i16, 4, Color::RGB(255, 0, 0)).unwrap();

                if self.firing {

                    self.fireVE.paint(canvas, &mut rand::thread_rng(), self.rect(), b.rect(), side_color);
                }
            }
        }

        let texture_creator = canvas.texture_creator();

        let surface = font
            .render(format!("{} {} : {}", self.admiral.name, self.group, self.ship_count as i32).as_str())
            .blended(Color::BLACK)
            .map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let r = sdl2::rect::Rect::new(
            self.rect().pos.x() as i32, 
            self.rect().pos.y() as i32, 
            surface.width(), 
            surface.height()
        );

        canvas.copy(&texture, None, Some(r)).unwrap();
    }

    fn proceed_kinematics(&mut self) {
        
        self.acc = -self.vel * 0.1;

        match self.dst_pos {
            Some(dst_pos) => {
                let acc_direction = (dst_pos - self.pos).normalize();

                let c = eFunction(self.vel.length(), 3.);

                self.acc += acc_direction * c * 0.01;
            },
            None => {},
        };

        self.vel = self.vel + self.acc;

        let angle = f64::atan2(self.vel.y(), self.vel.x());
        
        //let cos = (self.vel.dot(Vector::from([1., 0.]))) / self.vel.length();

        self.rot = Rotation::new(angle);
        self.pos = self.pos + self.vel;
    }

    pub fn set_target(&mut self, pos: Vector<f64, 2>, scene: &Scene) -> bool {
        for c in scene.fleets().iter() {
            match c.as_ref().try_borrow() {
                Ok(borrowed) => {
                    if borrowed.in_bounding_box(pos) {
                        let w = Rc::downgrade(&c);
                        *self.target.borrow_mut() = Some(w);
                        //*self.target.borrow_mut() = Some(w);
                        return true
                    }                    
                }
                Err(_) => {}
            };
        }
        false
    }

    pub fn map_target<R, F: FnOnce(Ref<Fleet>) -> R>(&self, f: F) -> Option<R> {
        if let Some(target) = &self.target {
            if let Some(target) = target.upgrade() {
                match target.try_borrow() {
                    Ok(borrowed) => { Some(f(borrowed)) },
                    Err(_) => None,
                }
            } else { None }
        } else { None }
    }

    pub fn map_target_mut<R, F: FnOnce(RefMut<Fleet>) -> R>(&self, f: F) -> Option<R> {
        if let Some(target) = &self.target {
            if let Some(target) = target.upgrade() {
                match target.try_borrow_mut() {
                    Ok(borrowed) => { Some(f(borrowed)) },
                    Err(_) => None,
                }
            } else { None }
        } else { None }
    }


    pub fn event(&mut self, e: &Event, scene: &Scene, selected: bool) -> bool {
            match e {
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    if *mouse_btn == MouseButton::Right {
                        if selected {
                            if !self.set_target(Vector::from([*x as f64, *y as f64]), scene) {
                                self.dst_pos = Some(Vector::from([*x as f64, *y as f64]));
                            }
                            true
                        } else { false }
                    } else {
                        false
                    }
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    false
                },
                _ => false
            }
    }

    pub fn update(&mut self, scene: &Scene, selected: bool) {
        {
            let c = self.side.clone();
            let mut borrowed = c.as_ref().borrow_mut();
            borrowed.each_fleet(self, scene);

        }

        self.proceed_kinematics();

        if let Some(tgt_pos) = self.map_target(|target| target.pos) {
            self.firing = (tgt_pos - self.pos).length() < self.fire_range;

            if self.firing {
                let ship_count = self.ship_count;
                self.map_target_mut(|mut target| {
                    target.ship_count = target.ship_count - ship_count * 0.0001;
                });
            }
        } else {
            self.firing = false;
        }
    }
}
