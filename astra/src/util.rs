use astra_types::TerrainData;
use egui::Color32;

use crate::AppConfig;

pub fn get_tile_color(tile: &TerrainData, config: &AppConfig) -> Color32 {
    config
        .tile_color_overrides
        .get(&tile.tid)
        .copied()
        .unwrap_or_else(|| {
            Color32::from_rgb(
                (tile.color_r.unwrap_or_default() as f32 * config.terrain_brightness) as u8,
                (tile.color_g.unwrap_or_default() as f32 * config.terrain_brightness) as u8,
                (tile.color_b.unwrap_or_default() as f32 * config.terrain_brightness) as u8,
            )
        })
}