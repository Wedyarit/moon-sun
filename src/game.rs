use ggez::{graphics, Context, GameResult, event::EventHandler};
use ggez::graphics::{Text, TextFragment, Color as GgezColor};
use nalgebra::Vector2;
use rand::Rng;

use crate::utils::count_objects;
use crate::object::{Object, Team, ObjectKind};
use crate::formatter::format_squares_numbers;

pub mod game_constants {
    pub const ROW_SIZE: i32 = 20;
    pub const COLUMN_SIZE: i32 = 10;

    pub const CIRCLE_SIZE: f32 = 25.0;
    pub const SQUARE_SIZE: f32 = 50.0;

    pub const FIELD_START_Y: f32 = 100.0;
    pub const FIELD_START_X: f32 = 128.0;

    pub const FIELD_WIDTH: f32 = (SQUARE_SIZE * 2.0) * COLUMN_SIZE as f32;
    pub const FIELD_HEIGHT: f32 = (SQUARE_SIZE) * ROW_SIZE as f32;

    pub const MOVEMENT_SPEED: f32 = 15.0;
}

pub struct Game {
    squares: Vec<Object>,
    sun_circle: Object,
    moon_circle: Object,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let mut rng = rand::thread_rng();

        let moon_x_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let moon_y_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };

        let sun_x_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let sun_y_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };

        let moon_circle = Object {
            position: (870.0, 650.0), 
            team: Team::MOON,
            kind: ObjectKind::Circle,
            direction: Vector2::new(
                moon_x_sign * 1.5 * game_constants::MOVEMENT_SPEED,
                moon_y_sign * 1.5 * game_constants::MOVEMENT_SPEED
            )
        };

        let sun_circle = Object {
            position: (385.5, 650.0),
            team: Team::SUN,
            kind: ObjectKind::Circle,
            direction: Vector2::new(
                sun_x_sign * 1.5 * game_constants::MOVEMENT_SPEED,
                sun_y_sign * 1.5 * game_constants::MOVEMENT_SPEED
            )
        };

        let mut squares = Vec::new();

        for row in 0..game_constants::ROW_SIZE {
            for column in 0..game_constants::COLUMN_SIZE {
                squares.push(Object {
                    position: (game_constants::FIELD_START_X + (column as f32) * game_constants::SQUARE_SIZE, game_constants::FIELD_START_Y + (row as f32) * game_constants::SQUARE_SIZE),
                    team: Team::MOON,
                    kind: ObjectKind::Square,
                    direction: Vector2::new(0.0, 0.0),
                });
            }
        }

        for row in 0..game_constants::ROW_SIZE {
            for column in 0..game_constants::COLUMN_SIZE {
                squares.push(Object {
                    position: (game_constants::FIELD_START_X + (column as f32) *  game_constants::SQUARE_SIZE + (game_constants::COLUMN_SIZE as f32) * game_constants::SQUARE_SIZE, game_constants::FIELD_START_Y + (row as f32) *  game_constants::SQUARE_SIZE),
                    team: Team::SUN,
                    kind: ObjectKind::Square,
                    direction: Vector2::new(0.0, 0.0),
                });
            }
        }

        match graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf") {
            Ok(font_data) => {
                ctx.gfx.add_font("LiberationMono", font_data);
            }
            Err(error) => {
                // Handle the error appropriately, e.g., printing an error message
                println!("Error loading font: {:?}", error);
            }
        }

        Game {
            squares,
            sun_circle,
            moon_circle,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult {
        self.sun_circle.update_position();
        self.moon_circle.update_position();
        self.sun_circle.handle_boundary_collision();
        self.moon_circle.handle_boundary_collision();

        for square in &mut self.squares {
            self.sun_circle.handle_collision(square);
            self.moon_circle.handle_collision(square);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas_color = GgezColor::from_rgb_u32(0x767B91);
        let mut canvas = graphics::Canvas::from_frame(ctx, canvas_color);

        let dest_point = ggez::glam::Vec2::new(428.0, 1180.0);
        let font_color = GgezColor::from_rgb_u32(0xE1E5EE); 
        let text_fragment = TextFragment::new(format_squares_numbers(count_objects(&self.squares, Team::MOON), count_objects(&self.squares, Team::SUN)))
            .font("LiberationMono")
            .scale(40.0)
            .color(font_color);
        canvas.draw(
            &Text::new(text_fragment),
            dest_point,
        );

        for square in &mut self.squares {
            square.draw(ctx, &mut canvas)?;
        }

        self.sun_circle.draw(ctx, &mut canvas)?;
        self.moon_circle.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)?;

        Ok(())
    }
}