extern crate ggez;
extern crate rand;
use std::collections::HashMap;
use ggez::*;
use ggez::event::*;
use ggez::graphics::{Point};
use std::time::Duration;
mod draw;
use draw::{rgba_float, draw_rabbits};
mod map;
use map::{map_creator};
mod movemant;
use movemant::{rabbit_run,rabbits_run, Rabbit};

pub struct MainState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub iteration: i32,
    pub font: graphics::Font,
    pub texts: HashMap<(i32, i32), graphics::Text>,
    pub rabbit: graphics::Image,
    pub rabbits_calc:i32,
    pub rabbits_hash:HashMap<(i32, i32), Point>,
    pub map_hash: HashMap<(i32, i32), Point>,
}



impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        MainState {
            pos_x: 100.0,
            pos_y: 100.0,
            iteration: 0,
            font: graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18).unwrap(),
            texts: HashMap::new(),
            rabbit: graphics::Image::new(ctx, "/rabbit.png").unwrap(),
            rabbits_calc: 5,
            rabbits_hash: HashMap::new(),
            map_hash: HashMap::new(),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx);

        graphics::set_background_color(ctx, rgba_float(255, 130, 20, 1.0));
        graphics::set_color(ctx, rgba_float(45, 1, 1, 0.35)).unwrap();
        map_creator(7, ctx, self);
        
        draw_rabbits(ctx, self);

        graphics::present(ctx);

        Ok(())
    }
    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
       //println!("key_down: {}", keycode);
    }


    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        
        if Keycode::R == keycode{
            println!("key_up: {}", keycode);
            rabbits_run(self);
            //self.rabbits_calc +=50;
        }
    }
}

fn main() {
    println!("Hunter is start");

    let mut c = conf::Conf::new();
    c.window_title = "Hunter!".to_string();
    c.window_width = 1024;
    c.window_height = 800;

    //c.resizable = true;
    c.window_icon = "/player.png".to_string();
    let ctx = &mut Context::load_from_conf("astroblasto", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx);
    event::run(ctx, state).unwrap();

}
