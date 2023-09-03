use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = CHIP8_WIDTH as u32 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = CHIP8_HEIGHT as u32 * SCALE_FACTOR;

pub struct Display {
    canvas: Canvas<Window>, // the actually drawn window
    representation: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT] // the internal machine's represantation of above,
}

impl Display {
    pub fn initialize(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "CHIP-8 Display",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        return Display { canvas: canvas, representation: [[false; CHIP8_WIDTH]; CHIP8_HEIGHT] }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
        self.representation = [[false; CHIP8_WIDTH]; CHIP8_HEIGHT];
    }

    pub fn draw(&mut self, flips: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT]) -> bool {
        let mut carry: bool = false;
        for (y, row) in flips.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let canvas_x = x as u32 * SCALE_FACTOR;
                let canvas_y = y as u32 * SCALE_FACTOR;

                self.representation[y][x] ^= col;
                if !carry && col && !self.representation[y][x] {
                    carry = true;
                }

                self.canvas.set_draw_color(color(self.representation[y][x]));
                let _ = self.canvas
                    .fill_rect(Rect::new(canvas_x as i32, canvas_y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present(); // might need to move this to its own function that fires in the loop

        return carry;
    }
}

fn color(value: bool) -> pixels::Color {
    if value {
        pixels::Color::RGB(0, 250, 0)
    } else {
        pixels::Color::RGB(0, 0, 0)
    }
}