use std::rc::Rc;

use sdl2::pixels::Color;
use vecmat::Vector;

use crate::{scene::Scene, fleet::{Fleet, Rotation, Formation}, admiral::Admiral, side::Side};




pub fn scenario1() -> Scene {

    let s0 = Side::new("s0".to_string(), Color::GREY, true);
    let s1 = Side::new("s1".to_string(), Color::MAGENTA, false);



    Scene::from(vec![
        Fleet::new(
            s0.clone(),
            Admiral{name: "borys".to_string(), fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
            4000, 
            1., 
            Vector::from([100.,100.]),
            15.,
            120.
        ),
        Fleet::new(
            s0.clone(),
            Admiral{name: "alex".to_string(), fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
            4000, 
            1., 
            Vector::from([100.,200.]),
            15.,
            120.
        ),
        Fleet::new(
            s0,
            Admiral{name: "igor".to_string(), fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
            2000, 
            1., 
            Vector::from([500.,100.]),
            15.,
            120.
        ),
        Fleet::new(
            s1.clone(),
            Admiral{name: "vania0".to_string(), fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
            3000, 
            1., 
            Vector::from([200.,200.]),
            5.,
            100.
        ),
        Fleet::new(
            s1.clone(),
            Admiral{name: "vania1".to_string(), fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
            3000, 
            1., 
            Vector::from([400.,400.]),
            5.,
            100.
        ),
        Fleet::new(
            s1.clone(),
            Admiral{name: "vania2".to_string(), fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
            3000, 
            1., 
            Vector::from([600.,600.]),
            5.,
            100.
        )
    ])
}