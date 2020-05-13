use std::any::Any;

use crate::algebra::{Point2f, Mat2x2f};
use crate::canvas::Canvas;
use super::{GraphicObject};

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
        let mut x1: i32 = x1 as i32;
        let mut y1: i32 = y1 as i32;
        let mut x2: i32 = x2 as i32;
        let mut y2: i32 = y2 as i32;
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

    fn shear(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(LineSegs2f {
            vertices: self.vertices.iter().map(|x| Point2f::from_floats(x.x + k * x.y, x.y)).collect(),
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
