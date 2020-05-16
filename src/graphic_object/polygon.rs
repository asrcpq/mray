use std::any::Any;

use crate::algebra::{Point2f, Mat2x2f};
use crate::canvas::Canvas;
use super::{LineSegs2f, GraphicObject};

#[derive(Clone, Debug)]
pub struct Polygon2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4],
    pub border_color: [f32; 4],
}

impl GraphicObject for Polygon2f {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| *x + dp).collect(),
            color: self.color,
            border_color: self.border_color,
        })
    }

    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| rotate_mat * *x).collect(),
            color: self.color,
            border_color: self.border_color,
        })
    }

    fn zoom(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| *x * k).collect(),
            color: self.color,
            border_color: self.border_color,
        })
    }

    fn shear(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| Point2f::from_floats(x.x + k * x.y, x.y)).collect(),
            color: self.color,
            border_color: self.border_color,
        })
    }

    fn set_color(&mut self,  new_color: Vec<f32>) {
        assert_eq!(new_color.len(), 8);
        for i in 0..4 {
            self.border_color[i] = new_color[i];
        }
        for i in 4..8 {
            self.color[i - 4] = new_color[i];
        }
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
                    for x in last_x + 1..current_x + 1 {
                        canvas.putpixel(x, current_y, self.color[3]);
                    }
                }
                last_x = each_processing_edge.current_x as i32;
                each_processing_edge.current_x += each_processing_edge.dxy;
            }

            current_y += 1;
        }
        // draw border
        if self.border_color[3] != 0. {
            let mut border_vertices = self.vertices.clone();
            border_vertices.push(border_vertices[0]);
            LineSegs2f {
                vertices: border_vertices,
                color: self.border_color,
            }.render(canvas);
        }
    }
}

impl Polygon2f {
    pub fn new(vertices: Vec<Point2f>, color: [f32; 4], border_color: [f32; 4]) -> Polygon2f {
        Polygon2f { vertices, color, border_color }
    }

    pub fn from_floats(floats: Vec<f32>) -> Polygon2f {
        let mut vertices: Vec<Point2f> = Vec::new();
        let mut iter = floats.iter();
        let mut border_color = [0f32; 4];
        for c in border_color.iter_mut() {
            *c = *iter.next().unwrap();
        }
        let mut color = [0f32; 4];
        for c in color.iter_mut() {
            *c = *iter.next().unwrap();
        }
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
        Polygon2f::new(vertices, color, border_color)
    }
}
