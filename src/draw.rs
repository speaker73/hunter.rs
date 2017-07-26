extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point, Color};
use ::MainState;
use movemant::{Rabbit};
pub struct Txt {
    pub text_string: String,
    pub x: f32,
    pub y: f32,
}
pub struct Hex {
	pub w:f32,
	pub h:f32,
	pub x:f32,
	pub y:f32,
}

pub fn draw_text(txt: Txt, font: &graphics::Font, ctx: &mut Context) {
    let text = graphics::Text::new(ctx, &txt.text_string, &font).unwrap();
    let cord = graphics::Point::new(txt.x as f32, txt.y as f32);
    graphics::draw(ctx, &text, cord, 0.0).unwrap();
}

pub fn draw_hex(hex: Hex, ctx: &mut Context){
	let rect = graphics::Rect::new(hex.x as f32, hex.y as f32, hex.w as f32, hex.h as f32);
            graphics::polygon(
                ctx,
                DrawMode::Fill,
                &[
                    Point {
                        x: (hex.x - hex.w / 2.0),
                        y: (hex.y - hex.h / 2.0),
                    },
                    Point {
                        x: hex.x,
                        y: (hex.y - hex.h),
                    },
                    Point {
                        x: (hex.x + hex.w / 2.0),
                        y: (hex.y - hex.h / 2.0),
                    },
                    Point {
                        x: (hex.x - hex.w / 2.0),
                        y: (hex.y - hex.h / 2.0),
                    },
                ],
            ).unwrap();
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).unwrap();
            graphics::polygon(
                ctx,
                DrawMode::Fill,
                &[
                    Point {
                        x: (hex.x - hex.w / 2.0),
                        y: (hex.y + hex.h / 2.0),
                    },
                    Point {
                        x: hex.x as f32,
                        y: (hex.y + hex.h) as f32,
                    },
                    Point {
                        x: (hex.x + hex.w / 2.0),
                        y: (hex.y + hex.h / 2.0),
                    },
                    Point {
                        x: (hex.x - hex.w / 2.0),
                        y: (hex.y + hex.h / 2.0),
                    },
                ],
            ).unwrap();
}

pub fn rgba_float(r_i: u8, g_i: u8, b_i: u8, a_i: f32) -> Color {

    fn to_float(color: u8) -> f32 {
        (color as f32) / 255.0
    };

    let result = Color {
        r: to_float(r_i),
        g: to_float(g_i),
        b: to_float(b_i),
        a: a_i,
    };
    result
}

pub fn render_rabbit(point:Point, ctx: &mut Context, tself: &MainState){
     graphics::draw_ex(ctx,
                       &tself.rabbit,
                       graphics::DrawParam {
                             dest:point,
                             scale: Point{x:0.3, y:0.3},
                            ..Default::default()
                             
                       }).unwrap();
}

pub fn draw_rabbits(ctx: &mut Context, tself: &mut MainState){
    let map =  tself.map_hash.iter();
    if tself.rabbits_calc > 0 {
     for obj in map{
        if tself.rabbits_calc > 0{
             let cor = *obj.1;
             let hex_cor = obj.0;
             tself.rabbits_hash.insert( ( hex_cor.0, hex_cor.1 ), Rabbit{x:hex_cor.0, y:hex_cor.1, point:cor} );
             render_rabbit(cor, ctx, tself);
             tself.rabbits_calc -= 1;
        }       
     }        
    }else{
        for rabbit in tself.rabbits_hash.values(){
            let cor = rabbit.point;
            render_rabbit(cor, ctx, tself);
        }
    }
                 
}
