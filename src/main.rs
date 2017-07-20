extern crate ggez;
extern crate rand;

use ggez::*;
//use ggez::event::*;
use ggez::graphics::{DrawMode, Point, Color};
use std::time::Duration;


struct MainState {
    pos_x: f32,
    pos_y: f32,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            pos_x: 100.0,
            pos_y: 100.0,
        }
    }
}

fn rgba_float(r_i: u8, g_i: u8, b_i: u8, a_i: f32) -> Color {

    fn to_float(color: u8) -> f32 {
        (color as f32) / 255.0
    };

    let result = Color {
        r: to_float(r_i),
        g: to_float(g_i),
        b: to_float(b_i),
        a: a_i,
    };
    // println!("u8 rgba ({},{},{},{})", r_i, g_i, b_i, a_i);
    // println!("f32 rgba ({},{},{},{})", result.r, result.g, result.b, result.a);
    return result;
}


fn map_creator(rows: i32, ctx: &mut Context) {
    println!("map_creator is start");
    graphics::set_color(ctx, rgba_float(45, 1, 1, 0.35)).unwrap();
    //let map_width = rows;
    let min_width = rows - ((rows - 1) / 2);
    //let mut start = rows;
    let mut end = min_width;
    let mut iter = 0;
    fn render_row(end: i32, ctx: &mut Context, iter: i32, min_width: i32, rows: i32, invert: bool) {

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18).unwrap();
        let w = 136;
        let h = 68;
        let padding_left = 2;
        let padding_top = -2;
        for i in 0..end {

            let mut x = 310 - ((end - min_width) * 4) + (i * (w + padding_left)) -
                ((end - min_width) * (w / 2));
            let y = 110 + (iter * (h + h / 2 + padding_top));

            let mut znak: i32 = 1;
            if invert == false {
                x = 310 - (iter * 4) + (i * (w + padding_left)) - ((end - min_width) * (w / 2));
                znak = -1;
            }

            let mut x_str: i32 = 0;

            if invert == false {
                x_str = i - (end - min_width);
            } else {
                x_str = i - (rows - min_width);
            }
            let y_str = (rows - end) * znak;
            let mut xy_str = format!("{},{}", x_str, y_str);
            let score_text = graphics::Text::new(ctx, "score", &font).unwrap();
            let xy_text = graphics::Text::new(ctx, &xy_str, &font).unwrap();
            let score_dest = graphics::Point::new(x as f32, y as f32);
            graphics::draw(ctx, &xy_text, score_dest, 0.0).unwrap();
            let rect = graphics::Rect::new(x as f32, y as f32, w as f32, h as f32);

            graphics::set_color(ctx, rgba_float(45, 1, 1, 0.35)).unwrap();

            graphics::polygon(
                ctx,
                DrawMode::Fill,
                &[
                    Point {
                        x: (x - w / 2) as f32,
                        y: (y - h / 2) as f32,
                    },
                    Point {
                        x: x as f32,
                        y: (y - h) as f32,
                    },
                    Point {
                        x: (x + w / 2) as f32,
                        y: (y - h / 2) as f32,
                    },
                    Point {
                        x: (x - w / 2) as f32,
                        y: (y - h / 2) as f32,
                    },
                ],
            ).unwrap();

            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).unwrap();

            graphics::polygon(
                ctx,
                DrawMode::Fill,
                &[
                    Point {
                        x: (x - w / 2) as f32,
                        y: (y + h / 2) as f32,
                    },
                    Point {
                        x: x as f32,
                        y: (y + h) as f32,
                    },
                    Point {
                        x: (x + w / 2) as f32,
                        y: (y + h / 2) as f32,
                    },
                    Point {
                        x: (x - w / 2) as f32,
                        y: (y + h / 2) as f32,
                    },
                ],
            ).unwrap();

            //graphics::set_color(ctx, rgba_float(255, 1, 1, 1.0)).unwrap();
            // graphics::circle(
            //     ctx,
            //     DrawMode::Fill,
            //     Point {
            //         x: (x - 30) as f32,
            //         y: (y - 25) as f32,
            //     },
            //     2.0,
            //     100,
            // ).unwrap();
        }
    }
    let mut invert: bool = false;
    loop {
        if (end < rows) && !invert {
            render_row(end, ctx, iter, min_width, rows, invert);
            end = end + 1;
            iter = iter + 1;
        };
        if end == rows {
            render_row(end, ctx, iter, min_width, rows, invert);
            invert = true;
            end = end - 1;
            iter = iter + 1;
        };
        if invert && (min_width != end) {
            render_row(end, ctx, iter, min_width, rows, invert);
            end = end - 1;
            iter = iter + 1;
        };
        if min_width == end {
            render_row(end, ctx, iter, min_width, rows, invert);
            // iter = iter + 1;
            break;
        };

    }

    println!("map_creator is end");
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        /* let y_plus = self.pos_y + 100.0;
         let x_plus = self.pos_x + 100.0;
         let y_minus = self.pos_y - 100.0;
         let x_minus = self.pos_x - 100.0;*/
        graphics::clear(ctx);
        graphics::set_background_color(
            ctx,
            //rgba_float(255, 192, 82, 1.0)
            rgba_float(255, 130, 20, 1.0),
        );
        /* graphics::circle(ctx,
                         DrawMode::Fill,
                         Point {
                             x: self.pos_x,
                             y: self.pos_y,
                         },
                         20.0,
                         32)?;*/





        map_creator(7, ctx);
        graphics::present(ctx);
        Ok(())
    }

    /*
    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        println!(
            "Mouse button pressed: {:?}, x: {}, y: {}, pos_x: {}",
            button,
            x,
            y,
            self.pos_x
        );
        self.pos_x = x as f32;
        self.pos_y = y as f32;
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self, _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x,
            y,
            xrel,
            yrel
        );

    }

    fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
    }


    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("Key pressed: {:?}, modifier {:?}, repeat: {}",
                 keycode,
                 keymod,
                 repeat);
    }
    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("Key released: {:?}, modifier {:?}, repeat: {}",
                 keycode,
                 keymod,
                 repeat);
    }

    fn controller_button_down_event(&mut self, btn: Button, instance_id: i32) {
        println!("Controller button pressed: {:?} Controller_Id: {}",
                 btn,
                 instance_id);
    }

    fn controller_button_up_event(&mut self, btn: Button, instance_id: i32) {
        println!("Controller button released: {:?} Controller_Id: {}",
                 btn,
                 instance_id);
    }

    fn controller_axis_event(&mut self, axis: Axis, value: i16, instance_id: i32) {
        println!("Axis Event: {:?} Value: {} Controller_Id: {}",
                 axis,
                 value,
                 instance_id);
    }


    fn focus_event(&mut self, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }*/
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
    let state = &mut MainState::new();

    event::run(ctx, state).unwrap();

}
