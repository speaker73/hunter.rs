extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point, Color};
use ::MainState;

#[derive(Clone)]
pub struct Rabbit{
	pub x:i32,
	pub y:i32,
}

pub fn rabbits_run (tself:&mut MainState){
	for obj in tself.rabbits_hash.iter(){
		let cor_prev = *obj.0;
		let cor_next = rabbit_run(cor_prev.0,cor_prev.1);
		//let next_point = tself.map_hash.get( &(cor_next.x, cor_next.y) ).unwrap();
		println!("prev:{},{}", cor_prev.0, cor_prev.1);
		println!("next:{},{}", cor_next.x, cor_next.y);
		//tself.rabbits_hash.insert( ( cor_next.x, cor_next.y ), *next_point);
	}
}
pub fn rabbit_run (x:i32,y:i32)-> Rabbit {
	let left = Rabbit{
		x: (x - 1),
		y: y,
	};
	let right = Rabbit{
		x: (x + 1),
		y: y,
	};
	let up_left = Rabbit{
		x: (x - 1),
		y: (y - 1),
	};
	let down_right = Rabbit{
		x: (x + 1),
		y: (y + 1),
	};
	let falseRandom = 0;
	let variants = [
		left, 
		right,
		up_left,
		down_right,
	];
	return variants[falseRandom].clone()
}