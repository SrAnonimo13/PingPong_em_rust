extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use std::vec::*;

const COLOR: [f32; 4] = [1.0; 4];
const WIN_SIZE: [u32; 2] = [640, 480];

const BAR_SPEED: f64 = BOLL_SPEED;
const BAR_MARGIN: f64 = 10.0;
const START_BAR_POSS: f64 = (WIN_SIZE[1] / 2) as f64 - BAR_SIZE[1] / 2 as f64;
const BAR_SIZE: [f64; 2] = [15.0, 80.0];
const BOLL_SIZE: i8 = 25;
const BOLL_SPEED: f64 = 0.5;
const TEXT_MARGIN: f64 = 30.0;

struct Game {
    bar_one_poss: Vec<f64>,
    bar_two_poss: Vec<f64>,
    boll_poss: Vec<f64>,
    boll_vel: Vec<f64>,
    bar_one_vel: f64,
    bar_two_vel: f64,
}

impl Game {
    fn new() -> Self {
        Game {
            bar_one_poss: vec![BAR_MARGIN, START_BAR_POSS],
            bar_two_poss: vec![
                WIN_SIZE[0] as f64 - BAR_SIZE[0] - BAR_MARGIN,
                START_BAR_POSS,
            ],
            bar_one_vel: 0.0,
            bar_two_vel: 0.0,
            boll_poss: vec![
                (WIN_SIZE[0] / 2) as f64 - (BOLL_SIZE / 2) as f64,
                (WIN_SIZE[1] / 2) as f64 - (BOLL_SIZE / 2) as f64,
            ],
            boll_vel: vec![-BOLL_SPEED, -BOLL_SPEED],
        }
    }

    fn get_bar_one_params(&self) -> [f64; 4] {
        [
            self.bar_one_poss[0],
            self.bar_one_poss[1],
            BAR_SIZE[0] as f64,
            BAR_SIZE[1] as f64,
        ]
    }

    fn get_bar_two_params(&self) -> [f64; 4] {
        [
            self.bar_two_poss[0],
            self.bar_two_poss[1],
            BAR_SIZE[0] as f64,
            BAR_SIZE[1] as f64,
        ]
    }

    fn get_ball_params(&self) -> [f64; 4] {
        [
            self.boll_poss[0],
            self.boll_poss[1],
            BOLL_SIZE as f64,
            BOLL_SIZE as f64,
        ]
    }
}

fn main() {
    let mut window = create_window();
    let mut game = Game::new();
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut font = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    let mut points = [0, 0];

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            //Primeiro Jogador
            rectangle(
                COLOR,
                game.get_bar_one_params(),
                context.transform,
                graphics,
            );
            //Segundo Jogador
            rectangle(
                COLOR,
                game.get_bar_two_params(),
                context.transform,
                graphics,
            );
            //Bola
            rectangle(COLOR, game.get_ball_params(), context.transform, graphics);
        
            text::Text::new_color(COLOR, 30).draw(
                points[1].to_string().as_str(),
                &mut font,
                &context.draw_state,
                context.transform.trans(TEXT_MARGIN, TEXT_MARGIN),
                graphics
            ).unwrap();

            text::Text::new_color(COLOR, 30).draw(
                points[0].to_string().as_str(),
                &mut font,
                &context.draw_state,
                context.transform.trans(WIN_SIZE[0] as f64 - TEXT_MARGIN - 30.0, TEXT_MARGIN),
                graphics
            ).unwrap();

            font.factory.encoder.flush(device);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => game.bar_one_vel = -BAR_SPEED,
                Key::S => game.bar_one_vel = BAR_SPEED,
                Key::Up => game.bar_two_vel = -BAR_SPEED,
                Key::Down => game.bar_two_vel = BAR_SPEED,
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W => game.bar_one_vel = 0.0,
                Key::S => game.bar_one_vel = 0.0,
                Key::Up => game.bar_two_vel = 0.0,
                Key::Down => game.bar_two_vel = 0.0,
                _ => {}
            }
        }

        if game.boll_poss[0] <= game.bar_one_poss[0] + BAR_SIZE[0] &&
           game.boll_poss[1] <= game.bar_one_poss[1] + BAR_SIZE[1] &&
           game.boll_poss[1] >= game.bar_one_poss[1]
        {
            game.boll_vel[0] = BOLL_SPEED;
        }

        if game.boll_poss[0] >= game.bar_two_poss[0] - BAR_SIZE[0] &&
           game.boll_poss[1] + BOLL_SIZE as f64 >= game.bar_two_poss[1] &&
           game.boll_poss[1] <= game.bar_two_poss[1] + BAR_SIZE[1]
        {
            game.boll_vel[0] = -BOLL_SPEED;
        }

        if game.boll_poss[1] <= 0.0 {
            game.boll_vel[1] = BOLL_SPEED;
        }

        if game.boll_poss[1] >= (WIN_SIZE[1] - BOLL_SIZE as u32) as f64 {
            game.boll_vel[1] = -BOLL_SPEED;
        }

        if game.boll_poss[0] <= -BOLL_SIZE as f64 {
            points[0] += 1;
            reset(&mut game);
        }

        if game.boll_poss[0] >= WIN_SIZE[0] as f64 {
            points[1] += 1;
            reset(&mut game);
        }

        game.boll_poss[0] += game.boll_vel[0];
        game.boll_poss[1] += game.boll_vel[1];

        game.bar_one_poss[1] += game.bar_one_vel as f64;
        game.bar_two_poss[1] += game.bar_two_vel as f64;
    }
}

fn reset(game: &mut Game){
    game.boll_poss[0] = (WIN_SIZE[0] / 2) as f64 - (BOLL_SIZE / 2) as f64;
    game.boll_poss[1] = (WIN_SIZE[1] / 2) as f64 - (BOLL_SIZE / 2) as f64;
    game.boll_vel[0] *= -1.0;
}

fn create_window() -> PistonWindow {
    WindowSettings::new("Ping Pong", WIN_SIZE)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap()
}