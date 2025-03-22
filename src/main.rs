use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::time::Instant;

use rock_paper_scissor_sim::{Grid, PixelType};

extern crate rand;
extern crate sdl2;

const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 1024;

fn handle_events(event_pump: &mut sdl2::EventPump, quit: &mut bool) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *quit = true;
                break;
            }
            _ => (),
        }
    }
}

fn create_grid_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture_type: PixelType,
) -> Option<Texture<'a>> {
    let Ok(mut pixel) = texture_creator.create_texture_target(None, 1, 1) else {
        return None;
    };

    canvas
        .with_texture_canvas(&mut pixel, |texture| {
            match texture_type {
                PixelType::Paper => texture.set_draw_color(Color::RGB(255, 0, 0)),
                PixelType::Rock => texture.set_draw_color(Color::RGB(0, 255, 0)),
                PixelType::Scissors => texture.set_draw_color(Color::RGB(0, 0, 255)),
            };
            texture.clear();
        })
        .expect("Failed to color texture");

    Some(pixel)
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");

    let video_subsytem = sdl_context.video().expect("Unable to get video subsystem");

    let window = video_subsytem
        .window("RPS Simulation", WINDOW_WIDTH, WINDOW_HEIGHT)
        .vulkan()
        .build()
        .expect("Failed to build Window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .target_texture()
        .build()
        .expect("Failed to convert window into canvas");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get Event pump");
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut quit: bool = false;

    let textures = [
        create_grid_texture(&mut canvas, &texture_creator, PixelType::Rock).unwrap(),
        create_grid_texture(&mut canvas, &texture_creator, PixelType::Paper).unwrap(),
        create_grid_texture(&mut canvas, &texture_creator, PixelType::Scissors).unwrap(),
    ];

    let timer = Instant::now();
    let mut grid = Grid::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);
    let time_elapesd = timer.elapsed();
    println!("Time elapsed {:?}", time_elapesd);

    println!(
        "Grid has {} rows and {} cols",
        grid.grid.len(),
        grid.grid[0].len()
    );

    loop {
        handle_events(&mut event_pump, &mut quit);

        if quit {
            break;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let timer = Instant::now();
        for i in 0..grid.get_rows() {
            for j in 0..grid.get_cols() {
                let index: usize = match grid.grid[i][j] {
                    PixelType::Rock => 0,
                    PixelType::Paper => 1,
                    PixelType::Scissors => 2,
                };
                canvas
                    .copy(&textures[index], None, Rect::new(i as i32, j as i32, 1, 1))
                    .expect("Could not copy texture onto canvas");
            }
        }
        let duration = timer.elapsed();
        println!(
            "Latency: {:?}   FPS: {}",
            duration,
            1_000_000_000 as f32 / duration.as_nanos() as f32
        );

        canvas.present();

        grid.update_players();

        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
