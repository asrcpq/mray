extern crate mray;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use mray::algebra::Point2f;
use mray::canvas::Canvas;
use mray::graphic_object::GraphicObjects;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
pub struct Console {
    font_size: (i32, i32),
    scaler: f32,
    pub canvas: Canvas,
}

impl Console {
    pub fn new(size: (i32, i32)) -> Console {
        let font_size = (15, 20);
        Console {
            font_size,
            scaler: 20.,
            canvas: Canvas::new((size.0 * font_size.0, size.1 * font_size.1)),
        }
    }

    pub fn render(&mut self) {
        self.canvas.flush();
        for y in 0..16_u8 {
            for x in 0..16_u8 {
                let ch: u8 = y * 16 + x;
                for graphic_object in GraphicObjects::fsd(char::from(ch))
                    .zoom(self.scaler as f32)
                    .shift(Point2f::from_floats(
                        (self.font_size.0 * x as i32) as f32,
                        (self.font_size.1 * y as i32) as f32,
                    ))
                    .into_iter()
                {
                    graphic_object.render(&mut self.canvas);
                }
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_size: (u32, u32) = (1200, 480);
    let mut console = Console::new((80, 24));

    let window = video_subsystem
        .window("fsdterm", window_size.0 as u32, window_size.1 as u32)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(
            Some(sdl2::pixels::PixelFormatEnum::RGB24),
            window_size.0,
            window_size.1,
        )
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    console.render();

    texture
        .update(None, &console.canvas.data, window_size.0 as usize * 3)
        .unwrap();

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
    'main_loop: loop {
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 100));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
    }
}
