use std::fmt::Alignment;

use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult, event::EventHandler};
use ggez::graphics::{Color as GgezColor, Drawable, Text, TextFragment};
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

    pub const FIELD_WIDTH: f32 = (SQUARE_SIZE * 2.0) * COLUMN_SIZE as f32;
    pub const FIELD_HEIGHT: f32 = (SQUARE_SIZE) * ROW_SIZE as f32;

    pub const MOVEMENT_SPEED: f32 = 15.0;
}


pub struct Game {
    window_width: f32,
    window_height: f32,
    scale_factor: f32,
    bounds: (f32, f32),

    squares: Vec<Object>,
    sun_circle: Object,
    moon_circle: Object,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let (window_width, window_height) = ctx.gfx.drawable_size();
        let scale_factor = window_width / game_constants::FIELD_WIDTH;

        let mut rng = rand::thread_rng();

        let moon_x_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let moon_y_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };

        let sun_x_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let sun_y_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };

        let moon_pos = (window_width * 0.7, window_height * 0.6);
        let sun_pos = (window_width * 0.35, window_height * 0.6);

        let field_start_x: f32 = (window_width - game_constants::FIELD_WIDTH) / 2.0;
        let field_start_y: f32 = (window_height - game_constants::FIELD_HEIGHT) / 2.0;

        let bounds = (field_start_x, field_start_y);

        let moon_circle = Object {
            position: moon_pos,
            team: Team::MOON,
            kind: ObjectKind::Circle,
            direction: Vector2::new(
                moon_x_sign * 1.5 * game_constants::MOVEMENT_SPEED,
                moon_y_sign * 1.5 * game_constants::MOVEMENT_SPEED
            ),
        
        };

        let sun_circle = Object {
            position: sun_pos,
            team: Team::SUN,
            kind: ObjectKind::Circle,
            direction: Vector2::new(
                sun_x_sign * 1.5 * game_constants::MOVEMENT_SPEED,
                sun_y_sign * 1.5 * game_constants::MOVEMENT_SPEED
            ),
        
        };


        let mut squares = Vec::new();

        for row in 0..game_constants::ROW_SIZE {
            for column in 0..game_constants::COLUMN_SIZE {
                squares.push(Object {
                    position: (
                        (field_start_x + (column as f32) * game_constants::SQUARE_SIZE),
                        (field_start_y + (row as f32) * game_constants::SQUARE_SIZE),
                    ),
                    team: Team::MOON,
                    kind: ObjectKind::Square,
                    direction: Vector2::new(0.0, 0.0),
                
                });
            }
        }

        for row in 0..game_constants::ROW_SIZE {
            for column in 0..game_constants::COLUMN_SIZE {
                squares.push(Object {
                    position: (
                        (field_start_x + (column as f32) * game_constants::SQUARE_SIZE + (game_constants::COLUMN_SIZE as f32) * game_constants::SQUARE_SIZE),
                        (field_start_y + (row as f32) * game_constants::SQUARE_SIZE),
                    ),
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
            window_height,
            window_width,
            scale_factor,
            bounds,

            squares,
            sun_circle,
            moon_circle,
        }
    }

    fn scale_position(&self, position: (f32, f32)) -> (f32, f32) {
        (position.0 * self.scale_factor, position.1 * self.scale_factor)
    }

    fn scale_vector(&self, vector: Vector2<f32>) -> Vector2<f32> {
        Vector2::new(vector.x * self.scale_factor, vector.y * self.scale_factor)
    }
}

impl EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult {
        self.sun_circle.update_position();
        self.moon_circle.update_position();

        // println!("MOON POS: x: {} y: {}", self.moon_circle.position.0, self.moon_circle.position.1);
        // println!("SUN POS: x: {} y: {}", self.sun_circle.position.0, self.sun_circle.position.1);

        self.sun_circle.handle_boundary_collision(self.bounds);
        self.moon_circle.handle_boundary_collision(self.bounds);

        for square in &mut self.squares {
            self.sun_circle.handle_collision(square);
            self.moon_circle.handle_collision(square);
        }

        Ok(())
    }
    // TODO fix this MF
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas_color = GgezColor::from_rgb_u32(0x767B91);
        let mut canvas = graphics::Canvas::from_frame(ctx, canvas_color);

        for i in 0..self.squares.len() {
            self.squares[i].draw(ctx, &mut canvas)?;
        }


        self.sun_circle.draw(ctx, &mut canvas)?;
        self.moon_circle.draw(ctx, &mut canvas)?;

        let font_color = GgezColor::from_rgb_u32(0xE1E5EE); 
        let text_fragment = TextFragment::new(format_squares_numbers(count_objects(&self.squares, Team::MOON), count_objects(&self.squares, Team::SUN)))
            .font("LiberationMono")
            .scale(40.0 * self.scale_factor)
            .color(font_color);
        
        let text = Text::new(text_fragment);
        let text_dimensions = text.measure(ctx)?;
        
        let x = (self.window_width - text_dimensions.x) / 2.0;
        let y = self.window_height - text_dimensions.y;
        
        let dest_point = ggez::glam::Vec2::new(x, y);
        
        canvas.draw(
            &text,
            dest_point,
        );


        canvas.finish(ctx)?;

        Ok(())
    }
}