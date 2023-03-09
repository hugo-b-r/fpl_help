#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fpl_help::{get_coordinates, convert_coordinates, url_from};
use eframe::egui;
use geocoding::Point;
use arboard::Clipboard;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 700.0)),
        min_window_size: Some(egui::vec2(480.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "FPL Help",
        native_options,
        Box::new(
            |cc| Box::new(FPLHelp::new(cc))
        )
    )
}

struct FPLHelp {
    clipboard: Clipboard,
    address: String,
    coordinates: Vec<Point<f64>>,
    error: String,
    flight_plan_coordinates: Vec<String>, 
} //on stocke les différentes destinations sous la forme: 

impl Default for FPLHelp {
    fn default() -> FPLHelp {
        FPLHelp {
            clipboard: Clipboard::new().unwrap(),
            address: String::default(),
            coordinates: Vec::new(),
            error: String::default(),
            flight_plan_coordinates: Vec::default(),
        }
    }
}

impl FPLHelp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for FPLHelp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        let Self {
            clipboard,
            address,
            coordinates,
            error,
            flight_plan_coordinates
        } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            
            egui::menu::bar(ui, |ui| {
                ui.text_edit_singleline(address);
                if ui.button("Convert").clicked() {
                    *coordinates = get_coordinates(address.clone()).unwrap_or_else(|err| {
                        eprintln!("Error when geocoding: {}", err);
                        *error = format!("error when geocoding: {}", err);
                        Vec::new()
                    });
                };
                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
            ui.label(error.as_str());
            for point in coordinates.iter() {
                ui.horizontal(|ui| {
                    ui.label(convert_coordinates(*point).unwrap());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        #[cfg(not(traget_arch = "wasm32"))] //not copying or pasting on web
                        if ui.button("Copy").clicked() {
                            let _ = &clipboard.set_text(convert_coordinates(*point).unwrap());
                            println!("copy {} to clipboard", convert_coordinates(*point).unwrap());
                        }
                        if ui.button("Add").clicked() {
                            flight_plan_coordinates.push(format!("{} {}", convert_coordinates(*point).unwrap(), address));
                            println!("Add {} to list of destinations", format!("{} {}", convert_coordinates(*point).unwrap(), address));
                        }
                        ui.hyperlink_to("Verify coordinates", url_from(*point));
                    });
                });
            }

            ui.separator();
            
                
            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                for coordinates in flight_plan_coordinates.clone().iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label((*coordinates).clone());
    
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            if ui.button("Remove").clicked() {
                                let index = flight_plan_coordinates.clone().iter().position(|x| *x == *coordinates).unwrap();
                                flight_plan_coordinates.remove(index);
                            }
                            if ui.button("Copy coordinates").clicked() {
                                let _ = &clipboard.set_text((*coordinates)[0..11].to_string().clone());
                            }
                            if ui.button("Copy all").clicked() {
                                let _ = &clipboard.set_text((*coordinates).to_string().clone());
                            }
                        });
                    });   
                }

                ui.horizontal(|ui| {
                    let mut text: String = Default::default();
                    for address in flight_plan_coordinates.clone().iter_mut() {
                        text.push_str(format!("{} ", address[0..11].to_string()).as_str());
                    }

                    if text != "".to_string() {
                        
                        if ui.button("Copy complete trip").clicked() {
                            let _ = &clipboard.set_text(text.clone());
                        }
                        ui.label(text);
                    }
                });
            });
        });
    }
}