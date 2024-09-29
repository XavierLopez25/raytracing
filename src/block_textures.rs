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
}

impl Textures {
    pub fn new() -> Self {
        let grass_texture = Arc::new(Texture::new("assets/grass.png"));
        let grass_material =
            Material::new_with_texture(1.0, [1.0, 0.05, 0.0, 0.0], 0.0, grass_texture);

        let obsidian_texture = Arc::new(Texture::new("assets/obsidian.png"));
        let obsidian_material =
            Material::new_with_texture(0.01, [0.45, 0.01, 0.01, 0.0], 1.5, obsidian_texture);

        let bookshelf_texture = Arc::new(Texture::new("assets/bookshelf.png"));
        let bookshelf_material =
            Material::new_with_texture(0.8, [0.60, 0.40, 0.05, 0.0], 0.0, bookshelf_texture);

        let cactus_texture = Arc::new(Texture::new("assets/cactus.png"));
        let cactus_material =
            Material::new_with_texture(0.85, [0.75, 0.85, 0.15, 0.0], 0.0, cactus_texture);

        let dirt_texture = Arc::new(Texture::new("assets/dirt.png"));
        let dirt_material =
            Material::new_with_texture(1.0, [1.0, 0.05, 0.0, 0.0], 0.0, dirt_texture);

        let oak_log_texture = Arc::new(Texture::new("assets/oak_log.png"));
        let oak_log_material =
            Material::new_with_texture(0.9, [0.55, 0.40, 0.05, 0.0], 0.0, oak_log_texture);

        let oak_planks_texture = Arc::new(Texture::new("assets/oak_planks.png"));
        let oak_planks_material =
            Material::new_with_texture(1.0, [0.95, 0.1, 0.0, 0.0], 1.3, oak_planks_texture);

        let sand_texture = Arc::new(Texture::new("assets/sand.png"));
        let sand_material =
            Material::new_with_texture(1.0, [0.95, 0.90, 0.10, 0.0], 0.0, sand_texture);

        let shroomlight_texture = Arc::new(Texture::new("assets/shroomlight.png"));
        let shroomlight_material =
            Material::new_with_texture(0.5, [1.0, 0.9, 0.1, 0.0], 0.0, shroomlight_texture);

        let smoker_texture = Arc::new(Texture::new("assets/smoker.png"));
        let smoker_material =
            Material::new_with_texture(0.7, [0.35, 0.25, 0.05, 0.0], 0.0, smoker_texture);

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
        }
    }
}
