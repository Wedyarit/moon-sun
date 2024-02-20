
use ggez::{graphics, Context, GameResult, event::EventHandler};
use ggez::graphics::{Color, Text, TextFragment};
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
    prev_window_size: (f32, f32),

    squares: Vec<Object>,
    sun_circle: Object,
    moon_circle: Object,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let (window_width, window_height) = ctx.gfx.drawable_size();
        let prev_window_size = (window_width, window_height);
        let scale_factor = window_width / game_constants::FIELD_WIDTH;

        let mut rng = rand::thread_rng();

        let moon_x_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let moon_y_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };

        let sun_x_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let sun_y_sign = if rng.gen::<bool>() { 1.0 } else { -1.0 };

        let moon_pos = (window_width * 0.7, window_height * 0.6);
        let sun_pos = (window_width * 0.35, window_height * 0.6);

        let field_start_x: f32 = (window_width - game_constants::FIELD_WIDTH * scale_factor) / 2.0;
        let field_start_y: f32 = (window_height - game_constants::FIELD_HEIGHT * scale_factor) / 2.0;

        let bounds = (field_start_x, field_start_y);

        let moon_circle = Object {
            position: moon_pos,
            team: Team::MOON,
            kind: ObjectKind::Circle,
            direction: Vector2::new(
                moon_x_sign * 1.5 * game_constants::MOVEMENT_SPEED,
                moon_y_sign * 1.5 * game_constants::MOVEMENT_SPEED
            ),
            scale: scale_factor
        };

        let sun_circle = Object {
            position: sun_pos,
            team: Team::SUN,
            kind: ObjectKind::Circle,
            direction: Vector2::new(
                sun_x_sign * 1.5 * game_constants::MOVEMENT_SPEED,
                sun_y_sign * 1.5 * game_constants::MOVEMENT_SPEED
            ),
            scale: scale_factor
        
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
                    scale: scale_factor
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
                    scale: scale_factor
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
            prev_window_size,

            squares,
            sun_circle,
            moon_circle,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.sun_circle.update_position(self.scale_factor);
        self.moon_circle.update_position(self.scale_factor);

        self.sun_circle.handle_boundary_collision(self.bounds);
        self.moon_circle.handle_boundary_collision(self.bounds);

        for square in &mut self.squares {

            self.sun_circle.handle_collision(square);
            self.moon_circle.handle_collision(square);
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let captured = count_objects(&self.squares, Team::MOON);

        let max_cells = game_constants::ROW_SIZE * game_constants::COLUMN_SIZE;

        let darkness_level = captured as f32 / max_cells as f32;

        let start_color = [0xF7, 0xC5, 0x9F];
        let end_color = [0x2A, 0x32, 0x4B];

        let interpolated_color = [
        (start_color[0] as f32 + (end_color[0] as f32 - start_color[0] as f32) * darkness_level) as u8,
        (start_color[1] as f32 + (end_color[1] as f32 - start_color[1] as f32) * darkness_level) as u8,
        (start_color[2] as f32 + (end_color[2] as f32 - start_color[2] as f32) * darkness_level) as u8,
        ];
    
        let canvas_color = Color::from_rgb(interpolated_color[0], interpolated_color[1], interpolated_color[2]);

        let mut canvas = graphics::Canvas::from_frame(ctx, canvas_color);

        for i in 0..self.squares.len() {
            self.squares[i].draw(ctx, &mut canvas)?;
        }
    
        self.sun_circle.draw(ctx, &mut canvas)?;
        self.moon_circle.draw(ctx, &mut canvas)?;
    
        let font_color = Color::from_rgb_u32(0xE1E5EE); 
        let text_fragment = TextFragment::new(format_squares_numbers(count_objects(&self.squares, Team::MOON), count_objects(&self.squares, Team::SUN)))
            .font("LiberationMono")
            .scale(40.0 * self.scale_factor)
            .color(font_color);
        
        let text = Text::new(text_fragment);
        let text_dimensions = text.measure(ctx)?;
        
        let x = (self.window_width - text_dimensions.x) / 2.0;
        let y = self.window_height - text_dimensions.y;
        
        let dest_point = ggez::glam::Vec2::new(x, y);
        
        canvas.draw(&text, dest_point);
    
        canvas.finish(ctx)?;
    
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult {
        self.window_width = width;
        self.window_height = height;

        if width != height {
            let (prev_width, prev_height) = self.prev_window_size;
            if width > prev_width || width < prev_width {
                ctx.gfx.set_drawable_size(width, width)?;
            } 

            if height > prev_height || height < prev_height {
                ctx.gfx.set_drawable_size(height, height)?;
            } 
        }

        self.prev_window_size = (width, height);


        let scale_factor_width = width / game_constants::FIELD_WIDTH;
        let scale_factor_height = height / game_constants::FIELD_HEIGHT;
        self.scale_factor = scale_factor_width.min(scale_factor_height);

        let field_size = game_constants::FIELD_WIDTH.min(game_constants::FIELD_HEIGHT);
        let field_start_x = (width - field_size * self.scale_factor) / 2.0;
        let field_start_y = (height - field_size * self.scale_factor) / 2.0;

        self.bounds = (field_start_x, field_start_y);


        for square in &mut self.squares {
            square.scale = self.scale_factor;
        }

        self.sun_circle.scale = self.scale_factor;
        self.moon_circle.scale = self.scale_factor;

        Ok(())
    }
    
}
