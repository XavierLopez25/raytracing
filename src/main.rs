use core::f32;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Duration;

use std::f32::consts::PI;
use std::f32::INFINITY;

mod framebuffer;
use framebuffer::Framebuffer;

mod ray_intersect;
use ray_intersect::{Intersect, RayIntersect};

mod color;
use color::Color;

mod camera;
use camera::Camera;

mod material;
use material::Material;

mod light;
use light::Light;

mod texture;
use texture::Texture;

mod cube;
use cube::Cube;

use rayon::prelude::*;

mod block_textures;
use block_textures::Textures;

const BIAS: f32 = 0.001;
const AMBIENT_LIGHT_COLOR: Color = Color::new(25, 25, 25);
const AMBIENT_INTENSITY: f32 = 0.3;

fn offset_point(intersect: &Intersect, _direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * BIAS;
    intersect.point + offset
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);

    let n_cosi: f32;
    let eta: f32;
    let n_normal: Vec3;

    if cosi < 0.0 {
        // Entering
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // Leaving
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k > 0.0 {
        // Total internal reflection
        reflect(incident, &n_normal)
    } else {
        incident * eta + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Cube]) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();
    let shadow_ray_origin = offset_point(intersect, &light_dir);

    objects
        .iter()
        .find_map(|object| {
            let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
            if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
                Some(
                    0.5 - (shadow_intersect.distance / light_distance)
                        .powf(2.0)
                        .min(1.0),
                )
            } else {
                None
            }
        })
        .unwrap_or(0.0)
}

fn fresnel_schlick(cos_theta: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

fn get_skybox_color(direction: &Vec3, skybox_texture: &Texture) -> Color {
    let normalized_dir = direction.normalize();
    let u = 0.5 + normalized_dir.x.atan2(normalized_dir.z) / (2.0 * PI);
    let v = 0.5 - (normalized_dir.y.asin() / PI);

    let u = u.fract();
    let v = v.fract();

    let tex_width = skybox_texture.width() as f32;
    let tex_height = skybox_texture.height() as f32;
    let tex_x = (u * tex_width).floor() as usize;
    let tex_y = (v * tex_height).floor() as usize;

    skybox_texture.get_color(tex_x, tex_y)
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Cube],
    light: &Light,
    depth: u32,
    textures: &Textures,
) -> Color {
    if depth >= 3 {
        return get_skybox_color(ray_direction, &textures.skybox_texture);
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);

        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        return get_skybox_color(ray_direction, &textures.skybox_texture);
    }

    let ambient_light = AMBIENT_LIGHT_COLOR * AMBIENT_INTENSITY;

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
    let cos_theta = -ray_direction.dot(&intersect.normal).max(-1.0).min(1.0);

    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse_color = intersect
        .material
        .get_diffuse_color(intersect.u, intersect.v);
    let diffuse =
        diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intensity = view_dir
        .dot(&reflect_dir)
        .max(0.0)
        .powf(intersect.material.specular);
    let specular =
        light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

    let fresnel_effect = fresnel_schlick(cos_theta.abs(), intersect.material.refractive_index);
    let reflect_color = if intersect.material.albedo[2] > 0.0 {
        let reflect_dir = reflect(ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_point(&intersect, ray_direction);
        cast_ray(
            &reflect_origin,
            &reflect_dir,
            objects,
            light,
            depth + 1,
            textures,
        ) * fresnel_effect
    } else {
        Color::black()
    };

    let refract_color = if intersect.material.albedo[3] > 0.0 {
        let refract_dir = refract(
            ray_direction,
            &intersect.normal,
            intersect.material.refractive_index,
        );
        let refract_origin = offset_point(&intersect, &refract_dir);
        cast_ray(
            &refract_origin,
            &refract_dir,
            objects,
            light,
            depth + 1,
            textures,
        ) * (1.0 - fresnel_effect)
    } else {
        Color::black()
    };

    ambient_light + diffuse + specular + reflect_color + refract_color
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Cube],
    camera: &Camera,
    light: &Light,
    textures: &Textures,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();

    let pixels: Vec<_> = (0..framebuffer.height)
        .flat_map(|y| (0..framebuffer.width).map(move |x| (x, y)))
        .collect();

    // Calcula los colores de los píxeles en paralelo
    let pixel_colors: Vec<(usize, usize, u32)> = pixels
        .par_iter()
        .map(|&(x, y)| {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);
            let pixel_color =
                cast_ray(&camera.eye, &rotated_direction, objects, light, 0, textures);
            (x, y, pixel_color.to_hex())
        })
        .collect();

    // Aplica los colores de los píxeles en una operación secuencial
    for (x, y, color) in pixel_colors {
        framebuffer.set_current_color(color);
        framebuffer.point(x, y);
    }
}

fn generate_grid(
    width: usize,
    height: usize,
    start_x: f32,
    start_y: f32,
    start_z: f32,
    material: Material,
) -> Vec<Cube> {
    let mut objects = Vec::new();
    for y in 0..height {
        for x in 0..width {
            objects.push(Cube {
                min: Vec3::new(start_x + x as f32, start_y, start_z + y as f32),
                max: Vec3::new(
                    start_x + (x + 1) as f32,
                    start_y + 1.0,
                    start_z + (y + 1) as f32,
                ),
                material: material.clone(),
            });
        }
    }
    objects
}

fn create_nether_portal(textures: &Textures) -> Vec<Cube> {
    let mut objects = Vec::new();

    objects.push(Cube {
        min: Vec3::new(3.0, 2.0, 6.0),
        max: Vec3::new(4.0, 3.0, 7.0),
        material: textures.oak_planks_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(4.0, 2.0, 6.0),
        max: Vec3::new(5.0, 3.0, 7.0),
        material: textures.obsidian_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(5.0, 2.0, 6.0),
        max: Vec3::new(6.0, 3.0, 7.0),
        material: textures.obsidian_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(6.0, 2.0, 6.0),
        max: Vec3::new(7.0, 3.0, 7.0),
        material: textures.oak_planks_material.clone(),
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 3.0, 6.0),
        max: Vec3::new(4.0, 4.0, 7.0),
        material: textures.obsidian_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(6.0, 3.0, 6.0),
        max: Vec3::new(7.0, 4.0, 7.0),
        material: textures.obsidian_material.clone(),
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 4.0, 6.0),
        max: Vec3::new(4.0, 5.0, 7.0),
        material: textures.obsidian_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(6.0, 4.0, 6.0),
        max: Vec3::new(7.0, 5.0, 7.0),
        material: textures.obsidian_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(6.0, 5.0, 6.0),
        max: Vec3::new(7.0, 6.0, 7.0),
        material: textures.oak_planks_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(5.0, 5.0, 6.0),
        max: Vec3::new(6.0, 6.0, 7.0),
        material: textures.obsidian_material.clone(),
    });

    objects
}

fn create_player_tools(textures: &Textures) -> Vec<Cube> {
    let mut objects = Vec::new();

    let wood_row = generate_grid(1, 7, 7.0, 1.0, 0.0, textures.oak_planks_material.clone());
    objects.extend(wood_row);

    objects.push(Cube {
        min: Vec3::new(7.0, 2.0, 2.0),
        max: Vec3::new(8.0, 3.0, 3.0),
        material: textures.smoker_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(7.0, 2.0, 3.0),
        max: Vec3::new(8.0, 3.0, 4.0),
        material: textures.crafting_table_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(7.0, 2.0, 4.0),
        max: Vec3::new(8.0, 3.0, 5.0),
        material: textures.bookshelf_material.clone(),
    });

    objects
}

fn create_cactus(textures: &Textures) -> Vec<Cube> {
    let mut objects = Vec::new();

    objects.push(Cube {
        min: Vec3::new(2.0, 2.0, 2.0),
        max: Vec3::new(3.0, 3.0, 3.0),
        material: textures.sand_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(2.0, 3.0, 2.0),
        max: Vec3::new(3.0, 4.0, 3.0),
        material: textures.cactus_material.clone(),
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 2.0, 1.0),
        max: Vec3::new(4.0, 3.0, 2.0),
        material: textures.sand_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(3.0, 3.0, 1.0),
        max: Vec3::new(4.0, 4.0, 2.0),
        material: textures.cactus_material.clone(),
    });

    objects
}

fn create_tree(textures: &Textures) -> Vec<Cube> {
    let mut objects = Vec::new();

    objects.push(Cube {
        min: Vec3::new(1.0, 2.0, 5.0),
        max: Vec3::new(2.0, 3.0, 6.0),
        material: textures.oak_log_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(1.0, 3.0, 5.0),
        max: Vec3::new(2.0, 4.0, 6.0),
        material: textures.oak_log_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(1.0, 4.0, 5.0),
        max: Vec3::new(2.0, 5.0, 6.0),
        material: textures.oak_log_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(1.0, 5.0, 5.0),
        max: Vec3::new(2.0, 6.0, 6.0),
        material: textures.oak_log_material.clone(),
    });

    objects
}

fn create_shroomlights(textures: &Textures) -> Vec<Cube> {
    let mut objects = Vec::new();

    objects.push(Cube {
        min: Vec3::new(7.0, 2.0, 0.0),
        max: Vec3::new(8.0, 3.0, 1.0),
        material: textures.shroomlight_material.clone(),
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 2.0, 0.0),
        max: Vec3::new(1.0, 3.0, 1.0),
        material: textures.shroomlight_material.clone(),
    });

    objects
}
fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Gráficas - Diorama Minecraft",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(0x333355);

    let ivory: Material = Material::new(Color::new(100, 100, 80), 50.0, [0.6, 0.3, 0.6, 0.0], 0.0);

    let glass = Material::new(
        Color::new(255, 255, 255),
        1425.0,
        [0.0, 10.0, 0.5, 0.5],
        0.3,
    );

    let textures = Textures::new();

    let mut objects = generate_grid(7, 7, 0.0, 1.0, 0.0, textures.grass_material.clone());
    let mut dirt = generate_grid(7, 7, 0.0, 0.0, 0.0, textures.dirt_material.clone());
    objects.append(&mut dirt);

    objects.append(&mut create_nether_portal(&textures));
    objects.append(&mut create_player_tools(&textures));
    objects.append(&mut create_cactus(&textures));
    objects.append(&mut create_tree(&textures));
    objects.append(&mut create_shroomlights(&textures));

    // objects.append(&mut create_player_tools(&textures));

    // Configuración de la cámara
    let mut camera = Camera::new(
        Vec3::new(-5.0, 5.0, -10.0), // Posición de la cámara ajustada
        Vec3::new(0.0, 0.0, 0.0),    // Punto hacia el que mira la cámara
        Vec3::new(0.0, 1.0, 0.0),    // Vector "up" de la cámara
    );

    let light = Light::new(Vec3::new(-5.0, 10.0, -10.0), Color::new(255, 255, 255), 1.0);

    let rotation_speed = PI / 50.0;
    let movement_speed = 0.1;
    let zoom_speed = 0.5;

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        //  camera orbit controls
        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, rotation_speed);
        }

        // Camera movement controls
        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        if window.is_key_down(Key::A) {
            movement.x -= movement_speed;
        }
        if window.is_key_down(Key::D) {
            movement.x += movement_speed;
        }
        if window.is_key_down(Key::Q) {
            movement.y += movement_speed;
        }
        if window.is_key_down(Key::E) {
            movement.y -= movement_speed;
        }
        if movement.magnitude() > 0.0 {
            camera.move_center(movement);
        }

        // Camera zoom controls
        if window.is_key_down(Key::Up) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-zoom_speed);
        }

        framebuffer.clear();
        render(&mut framebuffer, &objects, &mut camera, &light, &textures);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
