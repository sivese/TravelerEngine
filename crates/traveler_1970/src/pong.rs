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

const PONG_WIDTH:u32= 860;
const PONG_HEIGHT:u32 = 530;

const CELL_SIZE:u32 = 10;

const TARGET_FRAME:u32 = 24;

enum Stage {
    TITLE,
    MAIN,
}

struct Screen;

impl Screen {
    const WIDTH : usize = (PONG_WIDTH / 10) as usize;
    const HEIGHT : usize = (PONG_HEIGHT / 10) as usize;
}

//
static MAP : [[u32; Screen::HEIGHT]; Screen::WIDTH] = [[1; Screen::HEIGHT]; Screen::WIDTH];

// TITLE GRAPHIC
// pong
const TITLE_DRAW : Vec<(u32, u32)> = vec![
    (0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2), (0, 3), (0, 4), //p
];

const BAR_DRAW : Vec<(u32, u32)> = vec![
    (0, 0), (0, 1), (0, 2), (0, 3)
];

//const BALL

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
        clear();

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

                    match game.stage {
                        Stage::TITLE => title_update(code),
                        Stage::MAIN => main_update(code),
                    }

                    if code == Keycode::Left { println!("Left key pressed"); }
                }
                _ => { }
            }
        }

        draw(canvas);
    }
}

fn clear() {
    
}

fn title_update(key : Keycode) {
    
}

fn main_update(key : Keycode) {
    
}

fn draw(canvas : &mut Canvas<Window>) {
    for x in 0..MAP.len() {
        for y in 0..MAP[0].len() {
            if MAP[x][y] == 0 { continue; }

            let pos_x = (x as u32) * CELL_SIZE;
            let pos_y = (y as u32) * CELL_SIZE;

            let xel = Rect::new(pos_x  as i32, pos_y as i32, CELL_SIZE, CELL_SIZE);

            _ = canvas.fill_rect(xel);
        }
    }
    
    canvas.present();
}

pub fn entry() {
    game_loop(init());
}