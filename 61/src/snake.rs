#![allow(static_mut_refs)]

use alloc::collections::VecDeque;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const INTERIOR_ROWS: usize = HEIGHT - 2;
const INTERIOR_COLS: usize = WIDTH - 2;
const INTERIOR_CELLS: usize = INTERIOR_ROWS * INTERIOR_COLS;

const MMIO: usize = 0xb8000;
const COLOR: u8 = 0x0f;

const TOP_LEFT: u8 = 0xDA;
const TOP_RIGHT: u8 = 0xBF;
const BOTTOM_LEFT: u8 = 0xC0;
const BOTTOM_RIGHT: u8 = 0xD9;
const HORIZONTAL: u8 = 0xC4;
const VERTICAL: u8 = 0xB3;

const SNAKE: u8 = 0xDB;
const FOOD: u8 = 0xA2;
const EMPTY: u8 = 0x20;

const TICK_DIVIDER: usize = 8;
const MSG_COLOR: u8 = 0x4f;

type Pos = [usize; 2];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct GameState {
    snake: VecDeque<Pos>,
    dir: Direction,
    food: Pos,
    seed: usize,
    ticks: usize,
    ready: bool,
    game_over: bool,
}

static mut GAME: Option<GameState> = None;

pub fn init() {
    draw_border();
    clear_interior();

    let mut snake = VecDeque::with_capacity(INTERIOR_CELLS);
    snake.push_back([12, 38]);
    snake.push_back([12, 39]);
    snake.push_back([12, 40]);

    let mut game = GameState {
        snake,
        dir: Direction::Right,
        food: [5, 20],
        seed: 1,
        ticks: 0,
        ready: false,
        game_over: false,
    };

    draw_snake(&game);
    if !place_food(&mut game) {
        win(&mut game);
    }
    draw_food(&game);
    game.ready = true;

    unsafe {
        GAME = Some(game);
    }
}

pub fn on_key(scancode: u8) {
    if (scancode & 0x80) != 0 {
        return;
    }

    unsafe {
        let Some(game) = GAME.as_mut() else {
            return;
        };

        game.ticks = game.ticks.wrapping_add(1);
        game.seed = game
            .seed
            .wrapping_mul(1664525)
            .wrapping_add(scancode as usize);

        match scancode {
            0x11 => {
                if game.dir != Direction::Down {
                    game.dir = Direction::Up;
                }
            }
            0x1E => {
                if game.dir != Direction::Right {
                    game.dir = Direction::Left;
                }
            }
            0x1F => {
                if game.dir != Direction::Up {
                    game.dir = Direction::Down;
                }
            }
            0x20 => {
                if game.dir != Direction::Left {
                    game.dir = Direction::Right;
                }
            }
            0x10 => crate::qemu_quit(crate::QEMU_PASS),
            _ => {}
        }
    }
}

pub fn on_tick() {
    unsafe {
        let Some(game) = GAME.as_mut() else {
            return;
        };
        if !game.ready || game.game_over {
            return;
        }

        game.ticks = game.ticks.wrapping_add(1);
        if (game.ticks % TICK_DIVIDER) != 0 {
            return;
        }

        step(game);
    }
}

fn step(game: &mut GameState) {
    let Some(head) = game.snake.back().copied() else {
        lose(game);
    };
    let Some(tail) = game.snake.front().copied() else {
        lose(game);
    };

    let next = next_head(head, game.dir);

    if is_border(next) {
        lose(game);
    }

    let growing = next == game.food;
    if contains_snake(game, next) && !(next == tail && !growing) {
        lose(game);
    }

    game.snake.push_back(next);
    draw_cell(next[0], next[1], SNAKE, COLOR);

    if growing {
        game.seed = game
            .seed
            .wrapping_mul(1103515245)
            .wrapping_add(12345)
            .wrapping_add(game.ticks);
        if !place_food(game) {
            win(game);
        }
        draw_food(game);
    } else if let Some(old_tail) = game.snake.pop_front() {
        draw_cell(old_tail[0], old_tail[1], EMPTY, COLOR);
    } else {
        lose(game);
    }
}

fn place_food(game: &mut GameState) -> bool {
    let start = game.seed % INTERIOR_CELLS;
    for off in 0..INTERIOR_CELLS {
        let idx = (start + off) % INTERIOR_CELLS;
        let row = 1 + idx / INTERIOR_COLS;
        let col = 1 + idx % INTERIOR_COLS;
        let candidate = [row, col];

        if !contains_snake(game, candidate) {
            game.food = candidate;
            return true;
        }
    }

    false
}

fn contains_snake(game: &GameState, pos: Pos) -> bool {
    game.snake.iter().any(|p| *p == pos)
}

fn is_border(pos: Pos) -> bool {
    pos[0] == 0 || pos[0] == HEIGHT - 1 || pos[1] == 0 || pos[1] == WIDTH - 1
}

fn next_head(head: Pos, dir: Direction) -> Pos {
    match dir {
        Direction::Up => [head[0].wrapping_sub(1), head[1]],
        Direction::Down => [head[0] + 1, head[1]],
        Direction::Left => [head[0], head[1].wrapping_sub(1)],
        Direction::Right => [head[0], head[1] + 1],
    }
}

fn draw_food(game: &GameState) {
    draw_cell(game.food[0], game.food[1], FOOD, COLOR);
}

fn draw_snake(game: &GameState) {
    for pos in game.snake.iter() {
        draw_cell(pos[0], pos[1], SNAKE, COLOR);
    }
}

fn clear_interior() {
    for row in 1..(HEIGHT - 1) {
        for col in 1..(WIDTH - 1) {
            draw_cell(row, col, EMPTY, COLOR);
        }
    }
}

fn draw_border() {
    draw_cell(0, 0, TOP_LEFT, COLOR);
    draw_cell(0, WIDTH - 1, TOP_RIGHT, COLOR);
    draw_cell(HEIGHT - 1, 0, BOTTOM_LEFT, COLOR);
    draw_cell(HEIGHT - 1, WIDTH - 1, BOTTOM_RIGHT, COLOR);

    for col in 1..(WIDTH - 1) {
        draw_cell(0, col, HORIZONTAL, COLOR);
        draw_cell(HEIGHT - 1, col, HORIZONTAL, COLOR);
    }

    for row in 1..(HEIGHT - 1) {
        draw_cell(row, 0, VERTICAL, COLOR);
        draw_cell(row, WIDTH - 1, VERTICAL, COLOR);
    }
}

fn draw_cell(row: usize, col: usize, ch: u8, color: u8) {
    let idx = row * WIDTH + col;
    let ptr = (MMIO + idx * 2) as *mut u8;
    unsafe {
        ptr.write_volatile(ch);
        ptr.add(1).write_volatile(color);
    }
}

fn write_text(row: usize, col: usize, text: &[u8], color: u8) {
    for (i, b) in text.iter().enumerate() {
        draw_cell(row, col + i, *b, color);
    }
}

fn lose(game: &mut GameState) -> ! {
    game.game_over = true;
    write_text(HEIGHT / 2, (WIDTH / 2) - 5, b"GAME OVER", MSG_COLOR);
    crate::qemu_quit(crate::QEMU_FAIL);
    crate::halt();
}

fn win(game: &mut GameState) -> ! {
    game.game_over = true;
    write_text(HEIGHT / 2, (WIDTH / 2) - 3, b"YOU WIN", MSG_COLOR);
    crate::qemu_quit(crate::QEMU_PASS);
    crate::halt();
}
