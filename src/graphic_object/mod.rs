use crate::algebra::{Mat2x2f, Point2f};
use crate::canvas::Canvas;
use std::any::Any;

use dyn_clone::DynClone;

pub mod line_segs;
pub mod polygon;
pub use line_segs::LineSegs2f;
pub use polygon::Polygon2f;

pub trait GraphicObject: DynClone + Sync + Send + Any {
    fn as_any(&self) -> &dyn Any;
    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject>;
    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject>;
    fn zoom(&self, k: f32) -> Box<dyn GraphicObject>;
    fn shear(&self, k: f32) -> Box<dyn GraphicObject>;

    fn render(&self, canvas: &mut Canvas);
}

dyn_clone::clone_trait_object!(GraphicObject);

// works for both counter/clockwise direction
pub fn generate_arc_vertices(center: Point2f, r: f32, theta: (f32, f32)) -> Vec<Point2f> {
    const SPLIT_R_K: f32 = 1.; // points every pixel length of arc
    let split: u32 = ((theta.1 - theta.0).abs() * SPLIT_R_K * r) as u32;
    let d_theta: f32 = (theta.1 - theta.0) / split as f32;
    let mut theta_now = theta.0;
    let mut vertices: Vec<Point2f> = Vec::new();
    for _ in 0..split + 1 {
        vertices.push(Point2f::from_polar(r, theta_now) + center);
        theta_now += d_theta;
    }
    vertices
}

pub fn generate_thick_arc(
    center: Point2f,
    r: (f32, f32),
    theta: (f32, f32),
    border_color: Option<[f32; 4]>,
    fill_color: Option<[f32; 4]>,
) -> GraphicObjects {
    let mut nodes = generate_arc_vertices(center, r.0, theta);
    nodes.extend(generate_arc_vertices(center, r.1, (theta.1, theta.0)));
    let mut graphic_objects: GraphicObjects = Default::default();
    if let Some(fill_color) = fill_color {
        graphic_objects.push(Box::new(Polygon2f {
            vertices: nodes.clone(),
            color: fill_color,
            border_color: [0., 0., 0., 0.],
        }));
    }
    nodes.push(nodes[0]);
    if let Some(border_color) = border_color {
        graphic_objects.push(Box::new(LineSegs2f {
            vertices: nodes,
            color: border_color,
        }));
    }
    graphic_objects
}

#[derive(Clone, Default)]
pub struct GraphicObjects {
    graphic_objects: Vec<Box<dyn GraphicObject>>,
}

impl GraphicObjects {
    // for debug
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.graphic_objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.graphic_objects.is_empty()
    }

    pub fn new(graphic_objects: Vec<Box<dyn GraphicObject>>) -> GraphicObjects {
        GraphicObjects { graphic_objects }
    }

    pub fn shift(&self, point2f: Point2f) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.shift(point2f))
                .collect(),
        }
    }

    pub fn rotate(&self, rotate_mat: Mat2x2f) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.rotate(rotate_mat))
                .collect(),
        }
    }

    pub fn zoom(&self, k: f32) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.zoom(k))
                .collect(),
        }
    }

    pub fn shear(&self, k: f32) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.shear(k))
                .collect(),
        }
    }

    pub fn push(&mut self, element: Box<dyn GraphicObject>) {
        self.graphic_objects.push(element);
    }

    pub fn extend(&mut self, other: GraphicObjects) {
        self.graphic_objects.extend(other.graphic_objects);
    }

    pub fn from_strs(strings: Vec<&str>) -> GraphicObjects {
        let mut graphic_objects = GraphicObjects {
            graphic_objects: Vec::new(),
        };
        for line in strings.iter() {
            let splited = line.split_whitespace().collect::<Vec<&str>>();
            match splited[0] {
                "l" => graphic_objects
                    .graphic_objects
                    .push(Box::new(LineSegs2f::from_floats(
                        splited[1..]
                            .iter()
                            .map(|x| x.parse::<f32>().expect("float parse fail"))
                            .collect(),
                    ))),
                "p" => graphic_objects
                    .graphic_objects
                    .push(Box::new(Polygon2f::from_floats(
                        vec![0f32; 4].into_iter().chain(splited[1..]
                            .iter()
                            .map(|x| x.parse::<f32>().expect("float parse fail")))
                            .collect(),
                    ))),
                "P" => graphic_objects
                    .graphic_objects
                    .push(Box::new(Polygon2f::from_floats(
                        splited[1..]
                            .iter()
                            .map(|x| x.parse::<f32>().expect("float parse fail"))
                            .collect(),
                    ))),
                _ => panic!("Format error"),
            }
        }
        graphic_objects
    }

}

impl IntoIterator for GraphicObjects {
    type Item = Box<dyn GraphicObject>;
    type IntoIter = GraphicObjectsIntoIter;
    fn into_iter(self) -> GraphicObjectsIntoIter {
        GraphicObjectsIntoIter {
            graphic_objects: self,
        }
    }
}

pub struct GraphicObjectsIntoIter {
    graphic_objects: GraphicObjects,
}

impl Iterator for GraphicObjectsIntoIter {
    type Item = Box<dyn GraphicObject>;

    fn next(&mut self) -> Option<Box<dyn GraphicObject>> {
        self.graphic_objects.graphic_objects.pop()
    }
}
