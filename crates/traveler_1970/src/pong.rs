extern crate sdl2;

#[cfg(test)]
use std::{print, println};

use log::{ debug, error, info, Level };
use sdl2::{ render::Canvas, video::Window, pixels::Color, Sdl, event::Event, keyboard::Keycode, sys::Time, TimerSubsystem };

const ATARI_WIDTH:u32= 160;
const ATARI_HEIGHT:u32 = 192;

const TARGET_FRAME:u32 = 24;

struct Game {
    context : Sdl,
    canvas : Canvas<Window>,
}

fn init() -> Game {
    //env_logger::init();

    let context = match sdl2::init() {
        Ok(ctx) => {
            debug!("Init SDL context succeed.");
            ctx
        },
        Err(err) => {
            panic!("{}", err);
        }
    };

    let video_subsystem = match context.video() {
        Ok(video) => {
            debug!("Init video subsystem");
            video
        },
        Err(err) => {
            panic!("{}", err);
        }
    };

    let win_result = video_subsystem.window("Pong", ATARI_WIDTH * 3, ATARI_HEIGHT * 3)
        .position_centered()
        .opengl()
        .build();

    let window = match win_result {
        Ok(win) => {
            debug!("SDL window created");
            win
        },
        Err(err) => {
            panic!("{}", err);
        }
    };

    let canvas_result = window.into_canvas()
        .build();

    let canvas = match canvas_result {
        Ok(can) => {
            debug!("Windos canvas created!");
            can
        },
        Err(err) => {
            panic!("{}", err);
        }
    };

    Game {
        context : context,
        canvas : canvas,
    }
}

fn game_loop(mut game : Game) {
    let canvas = &mut (game.canvas);

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let ep_result = game.context
        .event_pump();

    let mut event_pump = match ep_result {
        Ok(ev) => ev,
        Err(err) => panic!("{}", err),
    };

    'run: loop {
        for ev in event_pump.poll_iter() {
            match ev {
                Event::Quit { timestamp } => { 
                    println!("timestamp -> {}", timestamp);
                    break 'run 
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {

                }
                _ => { }
            }
        }

        canvas.present();
    }
}

fn clear() {

}

pub fn entry() {
    game_loop(init());
    clear();
}