// I wrote this in 3 hours, this is not an example of code that I write in a professional enivronment

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
fn main() {
    let (mut rl, thread) = raylib::init().size(500, 500).title("Snektris").build();
    let mut move_timer = 0;
    let mut tetris_mode = false;
    let mut cached_x_pos: Vec<i32> = Vec::new();
    let mut cached_y_pos: Vec<i32> = Vec::new();

    struct Snake {
        snake_dir: u8,
        pos_x: i32,
        pos_y: i32,
        snake_size: i32,
        prev_x_positions: Vec<i32>,
        prev_y_positions: Vec<i32>
    }

    let mut player = Snake{snake_dir: 4, pos_x: 250, pos_y: 50, snake_size: 15, prev_x_positions: [].to_vec(), prev_y_positions: [].to_vec()};
    player.prev_x_positions.push(player.pos_x);
    player.prev_y_positions.push(player.pos_y);

    rl.set_target_fps(60);


    while !rl.window_should_close() {
        let mut canvas = rl.begin_drawing(&thread);
        canvas.clear_background(Color::BLACK);

        if canvas.is_key_pressed(KEY_UP) {
            player.snake_dir = 0;
            move_timer = 25;
        } else if canvas.is_key_pressed(KEY_DOWN) {
            player.snake_dir = 1;
            move_timer = 25;
        } else if canvas.is_key_pressed(KEY_LEFT) {
            player.snake_dir = 2;
            move_timer = 25;
        } else if canvas.is_key_pressed(KEY_RIGHT) {
            player.snake_dir = 3;
            move_timer = 25;
        } else if canvas.is_key_pressed(KEY_SPACE) {
            tetris_mode = true;
        }

        if move_timer >= 20 {
            if tetris_mode == false {
                match player.snake_dir {
                    0 => player.pos_y = player.pos_y -player.snake_size,
                    1 => player.pos_y = player.pos_y +player.snake_size,
                    2 => player.pos_x = player.pos_x -player.snake_size,
                    3 => player.pos_x = player.pos_x +player.snake_size,
                    _=> print!(""),
                }
                if player.pos_x >= 300 || player.pos_x <= 180 || player.pos_y >= 100 || player.pos_y <= 0 {
                    println!("GAME OVER");
                    player = Snake{snake_dir: 4, pos_x: 250, pos_y: 50, snake_size: 15, prev_x_positions: [].to_vec(), prev_y_positions: [].to_vec()};
                    cached_x_pos = Vec::new();
                    cached_y_pos = Vec::new();
                }
                player.prev_x_positions.push(player.pos_x);
                player.prev_y_positions.push(player.pos_y);
                if player.prev_x_positions.len() > 4 {
                    player.prev_x_positions.remove(0);
                    player.prev_y_positions.remove(0);
                }
            } else {
                let mut demo_y: Vec<i32> = Vec::new();
                demo_y.clone_from(&mut player.prev_y_positions);
                for index in 0..player.prev_y_positions.len() as usize {
                    demo_y[index] = demo_y[index] + player.snake_size;
                }
                for index in 0..player.prev_y_positions.len() as usize {
                    if tetris_mode == false {
                        break;
                    }
                    if demo_y[index] >= 450 {
                        cached_x_pos.append(&mut player.prev_x_positions);
                        cached_y_pos.append(&mut player.prev_y_positions);
                        player = Snake{snake_dir: 4, pos_x: 250, pos_y: 50, snake_size: 15, prev_x_positions: [].to_vec(), prev_y_positions: [].to_vec()};
                        tetris_mode = false;
                        break;
                    } else {
                        for index_2 in 0..cached_y_pos.len() as usize {
                            if demo_y[index] == cached_y_pos[index_2] && player.prev_x_positions[index] == cached_x_pos[index_2]{                             
                                cached_x_pos.append(&mut player.prev_x_positions);
                                cached_y_pos.append(&mut player.prev_y_positions);
                                player = Snake{snake_dir: 4, pos_x: 250, pos_y: 50, snake_size: 15, prev_x_positions: [].to_vec(), prev_y_positions: [].to_vec()};
                                tetris_mode = false; 
                                break;
                            }
                        }
                    }
                }
                if tetris_mode == true {
                    player.prev_y_positions.clone_from(&mut demo_y);
                    demo_y = Vec::new();
                }
                let mut removed_rows: Vec<i32> = Vec::new();
                for index in 0..12 {
                    let mut row_num: Vec<usize> = Vec::new();
                    for index_2 in 0..cached_y_pos.len() as usize {
                        if ((cached_y_pos[index_2]/15)-29)*-1 == index {
                            row_num.push(index_2);
                        }
                    }
                    if row_num.len() >= 8 {
                        removed_rows.push(index);
                        let mut runs = 0;
                        for index_3 in 0..8 as usize {
                            cached_x_pos.remove(row_num[index_3]-runs);
                            cached_y_pos.remove(row_num[index_3]-runs);
                            runs += 1;
                        }
                    }
                }
                for index in 0..removed_rows.len() as usize {
                    for index_2 in 0..cached_y_pos.len() as usize {
                        if ((cached_y_pos[index_2]/15)-29)*-1 >= removed_rows[index] {
                            cached_y_pos[index_2] += 15;
                        }
                    }
                }
            }
            move_timer = 0;
        }

        
        canvas.draw_text(&canvas.get_fps().to_string(), 10, 10, 16, Color::GREEN);
        if tetris_mode == false {
            move_timer = move_timer +1;   
        } else if tetris_mode == true {
            move_timer = move_timer +5;
        }
        for index in 0..player.prev_x_positions.len() as usize {canvas.draw_rectangle(player.prev_x_positions[index], player.prev_y_positions[index], player.snake_size, player.snake_size, Color::GREEN);}
        for index in 0..cached_x_pos.len() as usize {canvas.draw_rectangle(cached_x_pos[index], cached_y_pos[index], player.snake_size, player.snake_size, Color::GRAY);}
        canvas.draw_rectangle(180, 0, 4, 100, Color::DARKGRAY);
        canvas.draw_rectangle(315, 0, 4, 100, Color::DARKGRAY);
        canvas.draw_rectangle(180, 100, 139, 4, Color::DARKGRAY);
        canvas.draw_rectangle(180, 0, 139, 4, Color::DARKGRAY);
    }
}