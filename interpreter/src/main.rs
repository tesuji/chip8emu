use std::borrow::Cow;
use std::env;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};

use chip8emu::{Cpu, CpuState, KeyCode, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u16 = 20;
const DISPLAY_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH * SCALE) as u32;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT * SCALE) as u32;
const FPS: u32 = 540;
const SLEEP_DURATION: Duration = Duration::from_nanos((10_u32.pow(9) / FPS) as u64);

static IBM_LOGO: &[u8] = include_bytes!("../../IBM_Logo.ch8");

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // full screen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window("CHIP-8 interpreter", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        // .target_texture()
        // screen cannot render faster than display rate (usually 60Hz or 144Hz)
        // .present_vsync()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::BLACK);
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this every time we want to render a new frame on the window.
    canvas.present();

    let rom_path = env::args_os().nth(1);
    let bin: Cow<[u8]> = match rom_path {
        Some(path) => fs::read(path).unwrap().into(),
        None => {
            eprintln!("Opening default IBM_LOGO rom ...");
            IBM_LOGO.into()
        }
    };
    let mut cpu: Cpu = Cpu::new();
    cpu.load_game(&bin).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => {
                    break 'running
                }
                Event::KeyDown { scancode: Some(sc), repeat: false, .. } => {
                    if let Some(kc) = keymap(sc) {
                        if let CpuState::Paused = cpu.state {
                            cpu.state = CpuState::Running;
                        }
                        cpu.set_key_state(kc, true);
                    }
                }
                Event::KeyUp { scancode: Some(sc), repeat: false, .. } => {
                    if let Some(kc) = keymap(sc) {
                        // cpu.state = CpuState::Running;
                        cpu.set_key_state(kc, false);
                    }
                }
                _ => {}
            }
        }

        /* The rest of the game loop goes here... */

        let start = Instant::now();
        if let CpuState::Running = cpu.state {
            if cpu.execute_cycle() {
                draw_sprites(&mut canvas, cpu.get_vram());
            }
        }

        let elapsed = start.elapsed();
        match SLEEP_DURATION.checked_sub(elapsed) {
            Some(dur) => thread::sleep(dur),
            None => {}
        }
    }
}

fn draw_sprites(canvas: &mut Canvas<Window>, vram: &[bool; DISPLAY_SIZE]) {
    for (i, &pixel) in vram.iter().enumerate() {
        let (x, y) = (i as u16 % DISPLAY_WIDTH, i as u16 / DISPLAY_WIDTH);
        let rect = Rect::new(
            i32::from(x * SCALE),
            i32::from(y * SCALE),
            u32::from(SCALE),
            u32::from(SCALE),
        );
        let color = if pixel { Color::GREEN } else { Color::BLACK };
        canvas.set_draw_color(color);
        canvas.fill_rect(rect).unwrap();
    }
    canvas.present();
}

fn keymap(sc: Scancode) -> Option<KeyCode> {
    let kc = match sc {
        Scancode::Num1 => KeyCode::K1,
        Scancode::Num2 => KeyCode::K2,
        Scancode::Num3 => KeyCode::K3,
        Scancode::Num4 => KeyCode::KC,
        Scancode::Q => KeyCode::K4,
        Scancode::W => KeyCode::K5,
        Scancode::E => KeyCode::K6,
        Scancode::R => KeyCode::KD,
        Scancode::A => KeyCode::K7,
        Scancode::S => KeyCode::K8,
        Scancode::D => KeyCode::K9,
        Scancode::F => KeyCode::KE,
        Scancode::Z => KeyCode::KA,
        Scancode::X => KeyCode::KB,
        Scancode::C => KeyCode::KC,
        Scancode::V => KeyCode::KF,

        Scancode::Left => KeyCode::K4,
        Scancode::Up => KeyCode::K2,
        Scancode::Right => KeyCode::K6,
        Scancode::Down => KeyCode::K8,
        _ => return None,
    };
    Some(kc)
}
