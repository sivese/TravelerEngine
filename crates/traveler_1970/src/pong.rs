extern crate sdl2;

use log::{ debug, error, info, Level };

const ATARI_WIDTH:u32= 160;
const ATARI_HEIGHT:u32 = 192;

fn init() {
    env_logger::init();

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

    let mut canvas_result = window.into_canvas()
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

    
}

fn game_loop() {

}

fn clear() {

}

pub fn entry() {
    init();
    game_loop();
    clear();
}