use crate::material::Material;
use crate::texture::Texture;
use std::sync::Arc;

pub struct Textures {
    pub grass_material: Material,
    // pub water_material: Material,
    // pub stone_material: Material,
    // Add other materials here
}

impl Textures {
    pub fn new() -> Self {
        let grass_texture = Arc::new(Texture::new("assets/grass_signed.png"));
        let grass_material =
            Material::new_with_texture(1.0, [1.0, 0.05, 0.0, 0.0], 0.0, grass_texture);

        // let water_texture = Arc::new(Texture::new("assets/water_signed.png"));
        // let water_material =
        //     Material::new_with_texture(0.5, [0.5, 0.5, 0.5, 0.5], 1.33, water_texture);

        // let stone_texture = Arc::new(Texture::new("assets/stone_signed.png"));
        // let stone_material =
        //     Material::new_with_texture(0.8, [0.1, 0.9, 0.0, 0.0], 0.0, stone_texture);

        Self {
            grass_material,
            // water_material,
            // stone_material,
            // Initialize other materials here
        }
    }
}
