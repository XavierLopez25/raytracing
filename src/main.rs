use image::Frame;
use minifb::{Key, KeyRepeat, MouseMode, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::f32::consts::PI;

use std::sync::Arc;
use std::time::{Duration, Instant};

mod framebuffer;
use framebuffer::Framebuffer;

mod player;
use player::{process_events, Player};

mod texture;
use texture::Texture;

mod sphere;
use sphere::Sphere;

mod ray_intersect;
use ray_intersect::RayIntersect;

mod color;
use color::Color;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>]) -> u32 {
    for object in objects {
        if object.ray_intersect(ray_origin, ray_direction) {
            return 0x91e57d;
        }
    }
    0x91e57d
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

            // Cast the ray and get the pixel color
            let origin = Vec3::new(0.0, 0.0, 0.0);
            let pixel_color = cast_ray(&origin, &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}
fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Unnamed Raycaster - Press 'S' to Start",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.set_position(100, 100);

    let initial_mouse_x = window
        .get_mouse_pos(MouseMode::Pass)
        .map_or(0.0, |(x, _)| x as f32);

    framebuffer.set_background_color(0x333355);

    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    })];

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        framebuffer.clear();

        render(&mut framebuffer, &objects);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
