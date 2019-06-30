
extern crate rand;
extern crate sdl2;
#[macro_use]
extern crate human_panic;
extern crate structopt;

use rand::Rng;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, EventPump,
};
use std::{thread, time};
use structopt::StructOpt;

const MAX_X: i32 = 199;
const MAX_Y: i32 = MAX_X;
const CELL_WIDTH: i32 = 5;
const CELL_HEIGHT: i32 = CELL_WIDTH;
const NCELLS: i32 = (MAX_X + 1) / CELL_WIDTH;

// life_random
fn life_random(ncells: i32) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<Vec<bool>> = Vec::new();
    for i in 0..ncells {
        v.push(Vec::new());
        for _ in 0..ncells {
            v[i as usize].push(rng.gen());
        }
    }
    v
}

// An alternative initial state
// You can use this instead of life_random
#[allow(dead_code)]
fn glider(ncells: i32) -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::new();

    for i in 0..ncells {
        v.push(Vec::new());
        for _ in 0..ncells {
            v[i as usize].push(false);
        }
    }

    v[10][11] = true;
    v[11][12] = true;
    v[12][10] = true;
    v[12][11] = true;
    v[12][12] = true;

    v
}

// Another alternative initial state
// You can use this instead of life_random
#[allow(dead_code)]
fn infinite1(ncells: i32) -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::new();

    for i in 0..ncells {
        v.push(Vec::new());
        for _ in 0..ncells {
            v[i as usize].push(false);
        }
    }

    v[11][11] = true;
    v[11][12] = true;
    v[11][13] = true;
    v[11][15] = true;

    v[12][11] = true;

    v[13][14] = true;
    v[13][15] = true;

    v[14][12] = true;
    v[14][13] = true;
    v[14][15] = true;

    v[15][11] = true;
    v[15][13] = true;
    v[15][15] = true;
    v[15][19] = true;

    v
}

// display_cell 绘制单元
fn display_cell(r: &mut WindowCanvas, row: i32, col: i32) {
    let x: i32 = CELL_WIDTH * col;
    let y: i32 = CELL_WIDTH * row;

    let cell_color: Color = Color::RGB(255, 0, 0);
    r.set_draw_color(cell_color);
    let _ = r.fill_rect(Rect::new(x, y, CELL_WIDTH as u32, CELL_HEIGHT as u32));
}

// 帧率绘制
fn display_frame(r: &mut WindowCanvas, v: &Vec<Vec<bool>>) {
    r.set_draw_color(Color::RGB(200, 200, 200));
    r.clear();
    for i in 0..NCELLS {
        for j in 0..NCELLS {
            if v[i as usize][j as usize] {
                display_cell(r, i, j);
            }
        }
    }
    r.present();
}

fn inc(n: usize) -> usize {
    (n + 1) % (NCELLS as usize)
}

fn dec(n: usize) -> usize {
    if n == 0 {
        (NCELLS - 1) as usize
    } else {
        (n - 1) as usize
    }
}

fn count_surrounding(r: i32, c: i32, v: &Vec<Vec<bool>>) -> i32 {
    let r = r as usize;
    let c = c as usize;

    v[dec(r)][c] as i32
        + v[inc(r)][c] as i32
        + v[r][dec(c)] as i32
        + v[r][inc(c)] as i32
        + v[dec(r)][dec(c)] as i32
        + v[dec(r)][inc(c)] as i32
        + v[inc(r)][inc(c)] as i32
        + v[inc(r)][dec(c)] as i32
}

// alive 检查存活
fn alive(r: i32, c: i32, v: &Vec<Vec<bool>>) -> bool {
    let n = count_surrounding(r, c, v);
    let curr = v[r as usize][c as usize] as i32;

    match (curr, n) {
        (1, 0...1) => false,
        (1, 4...8) => false,
        (1, 2...3) => true,
        (0, 3) => true,
        (0, 0...2) => false,
        (0, 4...8) => false,
        _ => panic!("alive: error in match"),
    }
}

// 下一帧
fn life_next(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut v2: Vec<Vec<bool>> = Vec::new();

    for i in 0..NCELLS {
        v2.push(Vec::new());
        for j in 0..NCELLS {
            if alive(i, j, &v) {
                v2[i as usize].push(true);
            } else {
                v2[i as usize].push(false);
            }
        }
    }

    v2
}

// 初始化
fn init<'a>() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let mut timer = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("demo", MAX_X as u32 + 1, MAX_Y as u32 + 1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();


    // 画布
    let mut renderer = window.into_canvas().accelerated().build().unwrap();
    let event_pump: EventPump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}

fn run() {
    let (mut r, mut e) = init();
    // let mut v = life_random(NCELLS);
    let mut v = infinite1(NCELLS);

    'running: loop {
        for event in e.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        display_frame(&mut r, &v);
        v = life_next(v);
        thread::sleep(time::Duration::from_millis(50));
    }
}

// life 运行
#[derive(StructOpt, Debug)]
#[structopt(name = "life")]
struct Arg {
    #[structopt(subcommand)]
    sub_command: SubCommand,
}


#[derive(StructOpt, Debug)]
#[structopt()]
enum SubCommand {
    /// run life
    #[structopt(name = "run")]
    Run,
}

fn main() {
    setup_panic!();

    let args: Arg = Arg::from_args();

    match args.sub_command {
        SubCommand::Run => run(),
    }

}