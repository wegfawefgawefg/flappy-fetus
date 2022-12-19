use glam::*;
use raylib::prelude::*;

pub const FRAMES_PER_SECOND: u32 = 60;
pub const TIMESTEP: f32 = 1.0 / FRAMES_PER_SECOND as f32;
pub const GRAVITY: f32 = 0.5;

pub const SPACE_RADIUS: i32 = 400;
pub const CEILING_POS: i32 = -SPACE_RADIUS;
pub const FLOOR_POS: i32 = SPACE_RADIUS;

pub const SCIZORS_AHEAD_SPAWN_DISTANCE: i32 = 400;

use crate::{
    collisions::{is_intersection, Bounded},
    obstacle::Obstacle,
    player::Player,
    state::{Mode, State},
};

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    let dt = rl.get_frame_time();
    state.time_since_last_update += dt;
    while state.time_since_last_update > TIMESTEP {
        match state.mode {
            Mode::Title => step_title(rl, rlt, state),
            Mode::Playing => step_playing(rl, rlt, state),
            Mode::GameOver => step_game_over(rl, rlt, state),
        }
        state.time_since_last_update -= TIMESTEP;
    }
}

pub fn step_title(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}

pub fn step_playing(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    let player = &mut state.player;

    player.vel.y += GRAVITY;
    player.pos += player.vel.as_ivec2();
    player.vel.x += 0.01;

    if (player.pos.y + Player::SIZE.y as i32) > FLOOR_POS as i32 || player.pos.y < CEILING_POS {
        state.mode = Mode::GameOver;
    }

    // step obstacle timer
    // if obstacle timer is done
    // spawn obstacle at random height 200 units right of the player
    // reset obstacle timer

    state.obstacle_spawn_frame_countdown_timer -= 1;
    if state.obstacle_spawn_frame_countdown_timer <= 0 {
        state.obstacles.push(Obstacle::new(
            IVec2 {
                x: player.pos.x + SCIZORS_AHEAD_SPAWN_DISTANCE,
                y: rand::random::<i32>() % (FLOOR_POS - CEILING_POS) + CEILING_POS,
            },
            UVec2 { x: 20, y: 20 },
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
        }
    }
}

pub fn should_remove_obstacle(obstacle: &Obstacle, player: &Player) -> bool {
    obstacle.pos.x < player.pos.x - SCIZORS_AHEAD_SPAWN_DISTANCE
}

pub fn step_game_over(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}
