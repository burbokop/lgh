
use std::{ops::Deref, rc::{Rc, Weak}, borrow::BorrowMut, cell::{RefCell, RefMut, Ref}, os::unix::prelude::CommandExt};

use itertools::Itertools;
use rand::distributions::uniform::SampleBorrow;
use sdl2::{render::{Canvas, RenderTarget}, event::Event, mouse::MouseButton, video::Window, ttf::Font};
use vecmat::Vector;

use crate::side::Side;

use super::fleet::Fleet;


pub struct Group {
    pub fleets: Vec<Rc<RefCell<Fleet>>>,
    pub side: Rc<RefCell<Side>>,
}

#[derive(Default)]
pub struct Scene {
    pub fleets: Vec<Rc<RefCell<Fleet>>>,
    pub groups: Vec<Group>,
    pub selected: Option<*const Fleet>
}

impl From<Vec<Rc<RefCell<Fleet>>>> for Scene {
    fn from(fleets: Vec<Rc<RefCell<Fleet>>>) -> Self {
        Self { fleets: fleets, groups: Default::default(), selected: None }
    }
}

impl Scene {
    pub fn paint(&mut self, canvas: &mut Canvas<Window>, font: &Font) {
        let selected = self.selected;
        for f in &self.fleets {
            let c = f.clone();
            let mut b: RefMut<Fleet> = c.as_ref().borrow_mut();
            let s = selected.contains(&(b.deref() as *const Fleet));

            b.paint(canvas, self, font, s);
        }
    }

    pub fn fleets(&self) -> &Vec<Rc<RefCell<Fleet>>> {
        &self.fleets
    }

    fn process_event(&mut self, e: &Event) -> bool {
        match e {
            Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                if *mouse_btn == MouseButton::Left {
                    for f in &self.fleets {
                        let c = f.clone();
                        let b: Ref<Fleet> = c.as_ref().borrow();
            

                        if b.side().as_ref().borrow().player_controled && b.in_bounding_box(Vector::from([*x as f64, *y as f64])) {
                            self.selected = Some(b.deref());
                            return true
                        }
                    }
                    self.selected = None;
                    false
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

    pub fn event(&mut self, e: &Event) -> bool {
        let selected = self.selected;
        for f in &self.fleets {
            let c = f.clone();
            let mut b: RefMut<Fleet> = c.as_ref().borrow_mut();
            let s = selected.contains(&(b.deref() as *const Fleet));

            if b.event(e, self, s) {
                return true
            }
        }
        self.process_event(e)
    }

    pub fn recalculate_groups(&mut self, dst: f64) {
        let mut next_group = 1;

        self.fleets.iter().for_each(|f| f.as_ref().borrow_mut().group = 0);
        
        for f in &self.fleets {
            for o in &self.fleets {
                let mut f = f.as_ref().borrow_mut();
                if let Ok(mut o) = o.as_ref().try_borrow_mut() {
                    if f.dst_to(&o) < dst && !f.is_enemy(&o) {
                        if f.group == 0 && o.group == 0 {
                            f.group = next_group;
                            o.group = next_group;
                            next_group = next_group + 1;                            
                        } else if f.group == 0 {
                            f.group = o.group
                        } else {
                            o.group = f.group
                        }
                    } else {
                        if f.group == 0 {
                            f.group = next_group;
                            next_group = next_group + 1;                            
                        } else if o.group == 0 {
                            o.group = next_group;
                            next_group = next_group + 1;                            
                        }
                    }
                }
            }
        }

        //let g = self.fleets.iter().group_by(|f| {
        //    f.as_ref().borrow().side.as_ptr()
        //}).map(|g| { g.map(|f| { f.as_ref().borrow().pos() }) });
    }

    pub fn update(&mut self) {
        self.recalculate_groups(150.);
        let selected = self.selected;
        for f in &self.fleets {
            let c = f.clone();
            let mut b: RefMut<Fleet> = c.as_ref().borrow_mut();
            let s = selected.contains(&(b.deref() as *const Fleet));
            b.update(self, s);

        }

        self.fleets = self.fleets.iter().filter(|f| f.as_ref().borrow().ship_count > 0.).cloned().collect::<Vec<_>>();
    }
}