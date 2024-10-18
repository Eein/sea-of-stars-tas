use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};
use data::prelude::*;
use itertools::Itertools;

pub const NAME: &str = "Inventory Helper";

#[derive(Debug)]
pub struct InventoryHelper;

impl InventoryHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for InventoryHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let currency_manager = &game_state.memory_managers.currency_manager.data;
        let inventory_manager = &game_state.memory_managers.inventory_manager.data;

        ui.label(format!("Money: {}", currency_manager.money));
        ui.separator();
        ui.label("Inventory");

        for group in &inventory_manager
            .items
            .items
            .iter()
            .map(|(item, quantity)| (all_items().get(&item.0 as &str).unwrap(), quantity))
            .sorted_by_key(|(item, _quantity)| item.item_type.clone())
            .chunk_by(|(item, _quantity)| item.item_type.clone())
        {
            let title = format!("{:?}", group.0);
            egui::CollapsingHeader::new(title)
                .default_open(true)
                .show(ui, |ui| {
                    for (item, quantity) in group
                        .1
                        .sorted_by_key(|(item, _quantity)| item.order_priority)
                        .rev()
                    {
                        ui.label(format!("{} x {}", quantity.0, item.name));
                    }
                });
        }
    }
}
