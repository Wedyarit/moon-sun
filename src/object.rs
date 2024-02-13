use ggez::graphics::{self, Color as GgezColor, Canvas};
use ggez::{Context, GameResult};
use ggez::glam::*;
use nalgebra::Vector2;

use crate::game::game_constants;

pub enum ObjectKind {
    Circle,
    Square,
}

#[derive(PartialEq, Clone)]
pub enum Team {
    MOON,
    SUN,
}

pub struct Object {
    pub position: (f32, f32),
    pub team: Team,
    pub kind: ObjectKind,
    pub direction: Vector2<f32>,
    pub scale: f32
}

impl Object {
    pub fn update_position(&mut self) {
        self.position.0 += self.direction.x * self.scale;
        self.position.1 += self.direction.y * self.scale;
    }

    pub fn toggle_team(&mut self) {
        self.team = match self.team {
            Team::MOON => Team::SUN,
            Team::SUN => Team::MOON,
        };
    }

    pub fn intersects(&self, other: &Object) -> bool {
        match self.kind {
            ObjectKind::Circle => self.circle_intersects_square(other),
            ObjectKind::Square => other.circle_intersects_square(self),
        }
    }

    pub fn handle_collision(&mut self, other: &mut Object) {
        let is_intersecting = self.intersects(other);
        if is_intersecting && self.team == other.team {
            let normal = Vector2::new(other.position.0 * other.scale - self.position.0 * self.scale, other.position.1 * other.scale - self.position.1 * self.scale).normalize();
            self.direction = self.direction - 2.0 * self.direction.dot(&normal) * normal;
            other.toggle_team();
        }
    }

    pub fn handle_boundary_collision(&mut self, bounds: (f32, f32)) {
            let (mut x, mut y) = self.position;

            if x - game_constants::CIRCLE_SIZE <= bounds.0 || x + game_constants::CIRCLE_SIZE >= bounds.0 + game_constants::FIELD_WIDTH {
                self.direction.x *= -1.0;
                x += self.direction.x; 
            }

            if y - game_constants::CIRCLE_SIZE <= bounds.1 || y + game_constants::CIRCLE_SIZE >= bounds.1 + game_constants::FIELD_HEIGHT {
                self.direction.y *= -1.0;
                y += self.direction.y;
            }

            self.position = (x, y);
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        match self.kind {
            ObjectKind::Circle => self.draw_circle(ctx, canvas),
            ObjectKind::Square => self.draw_square(ctx, canvas),
        }
    }

    fn get_color(&self) -> GgezColor {
        match self.team {
            Team::SUN => GgezColor::from_rgb_u32(0xF7C59F),
            Team::MOON => GgezColor::from_rgb_u32(0x2A324B),
        }
    }

    fn circle_intersects_square(&self, square: &Object) -> bool {
        let (circle_x, circle_y) = self.position;
        let (square_x, square_y) = square.position;
        let half_size = game_constants::SQUARE_SIZE / 2.0;

        let closest_x = if circle_x < square_x {
            square_x
        } else if circle_x > square_x + game_constants::SQUARE_SIZE {
            square_x + game_constants::SQUARE_SIZE
        } else {
            circle_x
        };

        let closest_y = (circle_y.max(square_y - half_size)).min(square_y + half_size);

        let dx = circle_x - closest_x;
        let dy = circle_y - closest_y;

        let distance_squared = dx * dx + dy * dy;
        let radius_squared = game_constants::CIRCLE_SIZE * game_constants::CIRCLE_SIZE;
        let is_intersecting = distance_squared < radius_squared;

        is_intersecting
    }

    fn draw_circle(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(self.position.0 * self.scale, self.position.1 * self.scale), 
            game_constants::CIRCLE_SIZE * self.scale,
            0.1,
            self.get_color(),
        )?;
        canvas.draw(&circle, Vec2::new(0.0, 0.0)); 

        Ok(())
    }

    fn draw_square(&self, ctx: &mut Context, canvas: &mut Canvas) -> ggez::GameResult<()> {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.position.0 * self.scale, self.position.1 * self.scale, game_constants::SQUARE_SIZE * self.scale, game_constants::SQUARE_SIZE * self.scale),
            self.get_color(),
        )?;
        canvas.draw(&rect, Vec2::new(0.0, 0.0)); 

        Ok(())
    }
    
}
