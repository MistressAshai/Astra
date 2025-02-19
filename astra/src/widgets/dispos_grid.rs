use std::borrow::Cow;
use std::collections::HashMap;

use astra_formats::TerrainData;
use astra_types::Spawn;
use egui::{Button, Color32, Grid, ScrollArea, TextureHandle, Ui, Vec2};
use indexmap::IndexMap;
use itertools::Itertools;

use crate::editors::Difficulty;
use crate::util::get_tile_color;
use crate::{AppConfig, CoordinateKind, DecorationKind, EditorState, ViewItem};

struct SpawnData<'a> {
    group: &'a str,
    index: usize,
    spawn: &'a mut Spawn,
}

pub struct DisposGridResult {
    pub changed: bool,
    pub hovered_tile: Option<String>,
    pub hovered_spawn: Option<String>,
}

fn get_position(spawn: &Spawn, coordinate_kind: CoordinateKind) -> (usize, usize) {
    match coordinate_kind {
        CoordinateKind::Dispos => (spawn.dispos_x as usize, spawn.dispos_y as usize),
        CoordinateKind::Appear => (spawn.appear_x as usize, spawn.appear_y as usize),
    }
}

fn set_position(spawn: &mut Spawn, coordinate_kind: CoordinateKind, row: usize, col: usize) {
    match coordinate_kind {
        CoordinateKind::Dispos => {
            spawn.dispos_x = col as i8;
            spawn.dispos_y = row as i8;
        }
        CoordinateKind::Appear => {
            spawn.appear_x = col as i8;
            spawn.appear_y = row as i8;
        }
    }
}

struct SpawnDataMap<'a> {
    spawns_by_position: HashMap<(usize, usize), Vec<SpawnData<'a>>>,
}

impl<'a> SpawnDataMap<'a> {
    pub fn new(
        dispos: &'a mut IndexMap<String, Vec<Spawn>>,
        coordinate_kind: CoordinateKind,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            spawns_by_position: dispos
                .iter_mut()
                .flat_map(|(group_name, group)| {
                    group
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, spawn)| match difficulty {
                            Difficulty::All => true,
                            Difficulty::Normal => spawn.flag & 1 != 0,
                            Difficulty::Hard => spawn.flag & 2 != 0,
                            Difficulty::Lunatic => spawn.flag & 4 != 0,
                        })
                        .map(|(index, spawn)| {
                            let position = get_position(spawn, coordinate_kind);
                            (
                                position,
                                SpawnData {
                                    group: group_name.as_str(),
                                    index,
                                    spawn,
                                },
                            )
                        })
                })
                .into_group_map(),
        }
    }

    pub fn get_sprite(
        &self,
        state: &EditorState,
        row: usize,
        col: usize,
    ) -> Option<(TextureHandle, f32)> {
        self.get_spawn(row, col).and_then(|spawn_data| {
            spawn_data
                .spawn
                .decoration(state, DecorationKind::Other("spawn_grid"))
        })
    }

    pub fn get_spawn(&self, row: usize, col: usize) -> Option<&SpawnData<'_>> {
        self.spawns_by_position
            .get(&(col, row))
            .and_then(|group| group.last())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn dispos_grid(
    ui: &mut Ui,
    terrain: &TerrainData,
    state: &EditorState,
    dispos: &mut IndexMap<String, Vec<Spawn>>,
    selected_spawn: &mut Option<(String, usize)>,
    coordinate_kind: CoordinateKind,
    difficulty: Difficulty,
    config: &AppConfig,
) -> DisposGridResult {
    let selected_spawn_position = selected_spawn
        .as_ref()
        .and_then(|(group, index)| dispos.get(group).and_then(|group| group.get(*index)))
        .map(|spawn| get_position(spawn, coordinate_kind));
    let spawn_data = SpawnDataMap::new(dispos, coordinate_kind, difficulty);
    let mut changed = false;
    let mut move_pos = None;
    let mut hovered_tile = None;
    let mut hovered_spawn = None;
    ScrollArea::both()
        .id_source("spawn_grid_scroll")
        .show(ui, |ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            ui.spacing_mut().item_spacing = Vec2::new(1., 1.);
            Grid::new("chapter_spawn_grid").show(ui, |ui| {
                state.terrain.read(|data| {
                    for row in (0..(terrain.height as usize)).rev() {
                        for col in 0..(terrain.width as usize) {
                            let sprite = spawn_data.get_sprite(state, row, col);
                            let (tile_name, fill) = terrain
                                .terrains
                                .get(row * 32 + col)
                                .and_then(|tid| data.get(tid.as_str()))
                                .map(|tile| (tile.text(state), get_tile_color(tile, config)))
                                .unwrap_or_else(|| (Cow::Borrowed("???"), Color32::from_gray(0)));

                            // Put these in a container to please egui's grid.
                            let mut button = Button::new("").rounding(0.).fill(fill);
                            if Some((col, row)) == selected_spawn_position {
                                button = button.stroke(ui.visuals().widgets.active.fg_stroke)
                            }
                            ui.vertical(|ui| {
                                let response = ui.add_sized([48., 48.], button);
                                if let Some((sprite, _)) = sprite {
                                    ui.allocate_ui_at_rect(response.rect, |ui| {
                                        ui.image(&sprite);
                                    });
                                }
                                if response.hovered() {
                                    hovered_tile = Some(tile_name.into_owned());
                                    if let Some(spawn_data) = spawn_data.get_spawn(row, col) {
                                        hovered_spawn =
                                            Some(spawn_data.spawn.text(state).into_owned());
                                    }
                                }
                                if response.clicked() {
                                    move_pos = Some((row, col));
                                } else if response.secondary_clicked() {
                                    if let Some(spawn_data) = spawn_data.get_spawn(row, col) {
                                        *selected_spawn =
                                            Some((spawn_data.group.to_string(), spawn_data.index));
                                    }
                                }
                            });
                        }
                        ui.end_row();
                    }
                });
            });
        });
    if let (Some((row, col)), Some((group, index))) = (move_pos, selected_spawn) {
        let spawn = dispos
            .get_mut(group)
            .and_then(|group| group.get_mut(*index));
        if let Some(spawn) = spawn {
            set_position(spawn, coordinate_kind, row, col);
            changed = true;
        }
    }

    DisposGridResult {
        changed,
        hovered_tile,
        hovered_spawn,
    }
}
