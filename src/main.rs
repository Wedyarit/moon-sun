mod game;
mod object;
mod utils;
mod formatter;

use ggez::{conf, ContextBuilder};
use std::{env, path};

use game::Game;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let result = ContextBuilder::new("Moon & Sun", "wedyarit")
            .window_mode(conf::WindowMode::default().dimensions(1280.0, 1280.0))
            .add_resource_path(resource_dir)
            .window_setup(conf::WindowSetup::default().title("Moon & Sun").icon("/icon.png"))
            .build();
    let (mut ctx, event_loop) = match result {
        Ok((ctx, event_loop)) => (ctx, event_loop),
        Err(e) => {
            eprintln!("Error building context: {:?}", e);
            return;
        }
    };
    
    let game = Game::new(&mut ctx);

    ggez::event::run(ctx, event_loop, game)
}