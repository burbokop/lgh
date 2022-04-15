use std::{rc::Rc, cell::{RefCell, Ref}, convert::identity, borrow::BorrowMut};

use rand::distributions::uniform::SampleBorrow;
use sdl2::pixels::Color;

use crate::{fleet::Fleet, scene::{Scene, Group}, math::map_range_vec_f64};
use std::hash::Hash;
#[derive(Debug)]
pub enum ChooseEnemy {
    ByStrength,
    ByGroupStrength,
    MulBoth
}

#[derive(Debug)]
pub struct Side {
    pub name: String,
    pub color: Color,
    pub player_controled: bool,
    pub choose_enemy: ChooseEnemy,
    pub raiting: f64
}

impl Hash for Side {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Side {
    fn assert_receiver_is_total_eq(&self) {}
}


impl Side {
    pub fn new(name: String, color: Color, player_controled: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { name: name, color: color, player_controled: player_controled, choose_enemy: ChooseEnemy::ByGroupStrength, raiting: 0. }))
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

        let fleet_group = &scene.group_by_fleet(fleet);
        let gf_strength = fleet_group.map(|g| g.strength_not_borrow(fleet)).unwrap_or(1.);


        scene.fleets
            .iter()
            .map(|f| f.as_ref().try_borrow().ok().map(|ff| (f, ff))).filter_map(identity).filter(|f| f.1.is_enemy(fleet))
            .max_by(|f0, f1| {

                let g0 = scene.group_by_fleet(&f0.1);
                let g1 = scene.group_by_fleet(&f1.1);

                let g0_strength = g0.map(|g| g.strength()).unwrap_or(1.);
                let g1_strength = g1.map(|g| g.strength()).unwrap_or(1.);
                //let f0 = f0.as_ref().try_borrow();
                //let f1 = f1.as_ref().try_borrow();
                let f0 = &f0.1;
                let f1 = &f1.1;

            
                let c0 = fleet.divide_strenght(f0);
                let gc0 = gf_strength / g0_strength;
                let dst0: f64 = f0.dst_to(fleet) / aver_dst;

                let c1 = fleet.divide_strenght(f1);
                let gc1 = gf_strength / g1_strength;
                let dst1: f64 = f1.dst_to(fleet) / aver_dst;

                match self.choose_enemy {
                    ChooseEnemy::ByStrength => f64::partial_cmp(&(c0 / dst0), &(c1 / dst1)).unwrap(),
                    ChooseEnemy::ByGroupStrength => f64::partial_cmp(&(gc0 / dst0), &(gc1 / dst1)).unwrap(),
                    ChooseEnemy::MulBoth => f64::partial_cmp(&(c0 * gc0 / dst0), &(c1 * gc1 / dst1)).unwrap(),
                }
                

        }).map(|f|f.0.clone())
    }

    pub fn update(&mut self, scene: &Scene) {
        self.raiting = scene.fleets_by_side(self).map(|f| f.ship_count).sum();
    }

    pub fn each_group(&mut self, group: &Group, scene: &Scene) {

    }

    pub fn each_fleet(&mut self, fleet: &mut Fleet, scene: &Scene) {
        if !self.player_controled {

            let enemy = self.choose_enemy(fleet, scene);

            match enemy {
                Some(aa) => {
                    
                    let w = Rc::downgrade(&aa);
                    *fleet.target.borrow_mut() = Some(w);

                    let enemy = aa.as_ref().borrow();

                    let fleet_group = scene.group_by_fleet(&fleet);
                    let enemy_group = scene.group_by_fleet(&enemy);
    
                    let fgs = fleet_group.map(|g| g.strength_not_borrow(fleet)).unwrap_or(1.);
                    let egs = enemy_group.map(|g| g.strength_not_borrow(&enemy)).unwrap_or(1.);    

                    let c = match self.choose_enemy {
                        ChooseEnemy::ByStrength => fleet.divide_strenght(&enemy),
                        ChooseEnemy::ByGroupStrength => fgs / egs,
                        ChooseEnemy::MulBoth => fleet.divide_strenght(&enemy) * fgs / egs,
                    };

                    println!("enemy: {} -> {} (f: {}, e: {})", aa.as_ref().borrow().admiral.name, c, fgs, egs);

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