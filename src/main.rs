extern crate ggez;
extern crate rand;
use std::collections::HashMap;
use ggez::*;
use ggez::event::*;
use ggez::graphics::{Point};
use std::time::Duration;
mod draw;
use draw::{rgba_float, draw_rabbits, draw_hunter};
mod map;
use map::{map_creator};
mod movemant;
use movemant::{rabbit_run,rabbits_run, Rabbit};
mod events;
use events::{on_hex_press};
pub struct MainState {
    pub w: i32,
    pub h: i32,
    pub iteration: i32,
    pub font: graphics::Font,
    pub texts: HashMap<(i32, i32), graphics::Text>,
    pub rabbit: graphics::Image,
    pub hunter: Hunter,
    pub rabbits_calc:i32,
    pub rabbits_hash:HashMap<(i32, i32, i32), Rabbit>,
    pub map_hash: HashMap<(i32, i32), Point>,
    pub is_click: Vec<(i32, i32)>,
}


pub struct Hunter {
    hex_cor: Vec<(i32, i32)>,
    view: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        MainState {
            w: 136,
            h: 68,
            iteration: 0,
            font: graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18).unwrap(),
            texts: HashMap::new(),
            rabbit: graphics::Image::new(ctx, "/rabbit.png").unwrap(),
            hunter: Hunter{
                hex_cor:Vec::new(),
                view: graphics::Image::new(ctx, "/mini-hunter.png").unwrap(),
            },
            rabbits_calc: 3,
            rabbits_hash: HashMap::new(),
            map_hash: HashMap::new(),
            is_click: Vec::new(),
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
        graphics::set_color(ctx, rgba_float(45, 1, 1, 0.35)).unwrap();
        draw_rabbits(ctx, self);
        draw_hunter(ctx, self);
        //rabbits_run(self);
        graphics::present(ctx);

        Ok(())
    }
    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
       //println!("key_down: {}", keycode);
       if Keycode::R == keycode{
            println!("key_up: {}", keycode);
            rabbits_run(self);
            //self.rabbits_calc +=50;
        }
    }


    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        
       /* if Keycode::R == keycode{
            println!("key_up: {}", keycode);
            rabbits_run(self);
            //self.rabbits_calc +=50;
        }*/
    }
     fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        on_hex_press(x, y, self);
    }

    /*fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }
*/
   /* fn mouse_motion_event(&mut self, _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        println!("Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
                 x,
                 y,
                 xrel,
                 yrel);
    }*/

    /*fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
    }*/
}

fn main() {
    println!("Hunter is start");

    let mut c = conf::Conf::new();
    c.window_title = "Hunter!".to_string();
    c.window_width = 1680;
    c.window_height = 800;

    //c.resizable = true;
    c.window_icon = "/player.png".to_string();
    let ctx = &mut Context::load_from_conf("Hunter", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx);
    event::run(ctx, state).unwrap();

}

