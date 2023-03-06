use std::{env, process};
use fpl_help::{Config, FPL, get_list_coordinates_list, get_coordinates, convert_coordinates, url_from};
use eframe::egui;
use geocoding::Point;


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
        ..Default::default()
    };
    eframe::run_native("FPL Help", native_options, Box::new(|cc| Box::new(FPLHelp::new(cc))));

}

struct FPLHelp {
    address: String,
    location_url: String,
    coordinates: Vec<Point<f64>>,
}

impl Default for FPLHelp {
    fn default() -> FPLHelp {
        FPLHelp {
            address: String::default(),
            location_url: String::default(),
            coordinates: Vec::new(),
        }
    }
}

impl FPLHelp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
        frame: &mut eframe::Frame,
    ) {
        let Self {
            address,
            location_url,
            coordinates 
        } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Add an address");
            egui::menu::bar(ui, |ui| {
                ui.text_edit_singleline(address);
                if ui.button("Convert").clicked() {
                    *coordinates = get_coordinates(address.clone()).unwrap_or_else(|err| {
                        eprintln!("Error when geocoding: {}", err);
                        ui.label(format!("error when geocoding: {}", err));
                        Vec::new()
                    });
                };
                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
            for point in coordinates.iter() {
                ui.horizontal(|ui| {
                    ui.label(convert_coordinates(*point).unwrap());
                    if ui.button("Copy").clicked() {
                        println!("copy {} to clipboard", convert_coordinates(*point).unwrap());
                    }
                    ui.hyperlink_to("Verify coordinates", url_from(*point));
                });
            }
           // some sort of for address in coordinatespatattit a line with the coorinates, a copy button and the real address
       });
   }
}