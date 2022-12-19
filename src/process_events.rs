use glam::*;
use raylib::prelude::*;

use crate::state::{Mode, State};

const JUMP_POWER: f32 = 10.0;

pub fn process_events(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    match state.mode {
        Mode::Title => process_events_title(rl, rlt, state),
        Mode::Playing => process_events_playing(rl, rlt, state),
        Mode::GameOver => process_events_game_over(rl, rlt, state),
    }
}

pub fn process_events_title(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.mode = Mode::Playing;
        state.reset();
    }
}

pub fn process_events_playing(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.player.vel.y = -JUMP_POWER;
    }
}

pub fn process_events_game_over(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.mode = Mode::Title;
    }
}
