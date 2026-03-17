use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Platform {
    canvas: Canvas<Window>,
}

impl Platform {
    pub fn new(title: &str, width: u32, height: u32, scale: u32) -> (Self, sdl3::EventPump) {
        let sdl_context = sdl3::init().expect("Errore inizializzazione SDL3");
        let video_subsystem = sdl_context.video().expect("Errore sistema video");

        let window = video_subsystem
            .window(title, width * scale, height * scale)
            .position_centered()
            .build()
            .expect("Errore creazione finestra");

        let canvas = window.into_canvas();
        let event_pump = sdl_context.event_pump().expect("Errore event pump");

        (Platform { canvas }, event_pump)
    }

    pub fn draw(&mut self, display: &[bool; 64 * 32]) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..32 {
            for x in 0..64 {
                if display[y * 64 + x] {
                    let rect = Rect::new((x * 15) as i32, (y * 15) as i32, 15, 15);
                    let _ = self.canvas.fill_rect(rect);
                }
            }
        }
        self.canvas.present();
    }
    pub fn map_keycode(keycode: Keycode) -> Option<usize> {
        match keycode {
            Keycode::Kp1 => Some(0x1),
            Keycode::Kp2 => Some(0x2),
            Keycode::Kp3 => Some(0x3),
            Keycode::Kp4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }
}
