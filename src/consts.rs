use egui::Color32;

pub mod field {
    pub const CELL_SIZE: usize = 8;
    pub const CELL_SIZE_HALF: usize = CELL_SIZE / 2;
}

pub mod drawing {
    use super::*;

    pub const BOT_OUTLINE_COLOR_DEF: Color32 = Color32::from_rgb(111, 111, 111);
    pub const BOT_OUTLINE_COLOR: Option<Color32> = Some(BOT_OUTLINE_COLOR_DEF);
    pub const BOT_HEAD_COLOR: Option<Color32> = Some(BOT_OUTLINE_COLOR_DEF);
    pub const FIELD_BACKGROUND_COLOR: Color32 = Color32::WHITE;
    pub const ROCK_DRAW_COLOR: Color32 = Color32::from_rgb(0x62, 0x62, 0x62);
    pub const ORGANIC_WASTE_DRAW_COLOR: Color32 = Color32::from_rgb(0xC8, 0xC8, 0xC8);
    pub const ORGANIC_WASTE_OUTLINE_COLOR: Color32 = Color32::from_rgb(0x80, 0x80, 0x80);
    pub const APPLE_DRAW_COLOR_RGBA: Color32 = Color32::from_rgb(0, 0x64, 0);
    /// Blue water
    pub const BLUE_WATER: Color32 = Color32::from_rgb(150, 150, 255);
    /// Green water
    pub const GREEN_WATER: Color32 = Color32::from_rgb(150, 255, 150);
    pub const OCEAN_COLOR: Color32 = BLUE_WATER;
    pub const MUD_COLOR: Color32 = Color32::from_rgb(140, 80, 62);
    pub const UNDERWATER_MASK_COLOR: Option<Color32> =
        Some(Color32::from_rgba_premultiplied(100, 100, 255, 80));
}
