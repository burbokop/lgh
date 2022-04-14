use std::{rc::Rc, cell::{RefCell, Ref}, convert::identity, borrow::BorrowMut};

use rand::distributions::uniform::SampleBorrow;
use sdl2::pixels::Color;

use crate::{fleet::Fleet, scene::Scene, math::map_range_vec_f64};


pub struct Side {
    pub name: String,
    pub color: Color,
    pub player_controled: bool
}


impl Side {
    pub fn new(name: String, color: Color, player_controled: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { name: name, color: color, player_controled: player_controled }))
    }

    pub fn choose_enemy(&mut self, fleet: &Fleet, scene: &Scene) -> Option<Rc<RefCell<Fleet>>> {
        let aver_dst_vec: Vec<_> = scene
            .fleets
            .iter()
            .map(|f| 
                f.as_ref()
                .try_borrow()
                .map(|f| if f.is_enemy(fleet) { Some(f.dst_to(fleet)) } else { None })
                .ok()
                .flatten()
            )
            .filter_map(identity)
            .collect();

        let aver_dst: f64 = aver_dst_vec.iter().sum::<f64>() / aver_dst_vec.len() as f64;

        scene.fleets
            .iter()
            .map(|f| f.as_ref().try_borrow().ok().map(|ff| (f, ff))).filter_map(identity).filter(|f| f.1.is_enemy(fleet))
            .max_by(|f0, f1| {
                //let f0 = f0.as_ref().try_borrow();
                //let f1 = f1.as_ref().try_borrow();
                let f0 = &f0.1;
                let f1 = &f1.1;

            
                let c0 = fleet.divide_strenght(f0);
                let dst0: f64 = f0.dst_to(fleet) / aver_dst;

                let c1 = fleet.divide_strenght(f1);
                let dst1: f64 = f1.dst_to(fleet) / aver_dst;

                f64::partial_cmp(&(c0 / dst0), &(c1 / dst1)).unwrap()

        }).map(|f|f.0.clone())
    }

    pub fn update_fleet(&mut self, fleet: &mut Fleet, scene: &Scene) {
        if !self.player_controled {

            let enemy = self.choose_enemy(fleet, scene);

            match enemy {
                Some(aa) => {
                    
                    println!("a: {}", aa.as_ref().borrow().admiral.name);
                    let w = Rc::downgrade(&aa);
                    *fleet.target.borrow_mut() = Some(w);

                    let enemy = aa.as_ref().borrow();

                    let c = fleet.divide_strenght(&enemy);
                    if c > 1.1 {
                        let dst = fleet.dst_to(&enemy);
                        let ddd = map_range_vec_f64(dst - fleet.fire_range * 0.9, 0., dst, fleet.pos, enemy.pos);
    
                        fleet.dst_pos = Some(ddd);
                    } else {
                        let dst = fleet.dst_to(&enemy);
                        let ddd = map_range_vec_f64(dst - enemy.fire_range * 1.1, 0., dst, fleet.pos, enemy.pos);
    
                        fleet.dst_pos = Some(ddd);                        
                    }


                },
                None => println!("not found"),
            }
        }
    }
}