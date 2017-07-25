use draw::{Txt, draw_text, Hex, draw_hex, rgba_float};
use ggez::*;
use ggez::graphics::{DrawMode, Point, Color, Drawable};
use ::MainState;


pub struct HexCord {
    pub x:i32,
    pub y:i32,
}

pub fn map_creator(rows: i32, ctx: &mut Context, tself: &mut MainState) {
    //println!("map_creator is start");
    graphics::set_color(ctx, rgba_float(45, 1, 1, 0.35)).unwrap();
    //let map_width = rows;
    let min_width = rows - ((rows - 1) / 2);
    //let mut start = rows;
    let mut end = min_width;
    let mut iter = 0;
    let mut il = 0;
    
    let mut invert: bool = false;
    loop {
        if (end < rows) && !invert {
            create_row(end, ctx, iter, min_width, rows, invert, tself, il);
            end += 1;
            iter += 1;
        };
        if end == rows {
            create_row(end, ctx, iter, min_width, rows, invert, tself, il);
            invert = true;
            end -= 1;
            iter += 1;
        };
        if invert && (min_width != end) {
            create_row(end, ctx, iter, min_width, rows, invert, tself, il);
            end -= 1;
            iter += 1;
        };
        if min_width == end {
            create_row(end, ctx, iter, min_width, rows, invert, tself, il);
            iter += 1;
            break;
        };

    }
    if tself.iteration != -1 {
    	println!("map_creator is end:{}", tself.iteration);
    	tself.iteration = -1;
    }
    il = 0;
    
}

fn create_row(
        end: i32,
        ctx: &mut Context,
        iter: i32,
        min_width: i32,
        rows: i32,
        invert: bool,
        tself: &mut MainState,
        mut il: i32,
    ) {
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
            let hex_cor = HexCord{
            	x: x_str,
            	y: y_str,
            };
            let xy_str = format!("{},{}", x_str, y_str);
            if tself.iteration == -1 {
            	//println!("il {}", il);
            	let texts = tself.texts.get( &(hex_cor.x, hex_cor.y) ).unwrap();
            	let cord = graphics::Point::new(x as f32, y as f32);
    			graphics::draw(ctx, texts, cord, 0.0).unwrap();
            }
            /*draw_text(
                Txt {
                    textString: xy_str,
                    x: x as f32,
                    y: y as f32,
                },
                &tself.font,
                ctx,
            );*/
            
            
            draw_hex(
                Hex{
                    x: x as f32,
                    y: y as f32,
                    w: w as f32,
                    h: h as f32,
                }, 
                ctx
            );
            if tself.iteration != -1 {
				let text_glif = graphics::Text::new(ctx, &xy_str, &tself.font).unwrap();
            	tself.texts.insert( ( hex_cor.x, hex_cor.y ), text_glif);
            	tself.iteration += 1;
            	il += 1;            	
            }else{
            	il += 1;  
            }
            
        }
    }
