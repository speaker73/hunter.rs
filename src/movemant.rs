extern crate ggez;
extern crate rand;
use rand::Rng;
use ggez::graphics::{Point};
use ::MainState;

#[derive(Clone)]
pub struct Rabbit{
	pub x:i32,
	pub y:i32,
	pub point:Point,
}

pub fn rabbits_run (tself:&mut MainState){
	let mut next_rabbits = Vec::new();
	for obj in tself.rabbits_hash.iter(){
		let cor_prev = obj.1;
		let rab = rabbit_run(cor_prev);
		let mut cor_next = rab.0;
		cor_next.point = match tself.map_hash.get(&(cor_next.x, cor_next.y)) {
		    Some(n) => *n,
		    None => Point{x:150.0,y:150.0},
		};
		//println!("Rabbit({},{},{}) go {}, prev:{},{}; next:{},{}",(obj.0).0,(obj.0).1,(obj.0).2, rab.1, cor_prev.x, cor_prev.y, cor_next.x, cor_next.y);
		//println!("next:{},{}", cor_next.x, cor_next.y);
		//println!("next point:{},{}", &next_point.x, &next_point.y);

		next_rabbits.push( (  *obj.0, cor_next ) );
	};
	for rabbit in next_rabbits{
		tself.rabbits_hash.remove( &rabbit.0 );
		tself.rabbits_hash.insert( ((rabbit.0).0, (rabbit.0).1, (rabbit.0).2), rabbit.1 );
	}
}
pub fn rabbit_run (cor_prev:& Rabbit)-> (Rabbit, &str){
	let not_run = Rabbit{
		x:cor_prev.x,
		y:cor_prev.y,
		point: cor_prev.point,
	};
	let left = Rabbit{
		x: (cor_prev.x - 1),
		y: cor_prev.y,
		point: cor_prev.point,
	};
	let right = Rabbit{
		x: (cor_prev.x + 1),
		y: cor_prev.y,
		point: cor_prev.point,
	};
	let up_left = Rabbit{
		x: (cor_prev.x - 1),
		y: (cor_prev.y - 1),
		point: cor_prev.point,
	};
	let down_right = Rabbit{
		x: (cor_prev.x + 1),
		y: (cor_prev.y + 1),
		point: cor_prev.point,
	};
	let false_random = rand::thread_rng().gen_range(0, 5);
	let variants = [
		(not_run, "not_run"),
		(left, "left") , 
		(right, "right"),
		(up_left, "up_left"),
		(down_right, "down_right"),
	];
	let lets_go = variants[false_random].clone();
	/*if (lets_go.0.x > 4)  ||  (lets_go.0.y < -4){
		return variants[0].clone();
	}*/
	return lets_go
}