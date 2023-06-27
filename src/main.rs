use chip8_core::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::fs::File;
use std::io::Read;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

//

type Context = Sdl;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let gamepath = &args[1];

    assert!(args.len() == 2);
    if args.len() != 2 {
        println!("Usage: cargo run <path_to_game>");
        return;
    }

    let (mut canvas, sdl_context) = canvas_context();

    canvas.clear();
    canvas.present();

    let mut chip8 = Emulator::new();
    init_event_pump(&mut chip8, &mut canvas, sdl_context);

    let mut rom = File::open(gamepath).expect("Failed to open game file!");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);
}

fn canvas_context() -> (Canvas<Window>, Context) {
    let context = sdl2::init().unwrap();
    let subsystem = context.video().unwrap();
    let window = subsystem
        .window("CHIP: Ate.", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let canvas = window.into_canvas().present_vsync().build().unwrap();

    (canvas, context)
}

fn init_event_pump(emulator: &mut Emulator, canvas: &mut Canvas<Window>, sdl_context: Sdl) {
    use sdl2::event::Event;

    let mut pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }

        emulator.tick();
        draw_screen(emulator, canvas);
    }
}

fn draw_screen(emulator: &Emulator, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = emulator.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
