use macroquad::prelude::*;
use crate::engine::types::{Rect, Point};

pub struct Renderer;

impl Renderer {
    pub fn clear(color: Color) {
        clear_background(color);
    }
    
    pub fn draw_texture_at(texture: &Texture2D, x: f32, y: f32, width: f32, height: f32, rotation: f32) {
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                rotation: rotation.to_radians(),
                ..Default::default()
            },
        );
    }
    
    pub fn draw_rect(rect: Rect, color: Color) {
        draw_rectangle(
            rect.x as f32,
            rect.y as f32,
            rect.w as f32,
            rect.h as f32,
            color,
        );
    }
    
    pub fn draw_rect_outline(rect: Rect, thickness: f32, color: Color) {
        draw_rectangle_lines(
            rect.x as f32,
            rect.y as f32,
            rect.w as f32,
            rect.h as f32,
            thickness,
            color,
        );
    }
    
    pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
            text,
            x - text_dims.width / 2.0,
            y + text_dims.height / 2.0,
            font_size,
            color,
        );
    }
}
