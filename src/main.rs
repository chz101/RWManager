use eframe::egui::{Context, Id};
use eframe::{egui, App, Frame, NativeOptions};

use egui_dnd::utils::shift_vec;
use egui_dnd::{DragDropItem, DragDropUi};

mod manager;

struct DnDApp {
    // DragDropUi stores state about the currently dragged item
    dnd: DragDropUi,
    itemsone: Vec<ItemType>,
    itemstwo: Vec<ItemType>,
}

impl Default for DnDApp {
    fn default() -> Self {
        DnDApp {
            dnd: DragDropUi::default(),
            itemsone: ["alfred", "bernhard", "christian"]
                .iter()
                .map(|name| ItemType {
                    name: name.to_string(),
                })
                .collect(),
            itemstwo: ["jerry", "simon", "lucy"]
                .iter()
                .map(|name| ItemType {
                    name: name.to_string(),
                })
                .collect(),
        }
    }
}

struct ItemType {
    name: String,
}

// We need this to uniquely identify items. You can also implement the Hash trait.
impl DragDropItem for ItemType {
    fn id(&self) -> Id {
        Id::new(&self.name)
    }
}

impl App for DnDApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("Lef Side").show(ctx, |ui| {
            let response =
                // make sure this is called in a vertical layout.
                // Horizontal sorting is not supported yet.
                self.dnd.ui::<ItemType>(ui, self.itemstwo.iter_mut(), |item, ui, handle| {
                    ui.horizontal(|ui| {
                        // Anything in the handle can be used to drag the item
                        handle.ui(ui, item, |ui| {
                            ui.label("grab");
                        });

                        ui.label(&item.name);
                    });
                });

            // After the drag is complete, we get a response containing the old index of the
            // dragged item, as well as the index it was moved to. You can use the
            // shift_vec function as a helper if you store your items in a Vec.
            if let Some(response) = response.completed {
                shift_vec(response.from, response.to, &mut self.itemstwo);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let response =
                // make sure this is called in a vertical layout.
                // Horizontal sorting is not supported yet.
                self.dnd.ui::<ItemType>(ui, self.itemsone.iter_mut(), |item, ui, handle| {
                    ui.horizontal(|ui| {
                        // Anything in the handle can be used to drag the item
                        handle.ui(ui, item, |ui| {
                            ui.label("grab");
                        });

                        ui.label(&item.name);
                    });
                });

            // After the drag is complete, we get a response containing the old index of the
            // dragged item, as well as the index it was moved to. You can use the
            // shift_vec function as a helper if you store your items in a Vec.
            if let Some(response) = response.completed {
                shift_vec(response.from, response.to, &mut self.itemsone);
            }
        });
    }
}

pub fn main() {
    let mut m = manager::Manager::new();
    m.fetch_mods(String::from("/home/creami/Documents/rwmanager/test/input"));
    m.load_active_from_file("/home/creami/Documents/rwmanager/test/input/ModsConfig.xml");
    m.save_mods("/home/creami/Documents/rwmanager/test/");
    //m.save_mod_list("/home/creami/Documents/rwmanager/test/");
    //m.load_mod_list("/home/creami/Documents/rwmanager/test/");

    eframe::run_native(
        "DnD Example",
        NativeOptions::default(),
        Box::new(|_a| Box::<DnDApp>::default()),
    )
    .unwrap();
}
