use anyhow::Result;
use eframe;
use eframe::egui;
use simple_logger::SimpleLogger;
use timegrapher::audio::io as audioio;
use timegrapher::ui::app;
use log::{info,LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {

    SimpleLogger::new().with_module_level("eframe", LevelFilter::Warn).init().unwrap();
    info!("Starting process!");

    // get the list of connectors and devices
    let  cons = audioio::get_connectors()?;

    info!("Found Connectors {:?}", &cons);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900. as f32, 500. as f32]),
        ..Default::default()
    };

    info!("Loading main app");

    let _ = eframe::run_native(
        "Timegrapher",
        native_options,
        Box::new(|_| Ok(Box::new(app::TimeGrapherUi::new(cons)))),
    );

    info!("Adios!");
    Ok(())
}
