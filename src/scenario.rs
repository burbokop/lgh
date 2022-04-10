use std::rc::Rc;

use vecmat::Vector;

use crate::{scene::Scene, fleet::{Fleet, Rotation, Formation}, admiral::Admiral};




pub fn scenario1() -> Scene {
    Scene::from(vec![
        Fleet::new(
        Admiral{fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
        4000, 
        1., 
        Vector::from([100.,100.]),
        10.,
        100.
    ),
    Fleet::new(
        Admiral{fighting_spirit:1.,will_to_die:1.,skill_level:1.}, 
        6000, 
        1., 
        Vector::from([200.,200.]),
        5.,
        120.
    )
    ])
}