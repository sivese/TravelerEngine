extern crate sdl2;

#[cfg(test)]
use std::{print, println};

use log::{ debug, error, info, Level };
use sdl2::{ render::Canvas, video::Window, pixels::Color, Sdl, event::Event, keyboard::Keycode, sys::Time, TimerSubsystem };
use sdl2::messagebox::{ButtonData, MessageBoxButtonFlag, MessageBoxFlag, show_message_box};
use sdl2::sys::SDL_Rect;
use sdl2::rect::Rect;
use crate::pong::Stage::TITLE;

const ATARI_WIDTH:u32= 160;
const ATARI_HEIGHT:u32 = 192;

const PONG_WIDTH:u32= 858;
const PONG_HEIGHT:u32 = 525;

const TARGET_FRAME:u32 = 24;

enum Stage {
    TITLE,
    MAIN,
}

struct Game {
    context : Sdl,
    canvas : Canvas<Window>,
    stage : Stage,
}

struct Round {
    left_score  : u32,
    right_score : u32,
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

    let win_result = video_subsystem.window("Pong", PONG_WIDTH, PONG_HEIGHT)
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
        stage : TITLE,
    }
}

fn game_loop(mut game : Game) {
    let canvas = &mut (game.canvas);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::WHITE);

    let rect = Rect::new(100, 100, 30, 30);

    canvas.fill_rect(rect).unwrap();
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

                    let res = show_message_box(
                        MessageBoxFlag::INFORMATION,
                        vec![
                            ButtonData {
                                flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
                                button_id: 2,
                                text: "No",
                            },
                            ButtonData {
                                flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
                                button_id: 1,
                                text: "Yes",
                            },
                        ].as_slice(),
                        "PONG",
                        "Do you want to quit PONG?",
                        None,
                        None,
                    );

                    break 'run
                },
                Event::KeyDown { keycode, .. } => {
                    let code = match keycode {
                        Some(key) => key,
                        None => panic!("Failed to get key code!"),
                    };

                    if code == Keycode::Left { println!("Left key pressed"); }
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