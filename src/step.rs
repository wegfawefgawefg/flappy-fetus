use glam::*;
use rand::seq::SliceRandom;
use raylib::prelude::*;

pub const FRAMES_PER_SECOND: u32 = 60;
pub const TIMESTEP: f32 = 1.0 / FRAMES_PER_SECOND as f32;
pub const GRAVITY: f32 = 0.5;

pub const SPACE_RADIUS: i32 = 400;
pub const CEILING_POS: i32 = -SPACE_RADIUS;
pub const FLOOR_POS: i32 = SPACE_RADIUS;

pub const SCIZORS_AHEAD_SPAWN_DISTANCE: i32 = 800;

use crate::{
    audio::{Audio, Song, SoundEffect},
    collisions::{is_intersection, Bounded},
    obstacle::Obstacle,
    player::Player,
    state::{Mode, State},
};

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State, audio: &mut Audio) {
    let dt = rl.get_frame_time();
    state.time_since_last_update += dt;
    while state.time_since_last_update > TIMESTEP {
        match state.mode {
            Mode::Title => step_title(rl, rlt, state, audio),
            Mode::Playing => step_playing(rl, rlt, state, audio),
            Mode::GameOver => step_game_over(rl, rlt, state, audio),
        }
        state.time_since_last_update -= TIMESTEP;
    }
}

pub fn step_title(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    audio
        .rl_audio_device
        .update_music_stream(&mut audio.songs[Song::Title as usize]);
}

pub fn step_playing(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    audio
        .rl_audio_device
        .update_music_stream(&mut audio.songs[Song::Playing as usize]);

    let player = &mut state.player;

    player.vel.y += GRAVITY;
    player.pos += player.vel.as_ivec2();
    player.vel.x += 0.02;

    for obstacle in state.obstacles.iter_mut() {
        obstacle.pos += obstacle.vel.as_ivec2();
    }

    state.play_time += TIMESTEP;

    // bounce on the bottom or top
    if (player.pos.y + Player::SIZE.y as i32) >= FLOOR_POS as i32 {
        if player.vel.y > 0.0 {
            player.vel.y *= -1.0;
            audio.play_random_smack_sound();
        }
    }
    if player.pos.y < CEILING_POS {
        if player.vel.y <= 0.0 {
            player.vel.y *= -1.0;
            audio.play_random_smack_sound();
        }
    }
    // let current_song = &mut audio.songs[Song::GameOver as usize];
    // audio.rl_audio_device.stop_music_stream(current_song);
    // audio.rl_audio_device.play_music_stream(current_song);
    // state.mode = Mode::GameOver;

    // step obstacle timer
    // if obstacle timer is done
    // spawn obstacle at random height 200 units right of the player
    // reset obstacle timer

    let mut scissors_speed = player.vel.x;
    if state.play_time < 10.0 {
        state.obstacle_spawn_period_in_frames = State::STARTING_OBSTACLE_SPAWN_FRAME_PERIOD;
        scissors_speed = player.vel.x;
    } else if state.play_time > 11.5 {
        state.obstacle_spawn_period_in_frames = State::STARTING_OBSTACLE_SPAWN_FRAME_PERIOD / 2;
        scissors_speed = player.vel.x;
        // scissors_speed = player.vel.x - 2.0 + 5.0 - (rand::random::<i32>() % 10) as f32;
    } else if state.play_time > 30.0 {
        state.obstacle_spawn_period_in_frames = State::STARTING_OBSTACLE_SPAWN_FRAME_PERIOD / 3;
        scissors_speed = player.vel.x - 1.0;
        // scissors_speed = player.vel.x - 2.0 + 5.0 - (rand::random::<i32>() % 10) as f32;
    }

    state.obstacle_spawn_frame_countdown_timer -= 1;
    state.obstacle_spawn_frame_countdown_timer = state.obstacle_spawn_frame_countdown_timer.max(0);
    if state.obstacle_spawn_frame_countdown_timer <= 0 {
        // let new_obst_y = rand::random::<i32>() % (FLOOR_POS - CEILING_POS);
        let new_obst_y = rand::random::<i32>() % (SPACE_RADIUS * 1);
        state.obstacles.push(Obstacle::new(
            IVec2 {
                x: player.pos.x + SCIZORS_AHEAD_SPAWN_DISTANCE,
                y: new_obst_y,
            },
            UVec2 { x: 20, y: 20 },
            Vec2 {
                x: scissors_speed, // + 10.0 + (rand::random::<i32>() % 10) as f32,
                y: 0.0,
            },
        ));
        state.obstacle_spawn_frame_countdown_timer = state.obstacle_spawn_period_in_frames;
    }

    state.score += 0.1;
    state
        .obstacles
        .retain(|obstacle| !should_remove_obstacle(obstacle, player));

    // if player collides with obstacle
    // game over
    for obstacle in &state.obstacles {
        let player_bounds = player.get_bounds();
        let obstacle_bounds = obstacle.get_bounds();
        if is_intersection(&player_bounds, &obstacle_bounds) {
            state.mode = Mode::GameOver;
            let current_song = &mut audio.songs[Song::GameOver as usize];
            audio.rl_audio_device.stop_music_stream(current_song);
            audio.rl_audio_device.play_music_stream(current_song);
            audio.play_random_end_scream_sound();
        }
    }
}

pub fn should_remove_obstacle(obstacle: &Obstacle, player: &Player) -> bool {
    obstacle.pos.x < player.pos.x - SCIZORS_AHEAD_SPAWN_DISTANCE
}

pub fn step_game_over(
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
    audio: &mut Audio,
) {
    audio
        .rl_audio_device
        .update_music_stream(&mut audio.songs[Song::GameOver as usize]);
}
