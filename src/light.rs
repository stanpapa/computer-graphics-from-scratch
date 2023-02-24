use crate::point3d::Point3D;

#[derive(PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Point,
}

pub struct Light {
    pub typ: LightType,
    pub intensity: f64,
    pub point: Option<Point3D>,
}

// impl Light {
//     pub fn new(typ: LightType, intensity: f64, point: Option<Point3D>) -> Light {
//         match typ {
//             LightType::Ambient => Self {
//                 typ,
//                 intensity,
//                 point: None,
//             },
//             _ => {
//                 if point.is_none() {
//                     panic!("Directional or Point light must have a direction or origin!");
//                 }

//                 Self {
//                     typ,
//                     intensity,
//                     point,
//                 }
//             }
//         }
//     }
// }
