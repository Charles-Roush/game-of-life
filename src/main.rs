use macroquad::prelude::*;

const BOARD_SIZE: usize = 100;
const NEIGHBORS_TO_LIVE: [u8; 2] = [2, 3];
const NEIGHBORS_TO_COME_ALIVE: [u8; 1] = [3];
const CHECK_DISTANCE: u8 = 1;
const UPDATE_INTERVAL: f32 = 0.2; // seconds between updates
// needs 2-3 neigbors to survive

fn get_mouse_box(
    mouse_pos: (f32, f32),
    square_width: f32,
    board_size: usize,
    start_x: f32,
    end_x: f32,
    start_y: f32,
    end_y: f32,
) -> Option<(usize, usize)> {
    let (mouse_x, mouse_y) = mouse_pos;

    if mouse_x < start_x || mouse_x > end_x {
        return None;
    }

    if mouse_y < start_y || mouse_y > end_y {
        return None;
    }

    let x = ((mouse_x - start_x) / square_width).floor();
    let y = ((mouse_y - start_y) / square_width).floor();

    let x = x as usize;
    let y = y as usize;

    if x >= board_size || y >= board_size {
        return None;
    }

    Some((x, y))
}
fn get_neighbors(
    board: &[[bool; BOARD_SIZE]; BOARD_SIZE],
    x: usize,
    y: usize,
    check_distance: usize,
) -> u8 {
    let x = x as isize;
    let y = y as isize;
    let check_distance = check_distance as isize;

    let start_x = x - check_distance;
    let end_x = x + check_distance;

    let start_y = y - check_distance;
    let end_y = y + check_distance;

    let mut neighbors = 0;

    for row in start_y..=end_y {
        if row < 0 || row >= BOARD_SIZE as isize {
            continue;
        }

        for col in start_x..=end_x {
            if col < 0 || col >= BOARD_SIZE as isize {
                continue;
            }
            if row == y && col == x {
                continue;
            }
            if board[row as usize][col as usize] {
                neighbors += 1
            }
        }
    }
    neighbors
}
fn update_board(board: [[bool; BOARD_SIZE]; BOARD_SIZE]) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    let mut new_board = board;
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let neighbors = get_neighbors(&board, x, y, CHECK_DISTANCE as usize);
            if *cell && !NEIGHBORS_TO_LIVE.contains(&neighbors) {
                new_board[y][x] = false;
            } else if !*cell && NEIGHBORS_TO_COME_ALIVE.contains(&neighbors) {
                new_board[y][x] = true;
            }
        }
    }
    new_board
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut mouse_pos;
    let mut board = [[false; BOARD_SIZE]; BOARD_SIZE];
    let board_size = BOARD_SIZE;
    let mut updating_life = false;
    let mut update_timer = 0.0;

    loop {
        // Input
        mouse_pos = mouse_position();
        let mut mouse_clicked = false;
        let mut space_clicked = false;

        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_clicked = true;
        }
        if is_key_pressed(KeyCode::Space) {
            space_clicked = true;
        }

        // Update
        let grid_size = screen_height().min(screen_width());
        let square_width = grid_size / BOARD_SIZE as f32;

        let start_x = (screen_width() - grid_size) / 2.0;
        let start_y = (screen_height() - grid_size) / 2.0;

        let end_x = start_x + grid_size;
        let end_y = start_y + grid_size;

        if mouse_clicked {
            let box_index = get_mouse_box(
                mouse_pos,
                square_width,
                board_size,
                start_x,
                end_x,
                start_y,
                end_y,
            );
            if let Some((x, y)) = box_index {
                println!("Clicked cell: {}, {}", x, y);
                board[y][x] = !board[y][x];
            }
        }

        if space_clicked {
            updating_life = !updating_life;
            println!("updating life = {updating_life}");
        }

        if updating_life {
            update_timer += get_frame_time();
            if update_timer >= UPDATE_INTERVAL {
                board = update_board(board);
                update_timer -= UPDATE_INTERVAL;
            }
        } else {
            update_timer = 0.0;
        }
        // Draw

        clear_background(BLACK);
        for (y_index, row) in board.iter().enumerate() {
            let y_pos = start_y + square_width * y_index as f32;
            for (x_index, square) in row.iter().enumerate() {
                let x_pos = start_x + square_width * x_index as f32;
                if *square {
                    draw_rectangle(x_pos, y_pos, square_width, square_width, WHITE);
                }
            }
        }

        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
