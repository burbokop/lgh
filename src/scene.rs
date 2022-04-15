
use std::{ops::Deref, rc::{Rc, Weak}, borrow::BorrowMut, cell::{RefCell, RefMut, Ref}, os::unix::prelude::CommandExt, convert::identity};

use itertools::Itertools;
use rand::distributions::uniform::SampleBorrow;
use sdl2::{render::{Canvas, RenderTarget}, event::Event, mouse::MouseButton, video::Window, ttf::Font};
use vecmat::Vector;

use crate::{side::Side, graphics::draw_text};

use super::fleet::Fleet;


pub struct Group {
    pub id: usize,
    pub fleets: Vec<Rc<RefCell<Fleet>>>,
    pub side: Rc<RefCell<Side>>,
}

impl Group {
    pub fn new(id: usize, fleets: Vec<Rc<RefCell<Fleet>>>) -> Self {
        let side = fleets[0].as_ref().borrow().side.clone();
        Self { id: id, fleets: fleets, side: side }
    }
    pub fn strength(&self) -> f64 {
        self.fleets.iter().map(|f|f.as_ref().borrow().strength()).sum()
    }
    pub fn strength_not_borrow(&self, fleet: &Fleet) -> f64 {
        self.fleets.iter().filter(|f| f.as_ref().as_ptr() as *const Fleet != fleet as *const Fleet).map(|f|f.as_ref().borrow().strength()).sum::<f64>() + fleet.strength()
    }
}

#[cfg(test)]
mod group_tests {
    use sdl2::pixels::Color;

    use crate::{fleet::Fleet, side::Side};

    use super::Group;

    #[test]
    fn strength_not_borrow_test() {
        let side = Side::new("s".to_string(), Color::BLACK, false);
        let flt0 = Fleet::new(side.clone(), crate::admiral::Admiral { name: "1".to_string(), fighting_spirit: 1., will_to_die: 1., skill_level: 1. }, 2000, 1., Default::default(), 1., 1.); 
        let flt1 = Fleet::new(side.clone(), crate::admiral::Admiral { name: "2".to_string(), fighting_spirit: 1., will_to_die: 1., skill_level: 1. }, 2000, 1., Default::default(), 1., 1.); 
        let flt2 = Fleet::new(side.clone(), crate::admiral::Admiral { name: "3".to_string(), fighting_spirit: 1., will_to_die: 1., skill_level: 1. }, 2000, 1., Default::default(), 1., 1.); 

        let sum = {
            flt0.as_ref().borrow().strength() + flt1.as_ref().borrow().strength() + flt2.as_ref().borrow().strength()
        };

        let group = Group { id: 1, fleets: vec![flt0, flt1.clone(), flt2], side: side  };

        let f = flt1.as_ref().borrow_mut();
        assert_eq!(group.strength_not_borrow(&f), sum);
    }

    #[test]
    #[should_panic]
    fn strength_should_panic_test() {
        let side = Side::new("s".to_string(), Color::BLACK, false);
        let flt0 = Fleet::new(side.clone(), crate::admiral::Admiral { name: "1".to_string(), fighting_spirit: 1., will_to_die: 1., skill_level: 1. }, 2000, 1., Default::default(), 1., 1.); 
        let flt1 = Fleet::new(side.clone(), crate::admiral::Admiral { name: "2".to_string(), fighting_spirit: 1., will_to_die: 1., skill_level: 1. }, 2000, 1., Default::default(), 1., 1.); 
        let flt2 = Fleet::new(side.clone(), crate::admiral::Admiral { name: "3".to_string(), fighting_spirit: 1., will_to_die: 1., skill_level: 1. }, 2000, 1., Default::default(), 1., 1.); 

        let sum = {
            flt0.as_ref().borrow().strength() + flt1.as_ref().borrow().strength() + flt2.as_ref().borrow().strength()
        };

        let group = Group { id: 1, fleets: vec![flt0, flt1.clone(), flt2], side: side  };

        let f = flt1.as_ref().borrow_mut();
        assert_eq!(group.strength(), sum);
        println!("{}", f.ship_count)
    }
}

#[derive(Default)]
pub struct Scene {
    pub fleets: Vec<Rc<RefCell<Fleet>>>,
    pub groups: Vec<Group>,
    pub sides: Vec<Rc<RefCell<Side>>>,
    pub selected: Option<*const Fleet>
}

impl From<Vec<Rc<RefCell<Fleet>>>> for Scene {
    fn from(fleets: Vec<Rc<RefCell<Fleet>>>) -> Self {
        let s: Vec<_> = fleets
            .iter()
            .unique_by(|f| f.as_ref().borrow().side.clone().as_ref().borrow().deref().name.clone())
            .cloned()
            .map(|f| f.as_ref().borrow().side.clone())
            .collect();

        Self { fleets: fleets, groups: Default::default(), sides: s, selected: None }
    }
}

impl Scene {
    pub fn paint(&mut self, canvas: &mut Canvas<Window>, font: &Font) {


        let mut side_y: f64 = 10.;
        for side in &self.sides {
            let borrowed = side.as_ref().borrow();

            draw_text(canvas, font, format!("{}: {}", borrowed.name, borrowed.raiting as i32).as_str(), Vector::from([10., side_y]));

            side_y = side_y + 20.;
        }


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

    pub fn try_borrow_fleets(&self) -> impl Iterator<Item=Ref<Fleet>> {
        self.fleets.iter().map(|f|f.as_ref().try_borrow()).filter_map(|f| f.ok())
    }

    pub fn fleets_by_side(&self, side: &Side) -> impl Iterator<Item=Ref<Fleet>> {
        let ptr = side as *const Side;
        self.try_borrow_fleets().filter(move |f| f.side.as_ref().as_ptr() as *const Side == ptr)
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

            self.groups = self.fleets
                .iter()
                .group_by(|f| f.as_ref().borrow().group).into_iter()
                .map(|g| Group::new(g.0, g.1.cloned().collect())).collect();
        }

        //let g = self.fleets.iter().group_by(|f| {
        //    f.as_ref().borrow().side.as_ptr()
        //}).map(|g| { g.map(|f| { f.as_ref().borrow().pos() }) });
    }

    //pub fn group_by_fleet_rc(&self, fleet: &Rc<RefCell<Fleet>>) -> Option<&Group> {
    //    self.groups.iter().find(|group| group.fleets.iter().find(|f| f.as_ref().as_ptr() == fleet.as_ref().as_ptr()).is_some())
    //}

    pub fn group_by_fleet(&self, fleet: &Fleet) -> Option<&Group> {
        self.groups.iter().find(|group| group.id == fleet.group)
    }


    pub fn update(&mut self) {
        self.recalculate_groups(150.);

        for side in &self.sides {
            side.clone().as_ref().borrow_mut().update(self)
        }

        for group in &self.groups {
            let c = group.side.clone();
            let mut borrowed = c.as_ref().borrow_mut();
            borrowed.each_group(group, self)
        }

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