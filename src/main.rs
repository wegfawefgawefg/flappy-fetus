#![warn(clippy::all, clippy::pedantic)]

use audio::Song;
use glam::*;
use raylib::prelude::*;

use graphics::render;
use graphics::Graphics;
use process_events::process_events;
use step::step;

mod audio;
mod collisions;
mod graphics;
mod obstacle;
mod player;
mod process_events;
mod state;
mod step;

fn main() {
    let mut state = state::State::new();
    let (mut rl, mut rlt) = raylib::init().title("Flappy Fetus").build();
    let mut graphics = graphics::Graphics::new(&mut rl, &rlt);
    let mut audio = audio::Audio::new(&mut rl, &rlt);

    audio
        .rl_audio_device
        .play_music_stream(&mut audio.songs[Song::Title as usize]);

    state.running = true;
    while state.running && !rl.window_should_close() {
        process_events(&mut rl, &mut rlt, &mut state, &mut audio);
        step(&mut rl, &mut rlt, &mut state, &mut audio);
        render(&mut graphics, &mut rl, &mut rlt, &mut state);
    }
}
