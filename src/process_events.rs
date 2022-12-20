use glam::*;
use raylib::prelude::*;

use crate::{
    audio::{Audio, Song},
    state::{Mode, State},
};

const JUMP_POWER: f32 = 10.0;

pub fn process_events(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    match state.mode {
        Mode::Title => process_events_title(rl, rlt, state, audio),
        Mode::Playing => process_events_playing(rl, rlt, state, audio),
        Mode::GameOver => process_events_game_over(rl, rlt, state, audio),
    }
}

pub fn process_events_title(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.mode = Mode::Playing;

        let current_song = &mut audio.songs[Song::Playing as usize];
        audio.rl_audio_device.stop_music_stream(current_song);
        audio.rl_audio_device.play_music_stream(current_song);
        state.reset();
    }
}

pub fn process_events_playing(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.player.vel.y = -JUMP_POWER;
    }
}

pub fn process_events_game_over(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.mode = Mode::Title;
        let current_song = &mut audio.songs[Song::Title as usize];
        audio.rl_audio_device.stop_music_stream(current_song);
        audio.rl_audio_device.play_music_stream(current_song);
    }
}
