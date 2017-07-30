extern crate ggez;
use ggez::*;
use ggez::graphics::{Point};

use ::MainState;
use draw::{Hex};


struct HexParam {
    w: f32, 
    h: f32,
    point:Point,
}
#[derive(Debug,Clone)]
struct Triangle {
    a:Point,
    b:Point,
    c:Point,
}

pub fn on_hex_press(x:i32, y:i32, tself: &mut MainState){
	let mut hex_param = HexParam{
		w: 136.0,
		h: 68.0,
		point:Point{x:0.0,y:0.0}
	};
	let press = Point{
		x: x as f32,
		y: y as f32,
	};
	//println!("event_rect=> a:{},{}; d:{},{}", event_rect.a.x,event_rect.a.y, event_rect.d.x, event_rect.d.y);
	let hex_map = tself.map_hash.iter();
	//let mut calc = 0;
	for hex in hex_map{
		//calc+=1;
		//println!("calc = {}", calc);
		hex_param.point = *hex.1;
		let min = Point{
			x:(hex_param.point.x - hex_param.w / 2.0),
			y:(hex_param.point.y - hex_param.h / 2.0),
		};
		
		let max = Point{
			x:(hex_param.point.x + hex_param.w / 2.0),
			y:(hex_param.point.y + hex_param.h / 2.0),
		};
		
		let top_triangle = Triangle{
			a:min,
			b:Point{
				x:hex_param.point.x,
				y:(hex_param.point.y - hex_param.h),
			},
			c:Point{
				x:max.x,
				y:min.y,
			},
		};
		let bottom_triangle = Triangle{
			a:Point{
				x:min.x,
				y:max.y,
			},
			b:Point{
				x:hex_param.point.x,
				y:(hex_param.point.y + hex_param.h),
			},
			c:max,
		};		
		//println!("hex_map=> hex_cord:{},{}; real_cord:{},{}", (hex.0).0, (hex.0).1, (hex.1).x, (hex.1).y);
		let is_pres_triangle = is_press_on_triangle(press, bottom_triangle.clone()) || is_press_on_triangle(press, top_triangle.clone());	

		let is_press_on_this_rect = (press.x < max.x) && (press.x > min.x) && (press.y < max.y) && (press.y > min.y); 
		if is_press_on_this_rect || is_pres_triangle {
			println!("____________________________________________________________________");	
			println!("press:{},{}", press.x, press.y);	
			println!("your press on rect => {},{} :: point: {}, {}", (hex.0).0, (hex.0).1, (hex.1).x, (hex.1).y );
			println!("{:?}",top_triangle);
			println!("hex_param => min:{},{}; max:{},{};", min.x, min.y, max.x, max.y);
			println!("{:?}",bottom_triangle);
			println!("____________________________________________________________________");
			let what_remove = add_or_remove( (  ( (hex.0).0, (hex.0).1 ) ), tself);
				
				if -1 == what_remove{
					tself.is_click.push( ( (hex.0).0, (hex.0).1 ));	
				}else{
					tself.is_click.remove(what_remove as usize);
				}
		}
	}
}



fn add_or_remove(event_hex:((i32, i32)), tself: & MainState) -> i32{
	println!("{:?}", tself.is_click );
	let vec_length = tself.is_click.len();
	for i in 0..vec_length {
			if tself.is_click[i as usize] == event_hex {
				return i as i32
			}
	}
	return -1	
}
fn is_press_on_triangle (press:Point, triangle:Triangle) ->bool{
	//println!("{:?}", &press);
	//println!("{:?}",&triangle);
	let is_bottom = triangle.b.y < triangle.a.y;
	let is_y = if is_bottom {
		(press.y < triangle.b.y) || (press.y > triangle.a.y)
	}else{
		(press.y > triangle.b.y) || (press.y < triangle.a.y)
	};

	if (press.x > triangle.c.x) || (press.x < triangle.a.x) || is_y {
		return false
	}
		let risn_bottom = if press.x <= triangle.b.x {
			triangle.b.y - press.y
		}else{
			press.y - triangle.b.y
		};
		let risn_top = if press.x <= triangle.b.x {
			press.y - triangle.b.y
		}else{
			triangle.b.y - press.y
		};
		let risn = if is_bottom{
			risn_bottom
		}else{
			risn_top
		};
		let k = risn / (triangle.b.x - press.x);
		let b = triangle.b.y - (triangle.b.x * k);
		let result_x = (triangle.a.y - b)/k;
		if result_x < triangle.c.x && result_x > triangle.a.x{
			println!("ok");
			return true
		}
	
	false
}
