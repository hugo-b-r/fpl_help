use std::{env, process};
use fpl_help::{Config, FPL, get_coordinates_list, get_coordinates};
use eframe::egui;


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

    println!( "{}", get_coordinates_list(file).unwrap() );

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("FPL Help", native_options, Box::new(|cc| Box::new(FPLHelp::new(cc))));

}

#[derive(Default)]
struct FPLHelp {
    address: String,
    location_url: String,
    coordinates: String,
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
        let Self { address, location_url, coordinates } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Add an address");
            egui::menu::bar(ui, |ui| {
                ui.text_edit_singleline(address);
                if ui.button("Convert").clicked() {
                    *coordinates = get_coordinates(address.clone()).unwrap_or_else(|err| {
                        eprintln!("Error when geocoding: {}", err);
                        "Not a valid address".to_string()
                    });
                };
                ui.label(coordinates.as_str());
           });
           ui.hyperlink_to("Verify", location_url.as_str());
       });
   }
}