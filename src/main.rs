use std::{env, process};
use fpl_help::{Config, FPL, get_list_coordinates_list, get_coordinates, convert_coordinates, url_from};
use eframe::egui;
use geocoding::Point;
use arboard::Clipboard;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem found when getting arguments: {}", err);
        process::exit(1);
    });
    let file = FPL::new(config).unwrap_or_else(|err| {
        eprintln!("Problem found when loading file: {}", err);
        process::exit(1);
    });

    println!( "{}", get_list_coordinates_list(file).unwrap() );

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 700.0)),
        min_window_size: Some(egui::vec2(480.0, 300.0)),
        ..Default::default()
    };
    let _ = eframe::run_native("FPL Help", native_options, Box::new(|cc| Box::new(FPLHelp::new(cc))));

}

struct FPLHelp {
    clipboard: Clipboard,
    address: String,
    coordinates: Vec<Point<f64>>,
    error: String,
}

impl Default for FPLHelp {
    fn default() -> FPLHelp {
        FPLHelp {
            clipboard: Clipboard::new().unwrap(),
            address: String::default(),
            coordinates: Vec::new(),
            error: String::default(),
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
            error
        } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Add an address");
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
                    if ui.button("Copy").clicked() {
                        let _ = &clipboard.set_text(convert_coordinates(*point).unwrap());
                        println!("copy {} to clipboard", convert_coordinates(*point).unwrap());
                    }
                    ui.hyperlink_to("Verify coordinates", url_from(*point));
                });
            }
        });
    }
}