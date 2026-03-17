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
}
