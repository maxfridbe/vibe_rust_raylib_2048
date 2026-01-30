mod game;

use game::{Game, Direction, GRID_SIZE, TileMove};
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 700;
const GRID_PADDING: i32 = 10;
const CELL_SIZE: i32 = (WINDOW_WIDTH - (GRID_SIZE as i32 + 1) * GRID_PADDING) / GRID_SIZE as i32;
const TOP_OFFSET: i32 = 100;
const ANIMATION_DURATION: f32 = 0.15;

struct Animation {
    move_data: TileMove,
}

fn get_color(value: u32) -> Color {
    match value {
        0 => Color::new(205, 193, 180, 255),
        2 => Color::new(238, 228, 218, 255),
        4 => Color::new(237, 224, 200, 255),
        8 => Color::new(242, 177, 121, 255),
        16 => Color::new(245, 149, 99, 255),
        32 => Color::new(246, 124, 95, 255),
        64 => Color::new(246, 94, 59, 255),
        128 => Color::new(237, 207, 114, 255),
        256 => Color::new(237, 204, 97, 255),
        512 => Color::new(237, 200, 80, 255),
        1024 => Color::new(237, 197, 63, 255),
        2048 => Color::new(237, 194, 46, 255),
        _ => Color::BLACK,
    }
}

fn get_text_color(value: u32) -> Color {
    if value <= 4 {
        Color::new(119, 110, 101, 255)
    } else {
        Color::new(249, 246, 242, 255)
    }
}

fn draw_tile(d: &mut RaylibDrawHandle, value: u32, x: i32, y: i32) {
    let color = get_color(value);
    d.draw_rectangle_rounded(
        Rectangle::new(x as f32, y as f32, CELL_SIZE as f32, CELL_SIZE as f32),
        0.1,
        10,
        color
    );
    
    if value != 0 {
        let text = format!("{}", value);
        let font_size = if value < 100 { 50 } else if value < 1000 { 40 } else { 30 };
        let text_width = d.measure_text(&text, font_size);
        let text_x = x + (CELL_SIZE - text_width) / 2;
        let text_y = y + (CELL_SIZE - font_size) / 2;
        d.draw_text(&text, text_x, text_y, font_size, get_text_color(value));
    }
}

fn grid_to_screen(r: usize, c: usize) -> (i32, i32) {
    let x = (c as i32 * (CELL_SIZE + GRID_PADDING)) + GRID_PADDING;
    let y = TOP_OFFSET + (r as i32 * (CELL_SIZE + GRID_PADDING)) + GRID_PADDING;
    (x, y)
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("2048 Rust + Raylib")
        .build();

    let mut audio = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    
    let mut move_sound: Option<Sound> = match audio.new_sound("assets/move.wav") {
        Ok(s) => Some(s),
        Err(_) => None,
    };
    
    let mut merge_sound: Option<Sound> = match audio.new_sound("assets/merge.wav") {
        Ok(s) => Some(s),
        Err(_) => None,
    };
    
    let mut sound_enabled = true;

    let mut game = Game::new();
    let mut active_animations: Vec<Animation> = Vec::new();
    let mut animation_timer = 0.0;
    let mut new_tile_pos: Option<(usize, usize)> = None;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        
        // Update Animations
        if animation_timer > 0.0 {
            animation_timer -= dt;
            if animation_timer <= 0.0 {
                animation_timer = 0.0;
                active_animations.clear();
                new_tile_pos = None; // Show the new tile now
            }
        }

        // Input
        if animation_timer == 0.0 {
            if rl.is_key_pressed(KeyboardKey::KEY_R) {
                game.reset();
            }
            
            if rl.is_key_pressed(KeyboardKey::KEY_M) {
                sound_enabled = !sound_enabled;
            }

            let mut dir = None;
            if !game.game_over && !game.won {
                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A) {
                    dir = Some(Direction::Left);
                } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) || rl.is_key_pressed(KeyboardKey::KEY_D) {
                    dir = Some(Direction::Right);
                } else if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
                    dir = Some(Direction::Up);
                } else if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S) {
                    dir = Some(Direction::Down);
                }
            }

            if let Some(d) = dir {
                let result = game.move_tiles(d);
                if result.moved {
                    // Play Sounds
                    if sound_enabled {
                        if result.merged {
                            if let Some(s) = &mut merge_sound { s.play(); }
                        } else {
                            if let Some(s) = &mut move_sound { s.play(); }
                        }
                    }

                    // Setup Animations
                    active_animations.clear();
                    for m in result.moves {
                        active_animations.push(Animation { move_data: m });
                    }
                    
                    if let Some((r, c, _)) = result.new_tile {
                        new_tile_pos = Some((r, c));
                    }
                    
                    animation_timer = ANIMATION_DURATION;
                }
            }
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(250, 248, 239, 255));

        // Draw Header
        d.draw_text("2048", 20, 20, 50, Color::new(119, 110, 101, 255));
        
        let score_text = format!("Score: {}", game.score);
        d.draw_text(&score_text, 200, 30, 30, Color::new(119, 110, 101, 255));

        let high_score_text = format!("Best: {}", game.high_score);
        d.draw_text(&high_score_text, 400, 30, 30, Color::new(119, 110, 101, 255));
        
        let sound_text = if sound_enabled { "Sound: ON (M)" } else { "Sound: OFF (M)" };
        d.draw_text(sound_text, 400, 75, 20, Color::DARKGRAY);
        d.draw_text("Press 'R' to Restart", 20, 75, 20, Color::DARKGRAY);

        // Draw Grid Background
        let grid_bg_color = Color::new(187, 173, 160, 255);
        d.draw_rectangle(0, TOP_OFFSET, WINDOW_WIDTH, WINDOW_WIDTH, grid_bg_color);

        // Calculate Set of Animating Destination Tiles (to skip drawing them statically)
        let mut animating_destinations: Vec<(usize, usize)> = Vec::new();
        if animation_timer > 0.0 {
            for anim in &active_animations {
                animating_destinations.push(anim.move_data.to);
            }
            if let Some(pos) = new_tile_pos {
                animating_destinations.push(pos);
            }
        }

        // Draw Static Grid (skipping animating ones)
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let val = game.grid[r][c];
                // Draw background cell always
                let (x, y) = grid_to_screen(r, c);
                d.draw_rectangle_rounded(
                    Rectangle::new(x as f32, y as f32, CELL_SIZE as f32, CELL_SIZE as f32),
                    0.1,
                    10,
                    get_color(0) // Empty cell color
                );

                if val != 0 {
                    // Only draw tile if NOT in animating destinations
                    let mut is_animating = false;
                    for dest in &animating_destinations {
                        if dest.0 == r && dest.1 == c {
                            is_animating = true;
                            break;
                        }
                    }
                    
                    if !is_animating {
                        draw_tile(&mut d, val, x, y);
                    }
                }
            }
        }

        // Draw Animations
        if animation_timer > 0.0 {
            let t = 1.0 - (animation_timer / ANIMATION_DURATION); // 0.0 to 1.0
            // Ease out cubic
            let t_eased = 1.0 - (1.0 - t).powi(3);

            for anim in &active_animations {
                let (start_x, start_y) = grid_to_screen(anim.move_data.from.0, anim.move_data.from.1);
                let (end_x, end_y) = grid_to_screen(anim.move_data.to.0, anim.move_data.to.1);
                
                let cur_x = start_x as f32 + (end_x - start_x) as f32 * t_eased;
                let cur_y = start_y as f32 + (end_y - start_y) as f32 * t_eased;
                
                // If it's a merge, the value is the old value (before merge). 
                // The new value appears when animation finishes.
                draw_tile(&mut d, anim.move_data.value, cur_x as i32, cur_y as i32);
            }
            
            // Optional: Draw new tile scaling up?
            // For now, new tile is hidden until animation ends.
        }

        if game.game_over {
             d.draw_rectangle(0, TOP_OFFSET, WINDOW_WIDTH, WINDOW_WIDTH, Color::new(255, 255, 255, 150));
             d.draw_text("Game Over!", WINDOW_WIDTH/2 - 100, WINDOW_HEIGHT/2 - 30, 50, Color::DARKGRAY);
        } else if game.won {
             d.draw_rectangle(0, TOP_OFFSET, WINDOW_WIDTH, WINDOW_WIDTH, Color::new(255, 215, 0, 100));
             d.draw_text("You Win!", WINDOW_WIDTH/2 - 100, WINDOW_HEIGHT/2 - 30, 50, Color::WHITE);
        }
    }
}
