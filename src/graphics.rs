use glam::*;
use raylib::prelude::*;

use crate::{state::State, step};

pub enum Textures {
    Fetus,
    Scizors,
    Title,
    GameOver,
    Background,
    WombWall,
    Backdrop,
}

pub struct Graphics {
    pub window_dims: glam::UVec2,
    pub dims: glam::UVec2,
    pub camera: Camera2D,
    pub textures: Vec<Texture2D>,
}

impl Graphics {
    pub fn new(rl: &mut RaylibHandle, rlt: &RaylibThread) -> Self {
        let window_dims = UVec2::new(1280, 720);
        let dims = UVec2::new(1280, 720);

        rl.set_window_size(dims.x as i32, dims.y as i32);
        // fullscreen
        // rl.toggle_fullscreen();

        // after resetting the window size, we should recenter it or its gonna be in a weird place on the screen
        let screen_dims = UVec2::new(rl.get_screen_width() as u32, rl.get_screen_height() as u32);
        let screen_center = screen_dims / 2;
        let window_center = dims / 2;
        let offset = window_center - screen_center;
        rl.set_window_position(offset.x as i32, offset.y as i32);
        rl.set_target_fps(144);

        let mouse_scale = dims / window_dims;
        rl.set_mouse_scale(mouse_scale.x as f32, mouse_scale.y as f32);

        // let frame_buffer = match rl.load_render_texture(rlt, dims.x, dims.y) {
        //     Ok(rt) => rt,
        //     Err(e) => {
        //         println!("Error loading render texture: {}", e);
        //         std::process::exit(1);
        //     }
        // };

        // load in the textures
        let texture_error = "Error loading texture";
        let mut textures = Vec::new();
        let texture_names = vec![
            "fetus",
            "scizors",
            "title",
            "game_over",
            "background",
            "wombwall",
            "backdrop",
        ];
        for name in texture_names {
            let path = format!("assets/{}.png", name);
            let texture = rl.load_texture(rlt, &path).expect(texture_error);
            textures.push(texture);
        }

        let screen_center = (window_dims / 2).as_vec2();
        let camera = Camera2D {
            target: raylib::math::Vector2::new(0.0, 0.0), // doesnt matter because were gonna move this every frame
            offset: raylib::math::Vector2::new(screen_center.x, screen_center.y), // makes what camera targets in the middle of the screen
            rotation: 0.0,
            zoom: 1.0,
        };

        Self {
            window_dims,
            dims,
            camera,
            textures,
        }
    }
}

pub fn render(
    graphics: &mut Graphics,
    rl: &mut RaylibHandle,
    rlt: &mut RaylibThread,
    state: &mut State,
) {
    let mut screen = rl.begin_drawing(rlt);
    screen.clear_background(Color::BLACK);

    let zoom_speed = 0.4;
    let mouse_wheel = screen.get_mouse_wheel_move();
    graphics.camera.zoom += mouse_wheel.floor() as f32 * zoom_speed;

    // camera offset should be half the screen
    let screen_center = (graphics.dims / 2).as_vec2();

    match state.mode {
        crate::state::Mode::Title => render_title(&mut screen, graphics, state),
        crate::state::Mode::Playing => render_playing(&mut screen, graphics, state),
        crate::state::Mode::GameOver => render_game_over(&mut screen, graphics, state),
    }

    match state.mode {
        crate::state::Mode::Playing => {
            // render_playing_debug_info(&mut screen, graphics, state);
            render_score(&mut screen, graphics, state);
        }
        _ => {}
    }
}

pub fn render_title(screen: &mut RaylibDrawHandle, graphics: &mut Graphics, state: &mut State) {
    let screen_center = (graphics.dims / 2).as_vec2();
    graphics.camera.zoom = 1.4;
    graphics.camera.rotation = 0.0;
    graphics.camera.target = raylib::math::Vector2::new(screen_center.x, screen_center.y);
    graphics.camera.offset = raylib::math::Vector2::new(screen_center.x, screen_center.y);
    {
        let mut d = screen.begin_mode2D(graphics.camera);
        {
            let color = raylib::color::Color::new(38, 43, 68, 255);
            d.clear_background(color);
            // match
            // screen center
            let screen_center = (graphics.dims / 2).as_vec2();
            graphics.camera.target = raylib::math::Vector2::new(screen_center.x, screen_center.y);

            let texture = &graphics.textures[Textures::Title as usize];
            d.draw_texture_pro(
                &texture,
                Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
                Rectangle::new(0.0, 0.0, graphics.dims.x as f32, graphics.dims.y as f32),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            let title_position = screen_center.as_ivec2();
            let title = "Flappy Fetus";
            let font_size = 100;
            let title_size =
                raylib::text::measure_text_ex(d.get_font_default(), title, font_size as f32, 0.0);
            let text_center = title_size / 2.0;

            d.draw_text(
                title,
                title_position.x - text_center.x as i32,
                title_position.y - text_center.y as i32,
                font_size,
                Color::WHITE,
            );

            // a bit under that draw "press [space]"
            let press_space = "Press [space]";
            let press_space_size =
                raylib::text::measure_text_ex(d.get_font_default(), press_space, 30.0, 0.0);
            let press_space_center = press_space_size / 2.0;
            let press_space_pos = Vec2::new(
                text_center.x - press_space_center.x + 100.0,
                text_center.y + press_space_center.y + 100.0,
            );
            d.draw_text(
                press_space,
                press_space_pos.x as i32,
                press_space_pos.y as i32,
                30,
                Color::WHITE,
            );
        }
    }
}

pub fn render_flesh_tunnel(
    d: &mut RaylibMode2D<RaylibDrawHandle>,
    graphics: &mut Graphics,
    state: &mut State,
) {
    // example how to render a tunnel
    // let tunnel_pos = IVec2::new(-64, step::CEILING_POS);
    let tunnel_width = 512 * 4;
    // render_flesh_tunnel_segment(d, graphics, state, tunnel_pos, tunnel_width);

    /*  we are going to render always 2 of them.
        use modulus and the player position to get the position of the first and second tunnel

    */
    let first_tunnel_x =
        state.player.pos.x - (state.player.pos.x % tunnel_width) - tunnel_width / 2;
    let first_tunnel_pos = IVec2::new(first_tunnel_x, step::CEILING_POS);
    render_flesh_tunnel_segment(d, graphics, state, first_tunnel_pos, tunnel_width);

    let second_tunnel_x = first_tunnel_x + tunnel_width;
    let second_tunnel_pos = IVec2::new(second_tunnel_x, step::CEILING_POS);
    render_flesh_tunnel_segment(d, graphics, state, second_tunnel_pos, tunnel_width);
}

pub fn render_flesh_tunnel_segment(
    d: &mut RaylibMode2D<RaylibDrawHandle>,
    graphics: &mut Graphics,
    state: &mut State,
    pos: IVec2,
    width: i32,
) {
    // we are going to render the background around the player
    let background_pos = pos;
    let background_size = IVec2::new(width, step::FLOOR_POS - step::CEILING_POS) * IVec2::new(1, 1);
    let background_texture = &graphics.textures[Textures::Background as usize];
    d.draw_texture_pro(
        &graphics.textures[Textures::Background as usize],
        Rectangle::new(
            0.0,
            0.0,
            background_texture.width() as f32,
            background_texture.height() as f32,
        ),
        Rectangle::new(
            background_pos.x as f32,
            background_pos.y as f32,
            background_size.x as f32,
            background_size.y as f32,
        ),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );

    // draw a womb wall facing down from the top, about half of its height above the ceiling
    let womb_wall_texture = &graphics.textures[Textures::WombWall as usize];
    let womb_wall_size = IVec2::new(background_size.x, womb_wall_texture.height() as i32 * 2);

    let upper_womb_wall_pos = IVec2::new(background_pos.x, step::CEILING_POS as i32);
    d.draw_texture_pro(
        womb_wall_texture,
        Rectangle::new(
            0.0,
            0.0,
            -womb_wall_texture.width() as f32,
            -womb_wall_texture.height() as f32,
        ),
        Rectangle::new(
            upper_womb_wall_pos.x as f32,
            upper_womb_wall_pos.y as f32,
            womb_wall_size.x as f32,
            womb_wall_size.y as f32,
        ),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );

    // above the upper womb wall, put a backdrop
    let backdrop_texture = &graphics.textures[Textures::Backdrop as usize];
    let backdrop_size = IVec2::new(background_size.x, backdrop_texture.height() as i32 * 2);
    let backdrop_pos = IVec2::new(background_pos.x, upper_womb_wall_pos.y - backdrop_size.y);
    d.draw_texture_pro(
        backdrop_texture,
        Rectangle::new(
            0.0,
            0.0,
            backdrop_texture.width() as f32,
            backdrop_texture.height() as f32,
        ),
        Rectangle::new(
            backdrop_pos.x as f32,
            backdrop_pos.y as f32,
            backdrop_size.x as f32,
            backdrop_size.y as f32,
        ),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );

    // now for the lower womb wall
    let lower_womb_wall_pos = IVec2::new(background_pos.x, step::FLOOR_POS as i32);
    d.draw_texture_pro(
        womb_wall_texture,
        Rectangle::new(
            0.0,
            0.0,
            womb_wall_texture.width() as f32,
            womb_wall_texture.height() as f32,
        ),
        Rectangle::new(
            lower_womb_wall_pos.x as f32,
            lower_womb_wall_pos.y as f32 - womb_wall_size.y as f32,
            womb_wall_size.x as f32,
            womb_wall_size.y as f32,
        ),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );

    // below the lower womb wall, put a backdrop
    let backdrop_pos = IVec2::new(background_pos.x, lower_womb_wall_pos.y);
    d.draw_texture_pro(
        backdrop_texture,
        Rectangle::new(
            0.0,
            0.0,
            backdrop_texture.width() as f32,
            backdrop_texture.height() as f32,
        ),
        Rectangle::new(
            backdrop_pos.x as f32,
            backdrop_pos.y as f32,
            backdrop_size.x as f32,
            backdrop_size.y as f32,
        ),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}

pub fn render_playing(screen: &mut RaylibDrawHandle, graphics: &mut Graphics, state: &mut State) {
    graphics.camera.zoom = 1.4 + state.player.vel.y * 0.015;
    let screen_center = (graphics.dims / 2).as_vec2();
    graphics.camera.target =
        raylib::math::Vector2::new(state.player.pos.x as f32, state.player.pos.y as f32);
    graphics.camera.offset = raylib::math::Vector2::new(screen_center.x, screen_center.y);
    graphics.camera.rotation = state.player.vel.y * 0.3;
    {
        let mut d = screen.begin_mode2D(graphics.camera);
        {
            render_flesh_tunnel(&mut d, graphics, state);
            state.player.render(&mut d, graphics);
            for obstacle in &mut state.obstacles {
                obstacle.render(&mut d, graphics);
            }
        }
    }
}

pub fn render_score(screen: &mut RaylibDrawHandle, graphics: &mut Graphics, state: &mut State) {
    // bottom left corner render score as red text
    let score_text = format!("{}", state.score as i32);
    // use                 raylib::text::measure_text_ex(d.get_font_default(), title, font_size as f32, 0.0);
    let score_text_size =
        raylib::text::measure_text_ex(screen.get_font_default(), &score_text, 20.0, 0.0);
    let score_text_pos = Vec2::new(10.0, graphics.dims.y as f32 - score_text_size.y - 10.0);
    screen.draw_text(
        &score_text,
        score_text_pos.x as i32,
        score_text_pos.y as i32,
        20,
        Color::new(255, 0, 0, 255),
    );
}

pub fn render_game_over(screen: &mut RaylibDrawHandle, graphics: &mut Graphics, state: &mut State) {
    let screen_center = (graphics.dims / 2).as_vec2();
    graphics.camera.target = raylib::math::Vector2::new(screen_center.x, screen_center.y);
    graphics.camera.offset = raylib::math::Vector2::new(screen_center.x, screen_center.y);
    graphics.camera.rotation = 0.0;
    graphics.camera.zoom = 1.3;
    {
        let mut d = screen.begin_mode2D(graphics.camera);
        {
            let screen_center = (graphics.dims / 2).as_vec2();
            graphics.camera.target = raylib::math::Vector2::new(screen_center.x, screen_center.y);

            let texture = &graphics.textures[Textures::GameOver as usize];
            d.draw_texture_pro(
                &texture,
                Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
                Rectangle::new(0.0, 0.0, graphics.dims.x as f32, graphics.dims.y as f32),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            // set the camera to the center of the screen
            let screen_center = (graphics.dims / 2).as_vec2();
            // draw the title
            let title = "Aborted";
            let font_size = 100;
            let title_size =
                raylib::text::measure_text_ex(d.get_font_default(), title, font_size as f32, 0.0);
            let text_center = title_size / 2.0;

            d.draw_text(
                title,
                screen_center.x as i32,
                screen_center.y as i32,
                font_size,
                Color::WHITE,
            );

            // a bit under that draw "press [space]"
            let press_space = "Press [space] to be born again";
            let press_space_size =
                raylib::text::measure_text_ex(d.get_font_default(), press_space, 30.0, 0.0);
            let press_space_center = press_space_size / 2.0;
            let press_space_pos =
                Vec2::new(text_center.x, text_center.y + press_space_center.y + 100.0);
            // transparency should fade in and out
            let a = ((d.get_time() * 1.0) % 2.0) as u8 * 255;
            d.draw_text(
                press_space,
                press_space_pos.x as i32,
                press_space_pos.y as i32,
                30,
                Color::new(255, 255, 255, a),
            );
        }
    }
}

pub fn render_playing_debug_info(
    screen: &mut RaylibDrawHandle,
    graphics: &mut Graphics,
    state: &mut State,
) {
    // put cam at 0,0
    let mut print_height = 0;
    let mouse_pos = screen.get_mouse_position();
    screen.draw_circle_v(mouse_pos, 20.0, Color::RED);

    // print mouse pos
    let mouse_pos = screen.get_mouse_position();
    let mp = Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32);
    screen.draw_text(
        &format!("Mouse Pos: ({}, {})", mouse_pos.x, mouse_pos.y),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    // print out camera target
    screen.draw_text(
        &format!(
            "Camera Target: ({}, {})",
            graphics.camera.target.x, graphics.camera.target.y
        ),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    // print out camera offset
    screen.draw_text(
        &format!(
            "Camera Offset: ({}, {})",
            graphics.camera.offset.x, graphics.camera.offset.y
        ),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    // print cam zoom
    screen.draw_text(
        &format!("Camera Zoom: {}", graphics.camera.zoom),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    let mouse_room_frac = mp / graphics.window_dims.as_vec2();
    screen.draw_text(
        &format!(
            "Mouse Room Frac: ({}, {})",
            mouse_room_frac.x, mouse_room_frac.y
        ),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    // print the player velocity
    screen.draw_text(
        &format!(
            "Player Velocity: ({}, {})",
            state.player.vel.x, state.player.vel.y
        ),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    // print the player position
    screen.draw_text(
        &format!(
            "Player Position: ({}, {})",
            state.player.pos.x, state.player.pos.y
        ),
        0,
        print_height,
        20,
        Color::WHITE,
    );
    print_height += 20;

    let d_bottom = graphics.window_dims.y - 80;
    // print character hanging
    screen.draw_text(
        &format!("ctrls: space to flap. dont get abortescreen. "),
        0,
        d_bottom as i32,
        20,
        Color::BLACK,
    );
}
