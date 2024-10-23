use std::sync::LazyLock;

pub struct Assets<'a> {
    pub damage_types: DamageTypeAssets<'a>,
    pub stats: StatAssets<'a>,
}

pub struct DamageTypeAssets<'a> {
    pub arcane: egui::Image<'a>,
    pub blunt: egui::Image<'a>,
    pub moon: egui::Image<'a>,
    pub poison: egui::Image<'a>,
    pub sun: egui::Image<'a>,
    pub sword: egui::Image<'a>,
}

pub struct StatAssets<'a> {
    pub hp: egui::Image<'a>,
    pub mp: egui::Image<'a>,
    pub physical_attack: egui::Image<'a>,
    pub physical_defense: egui::Image<'a>,
    pub magical_attack: egui::Image<'a>,
    pub magical_defense: egui::Image<'a>,
}

pub static ASSETS: LazyLock<Assets> = LazyLock::new(Assets::default);

const IMAGE_WIDTH: f32 = 30.0;

impl Default for Assets<'_> {
    fn default() -> Self {
        let damage_types = DamageTypeAssets {
            arcane: egui::Image::new(egui::include_image!("./damage_types/arcane.png"))
                .max_width(IMAGE_WIDTH),
            blunt: egui::Image::new(egui::include_image!("./damage_types/blunt.png"))
                .max_width(IMAGE_WIDTH),
            moon: egui::Image::new(egui::include_image!("./damage_types/moon.png"))
                .max_width(IMAGE_WIDTH),
            poison: egui::Image::new(egui::include_image!("./damage_types/poison.png"))
                .max_width(IMAGE_WIDTH),
            sun: egui::Image::new(egui::include_image!("./damage_types/sun.png"))
                .max_width(IMAGE_WIDTH),
            sword: egui::Image::new(egui::include_image!("./damage_types/sword.png"))
                .max_width(IMAGE_WIDTH),
        };
        let stats = StatAssets {
            hp: egui::Image::new(egui::include_image!("./stats/hp.png")).max_width(IMAGE_WIDTH),
            mp: egui::Image::new(egui::include_image!("./stats/mp.png")).max_width(IMAGE_WIDTH),
            physical_attack: egui::Image::new(egui::include_image!("./stats/atk.png"))
                .max_width(IMAGE_WIDTH),
            physical_defense: egui::Image::new(egui::include_image!("./stats/def.png"))
                .max_width(IMAGE_WIDTH),
            magical_attack: egui::Image::new(egui::include_image!("./stats/matk.png"))
                .max_width(IMAGE_WIDTH),
            magical_defense: egui::Image::new(egui::include_image!("./stats/mdef.png"))
                .max_width(IMAGE_WIDTH),
        };
        Self {
            damage_types,
            stats,
        }
    }
}
