use crate::algebra::{Mat2x2f, Point2f};
use crate::canvas::Canvas;
use std::any::Any;
use std::collections::HashMap;

use dyn_clone::DynClone;
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct LineSegs2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4], // rgba
}
impl LineSegs2f {
    pub fn new(vertices: Vec<Point2f>, color: [f32; 4]) -> LineSegs2f {
        LineSegs2f { vertices, color }
    }

    pub fn from_floats(floats: Vec<f32>) -> LineSegs2f {
        let mut vertices: Vec<Point2f> = Vec::new();
        let mut iter = floats.iter();
        let r = iter.next().unwrap();

        let g = iter.next().unwrap();
        let b = iter.next().unwrap();
        let a = iter.next().unwrap();
        let color: [f32; 4] = [*r, *g, *b, *a];
        while match iter.next() {
            Some(v1) => match iter.next() {
                Some(v2) => {
                    vertices.push(Point2f::from_floats(*v1, *v2));
                    true
                }
                None => panic!("odd parse"),
            },
            None => false,
        } {}
        LineSegs2f::new(vertices, color)
    }

    #[inline]
    pub fn shift(&self, dp: Point2f) -> LineSegs2f {
        LineSegs2f {
            vertices: self.vertices.iter().map(|x| *x + dp).collect(),
            color: self.color,
        }
    }

    #[inline]
    fn wu(x1: f32, y1: f32, x2: f32, y2: f32, color: [f32; 4], canvas: &mut Canvas) {
        let mut x1: i32 = x1.round() as i32;
        let mut y1: i32 = y1.round() as i32;
        let mut x2: i32 = x2.round() as i32;
        let mut y2: i32 = y2.round() as i32;
        let mut dx = x2 - x1;
        let dy = y2 - y1;
        canvas.set_color([color[0], color[1], color[2]]);
        if dx == 0 {
            if dy < 0 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for y in y1..y2 + 1 {
                canvas.putpixel(x1, y, color[3]);
            }
            return;
        }

        if dy == 0 {
            if dx < 0 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for x in x1..x2 + 1 {
                canvas.putpixel(x, y1, color[3]);
            }
            return;
        }

        if dx == dy {
            if dx < 0 {
                x1 = x2;
                y1 = y2;
                dx = -dx;
            }
            for i in 0..dx + 1 {
                canvas.putpixel(x1 + i, y1 + i, color[3]);
            }
            return;
        }

        if dx == -dy {
            if dx < 0 {
                x1 = x2;
                y1 = y2;
                dx = -dx;
            }
            for i in 0..dx + 1 {
                canvas.putpixel(x1 + i, y1 - i, color[3]);
            }
            return;
        }

        let k = dy as f32 / dx as f32;
        let mut e: f32 = 0.;

        if dx + dy < 0 {
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut y1, &mut y2);
        }

        if k > 0. && k < 1. {
            let mut py = y1;
            for px in x1..x2 {
                canvas.putpixel(px, py, color[3] * (1. - e));
                canvas.putpixel(px, py + 1, color[3] * e);
                e += k;
                if e >= 1. {
                    py += 1;
                    e -= 1.;
                }
            }
        } else if k > 1. {
            let mut px = x1;
            for py in y1..y2 {
                canvas.putpixel(px, py, color[3] * (1. - e));
                canvas.putpixel(px + 1, py, color[3] * e);
                e += 1. / k;
                if e >= 1. {
                    px += 1;
                    e -= 1.;
                }
            }
        } else if k > -1. && k < 0. {
            let mut py = y1;
            for px in x1..x2 {
                canvas.putpixel(px, py, color[3] * (1. + e));
                canvas.putpixel(px, py - 1, color[3] * -e);
                e += k;
                if e <= -1. {
                    py -= 1;
                    e += 1.0;
                }
            }
        } else if k < -1. {
            let mut px = x2;
            for py in (y1..y2).rev() {
                canvas.putpixel(px, py, color[3] * (1. - e));
                canvas.putpixel(px + 1, py, color[3] * e);
                e += -1. / k;
                if e >= 1. {
                    px += 1;
                    e -= 1.;
                }
            }
        }
    }
}

pub trait GraphicObject: DynClone + Sync + Send + Any {
    fn as_any(&self) -> &dyn Any;
    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject>;
    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject>;
    fn zoom(&self, k: f32) -> Box<dyn GraphicObject>;

    fn render(&self, canvas: &mut Canvas);
}

dyn_clone::clone_trait_object!(GraphicObject);

impl GraphicObject for LineSegs2f {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject> {
        Box::new(self.shift(dp))
    }

    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject> {
        Box::new(LineSegs2f {
            vertices: self.vertices.iter().map(|x| rotate_mat * *x).collect(),
            color: self.color,
        })
    }

    fn zoom(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(LineSegs2f {
            vertices: self.vertices.iter().map(|x| *x * k).collect(),
            color: self.color,
        })
    }

    fn render(&self, mut canvas: &mut Canvas) {
        let mut flag = false;
        let mut x1: f32 = 0.; // convince compiler
        let mut x2: f32;
        let mut y1: f32 = 0.; // convince compiler
        let mut y2: f32;
        for vertex in self.vertices.iter() {
            if !flag {
                flag = true;
                x1 = vertex.x;
                y1 = vertex.y;
            } else {
                x2 = vertex.x;
                y2 = vertex.y;
                LineSegs2f::wu(x1, y1, x2, y2, self.color, &mut canvas);
                x1 = x2;
                y1 = y2;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Polygon2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4],
}

impl GraphicObject for Polygon2f {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| *x + dp).collect(),
            color: self.color,
        })
    }

    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| rotate_mat * *x).collect(),
            color: self.color,
        })
    }

    fn zoom(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| *x * k).collect(),
            color: self.color,
        })
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.set_color([self.color[0], self.color[1], self.color[2]]);
        if self.vertices.len() < 3 {
            return;
        }
        #[derive(Debug)]
        struct Edge {
            pub startx: i32,
            pub starty: i32,
            pub endx: i32,
            pub endy: i32,
            pub dxy: f32,
            pub current_x: f32,
        }
        let mut edges: Vec<Edge> = Vec::new();
        let last_vertex = self.vertices.last().unwrap();
        let mut last_vertex = (last_vertex.x as i32, last_vertex.y as i32);
        for vertex in self.vertices.iter() {
            let vertex_i32 = (vertex.x as i32, vertex.y as i32);
            // dy = 0 is thrown
            if vertex_i32.1 > last_vertex.1 {
                edges.push(Edge {
                    startx: last_vertex.0,
                    starty: last_vertex.1,
                    endx: vertex_i32.0,
                    endy: vertex_i32.1,
                    dxy: (vertex_i32.0 - last_vertex.0) as f32
                        / (vertex_i32.1 - last_vertex.1) as f32,
                    current_x: last_vertex.0 as f32,
                })
            } else {
                edges.push(Edge {
                    startx: vertex_i32.0,
                    starty: vertex_i32.1,
                    endx: last_vertex.0,
                    endy: last_vertex.1,
                    dxy: (vertex_i32.0 - last_vertex.0) as f32
                        / (vertex_i32.1 - last_vertex.1) as f32,
                    current_x: vertex_i32.0 as f32,
                })
            }
            last_vertex = vertex_i32;
        }

        // from big to small, for pop_back
        edges.sort_by(|x, y| y.starty.partial_cmp(&x.starty).unwrap());
        let mut pop_yend_list = edges.iter().map(|x| x.endy).collect::<Vec<i32>>();
        pop_yend_list.sort();
        let mut pop_p: usize = 0;
        // should use balanced tree for massive points
        let mut sorted_processing_edges: Vec<Edge> = Vec::new();
        let mut current_y = edges.last().unwrap().starty;
        loop {
            // debug checkpoint
            // if sorted_processing_edges.len() %2 != 0 {
            //     panic!("Odd processing edges!");
            // }

            let mut need_resort_flag = false;
            // push
            while !edges.is_empty() && edges.last().unwrap().starty == current_y {
                sorted_processing_edges.push(edges.pop().unwrap());
                need_resort_flag = true;
            }

            // pops do not need re-sort
            while pop_p < pop_yend_list.len() && pop_yend_list[pop_p] == current_y {
                sorted_processing_edges.retain(|x| x.endy != current_y);
                pop_p += 1;
            }

            // exit immediately after pop
            if sorted_processing_edges.is_empty() {
                break;
            }

            if need_resort_flag {
                sorted_processing_edges.sort_by(|x, y| {
                    x.current_x
                        .partial_cmp(&y.current_x)
                        .unwrap()
                        .then(x.endx.partial_cmp(&y.endx).unwrap())
                });
            }

            let mut draw_on = false;
            let mut iter = sorted_processing_edges.iter_mut();
            let mut last_x: i32;
            {
                let mut first_edge = iter.next().unwrap();
                last_x = first_edge.current_x as i32;
                first_edge.current_x += first_edge.dxy;
            }
            for each_processing_edge in iter {
                draw_on = !draw_on;
                if draw_on {
                    let current_x = each_processing_edge.current_x as i32;
                    // debug checkpoint
                    // if last_x > current_x {
                    //     println!("{:?}", sorted_processing_edges);
                    //     panic!("not sorted!");
                    // }
                    for x in last_x..current_x {
                        canvas.putpixel(x, current_y, self.color[3]);
                    }
                }
                last_x = each_processing_edge.current_x as i32;
                each_processing_edge.current_x += each_processing_edge.dxy;
            }

            current_y += 1;
        }
    }
}

impl Polygon2f {
    pub fn new(vertices: Vec<Point2f>, color: [f32; 4]) -> Polygon2f {
        Polygon2f { vertices, color }
    }

    pub fn from_floats(floats: Vec<f32>) -> Polygon2f {
        let mut vertices: Vec<Point2f> = Vec::new();
        let mut iter = floats.iter();
        let r = iter.next().unwrap();
        let g = iter.next().unwrap();
        let b = iter.next().unwrap();
        let a = iter.next().unwrap();
        let color: [f32; 4] = [*r, *g, *b, *a];
        while match iter.next() {
            Some(v1) => match iter.next() {
                Some(v2) => {
                    vertices.push(Point2f::from_floats(*v1, *v2));
                    true
                }
                None => panic!("odd parse"),
            },
            None => false,
        } {}
        Polygon2f::new(vertices, color)
    }
}

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

    pub fn fsd(c: char) -> GraphicObjects {
        lazy_static! {
            static ref SEGMENTS: Vec<GraphicObjects> = vec![
                GraphicObjects::from_strs(vec!["p 1 1 1 0.5 0.2 0.1 0.3 0.2 0.7 0.2 0.8 0.1"]),
                GraphicObjects::from_strs(vec!["p 1 1 1 0.5 0.7 0.2 0.8 0.1 0.8 0.5 0.7 0.5"]),
                GraphicObjects::from_strs(vec!["p 1 1 1 0.5 0.7 0.5 0.7 0.8 0.8 0.9 0.8 0.5"]),
                GraphicObjects::from_strs(vec!["p 1 1 1 0.5 0.8 0.9 0.2 0.9 0.3 0.8 0.7 0.8"]),
                GraphicObjects::from_strs(vec!["p 1 1 1 0.5 0.2 0.9 0.3 0.8 0.3 0.5 0.2 0.5"]),
                GraphicObjects::from_strs(vec!["p 1 1 1 0.5 0.3 0.5 0.3 0.2 0.2 0.1 0.2 0.5"]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.3 0.45 0.45 0.45 0.5 0.5 0.45 0.55 0.3 0.55"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.55 0.55 0.5 0.5 0.55 0.45 0.7 0.45 0.7 0.55"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.3 0.2 0.45 0.325 0.45 0.45 0.3 0.325"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.45 0.2 0.55 0.2 0.55 0.45 0.5 0.5 0.45 0.45"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.7 0.2 0.55 0.325 0.55 0.45 0.7 0.325"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.3 0.8 0.3 0.675 0.45 0.55 0.45 0.675"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.45 0.55 0.5 0.5 0.55 0.55 0.55 0.8 0.45 0.8"
                ]),
                GraphicObjects::from_strs(vec![
                    "p 1 1 1 0.5 0.7 0.8 0.7 0.675 0.55 0.55 0.55 0.675"
                ]),
            ];
            static ref CHAR_MAP: HashMap<char, Vec<usize>> = (0..=255u8)
                .map(|x| char::from(x))
                .map(|c| (
                    c,
                    match c {
                        '!' => vec![3, 9],
                        '"' => vec![5, 9],
                        '#' => vec![1, 2, 3, 6, 7, 9, 12],
                        '$' => vec![0, 2, 3, 5, 6, 7, 9, 12],
                        '%' => vec![2, 5, 10, 11],
                        '&' => vec![0, 3, 4, 6, 8, 10, 13],
                        '\'' => vec![9],
                        '(' => vec![7, 10, 13],
                        ')' => vec![6, 8, 11],
                        '*' => vec![8, 9, 10, 11, 12, 13],
                        '+' => vec![6, 7, 9, 12],
                        ',' => vec![11],
                        '-' => vec![6, 7],
                        '.' => vec![12],
                        '/' => vec![10, 11],
                        '0' => vec![0, 1, 2, 3, 4, 5, 10, 11],
                        '1' => vec![1, 2, 10],
                        '2' => vec![0, 1, 3, 4, 6, 7],
                        '3' => vec![0, 1, 2, 3, 7],
                        '4' => vec![1, 2, 5, 6, 7],
                        '5' => vec![0, 2, 3, 5, 6, 7],
                        '6' => vec![0, 2, 3, 4, 5, 6, 7],
                        '7' => vec![0, 10, 12],
                        '8' => vec![0, 1, 2, 3, 4, 5, 6, 7],
                        '9' => vec![0, 1, 2, 5, 6, 7],
                        ':' => vec![9, 13],
                        ';' => vec![9, 11],
                        '<' => vec![10, 13],
                        '=' => vec![3, 6, 7],
                        '>' => vec![8, 11],
                        '?' => vec![0, 5, 10, 12],
                        '@' => vec![0, 1, 2, 3, 4, 5, 8, 10, 11, 13],
                        'A' => vec![0, 1, 2, 4, 5, 6, 7],
                        'B' => vec![0, 1, 2, 3, 7, 9, 12],
                        'C' => vec![0, 3, 4, 5],
                        'D' => vec![0, 1, 2, 3, 9, 12],
                        'E' => vec![0, 3, 4, 5, 6, 7],
                        'F' => vec![0, 4, 5, 6, 7],
                        'G' => vec![0, 2, 3, 4, 5, 7],
                        'H' => vec![1, 2, 4, 5, 6, 7],
                        'I' => vec![0, 3, 9, 12],
                        'J' => vec![1, 2, 3, 4],
                        'K' => vec![4, 5, 6, 10, 13],
                        'L' => vec![3, 4, 5],
                        'M' => vec![1, 2, 4, 5, 8, 10],
                        'N' => vec![1, 2, 4, 5, 8, 13],
                        'O' => vec![0, 1, 2, 3, 4, 5],
                        'P' => vec![0, 1, 4, 5, 6, 7],
                        'Q' => vec![0, 1, 2, 3, 4, 5, 13],
                        'R' => vec![0, 1, 4, 5, 6, 7, 13],
                        'S' => vec![0, 2, 3, 7, 8],
                        'T' => vec![0, 9, 12],
                        'U' => vec![1, 2, 3, 4, 5],
                        'V' => vec![4, 5, 10, 11],
                        'W' => vec![1, 2, 4, 5, 11, 13],
                        'X' => vec![8, 10, 11, 13],
                        'Y' => vec![8, 10, 12],
                        'Z' => vec![0, 3, 10, 11],
                        '[' => vec![0, 3, 4, 5],
                        '\\' => vec![8, 13],
                        ']' => vec![0, 1, 2, 3],
                        '^' => vec![0, 1, 5],
                        '_' => vec![3],
                        '`' => vec![6],
                        'a' => vec![2, 3, 4, 6, 7, 13],
                        'b' => vec![3, 4, 5, 6, 13],
                        'c' => vec![3, 4, 6, 7],
                        'd' => vec![1, 2, 3, 7, 11],
                        'e' => vec![3, 4, 6, 11],
                        'f' => vec![7, 9, 12],
                        'g' => vec![2, 3, 7, 13],
                        'h' => vec![2, 4, 5, 6, 7],
                        'i' => vec![4],
                        'j' => vec![1, 2, 3],
                        'k' => vec![9, 10, 12, 13],
                        'l' => vec![1, 2],
                        'm' => vec![2, 4, 6, 7, 12],
                        'n' => vec![4, 6, 13],
                        'o' => vec![2, 3, 4, 6, 7],
                        'p' => vec![4, 6, 11],
                        'q' => vec![2, 7, 13],
                        'r' => vec![4, 6],
                        's' => vec![3, 7, 13],
                        't' => vec![3, 4, 5, 6],
                        'u' => vec![2, 3, 4],
                        'v' => vec![4, 11],
                        'w' => vec![2, 4, 11, 13],
                        'x' => vec![2, 5, 8, 10, 11, 13],
                        'y' => vec![1, 2, 3, 7, 9],
                        'z' => vec![3, 6, 11],
                        '{' => vec![0, 3, 6, 8, 11],
                        '|' => vec![9, 12],
                        '}' => vec![0, 3, 7, 10, 13],
                        '~' => vec![0],
                        _ => Vec::new(),
                    }
                ))
                .collect();
        }
        let mut result: GraphicObjects = Default::default();
        for id in CHAR_MAP.get(&c).unwrap_or(&Vec::new()) {
            result.extend(SEGMENTS[*id].clone());
        }
        result
    }

    pub fn into_iter(self) -> GraphicObjectsIntoIter {
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
