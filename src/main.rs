use macroquad::prelude::*;

#[macroquad::main("sand")]
async fn main() {
	loop {
		clear_background(BLACK);

		// stuff here

		next_frame().await;
	}
}
