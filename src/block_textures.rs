use crate::color::Color;
use crate::material::Material;
use crate::texture::Texture;
use std::sync::Arc;

pub struct Textures {
    pub grass_material: Material,
    pub obsidian_material: Material,
    pub bookshelf_material: Material,
    pub cactus_material: Material,
    pub dirt_material: Material,
    pub oak_log_material: Material,
    pub oak_planks_material: Material,
    pub sand_material: Material,
    pub shroomlight_material: Material,
    pub smoker_material: Material,
    pub crafting_table_material: Material,
    pub skybox_texture: Arc<Texture>,
}

impl Textures {
    pub fn new() -> Self {
        let grass_texture = Arc::new(Texture::new("assets/grass.png"));
        let grass_material = Material::new_with_texture(
            1.0,
            [1.0, 0.05, 0.0, 0.0],
            0.0,
            grass_texture,
            Color::new(0, 0, 0),
        );

        let obsidian_texture = Arc::new(Texture::new("assets/obsidian.png"));
        let obsidian_material = Material::new_with_texture(
            0.2,
            [0.45, 0.01, 0.0, 0.0],
            0.2,
            obsidian_texture,
            Color::new(0, 0, 0),
        );

        let bookshelf_texture = Arc::new(Texture::new("assets/bookshelf.png"));
        let bookshelf_material = Material::new_with_texture(
            1.0,
            [0.95, 0.1, 0.0, 0.0],
            1.0,
            bookshelf_texture,
            Color::new(0, 0, 0),
        );

        let cactus_texture = Arc::new(Texture::new("assets/cactus.png"));
        let cactus_material = Material::new_with_texture(
            1.0,
            [0.50, 0.01, 0.0, 0.0],
            1.5,
            cactus_texture,
            Color::new(0, 0, 0),
        );

        let dirt_texture = Arc::new(Texture::new("assets/dirt.png"));
        let dirt_material = Material::new_with_texture(
            1.0,
            [1.0, 0.05, 0.0, 0.0],
            0.0,
            dirt_texture,
            Color::new(0, 0, 0),
        );

        let oak_log_texture = Arc::new(Texture::new("assets/oak_log.png"));
        let oak_log_material = Material::new_with_texture(
            1.0,
            [0.95, 0.10, 0.0, 0.0],
            1.3,
            oak_log_texture,
            Color::new(0, 0, 0),
        );

        let oak_planks_texture = Arc::new(Texture::new("assets/oak_planks.png"));
        let oak_planks_material = Material::new_with_texture(
            1.0,
            [0.95, 0.1, 0.0, 0.0],
            1.3,
            oak_planks_texture,
            Color::new(0, 0, 0),
        );

        let crafting_table_texture = Arc::new(Texture::new("assets/crafting_table.png"));
        let crafting_table_material = Material::new_with_texture(
            1.0,
            [0.95, 0.1, 0.0, 0.0],
            1.3,
            crafting_table_texture,
            Color::new(0, 0, 0),
        );

        let sand_texture = Arc::new(Texture::new("assets/sand.png"));
        let sand_material = Material::new_with_texture(
            1.0,
            [0.90, 0.2, 0.0, 0.0],
            0.0,
            sand_texture,
            Color::new(0, 0, 0),
        );

        let shroomlight_texture = Arc::new(Texture::new("assets/shroomlight.png"));
        let shroomlight_material = Material::new_with_texture(
            0.2,
            [1.0, 0.0, 0.0, 0.0],
            1.0,
            shroomlight_texture,
            Color::new(66, 47, 5),
        );

        let smoker_texture = Arc::new(Texture::new("assets/smoker.png"));
        let smoker_material = Material::new_with_texture(
            1.0,
            [0.95, 0.1, 0.0, 0.0],
            1.2,
            smoker_texture,
            Color::new(0, 0, 0),
        );

        let skybox_texture = Arc::new(Texture::new("assets/skybox.png")); // Asume que tienes una imagen skybox.png

        Self {
            grass_material,
            obsidian_material,
            bookshelf_material,
            cactus_material,
            dirt_material,
            oak_log_material,
            oak_planks_material,
            sand_material,
            shroomlight_material,
            smoker_material,
            crafting_table_material,
            skybox_texture,
        }
    }
}
