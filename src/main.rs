use anyhow::Result;
use eframe;
use eframe::egui;
// use plotly::{common::Mode, Layout, Plot, Scatter};
use timegrapher::audio::io as audioio;
use timegrapher::ui::app;

#[tokio::main]
async fn main() -> Result<()> {

    // get the list of connectors and devices
    let  cons = audioio::get_connectors()?;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900. as f32, 500. as f32]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Timegrapher",
        native_options,
        Box::new(|_| Ok(Box::new(app::TimeGrapherUi::new(cons)))),
    );

    Ok(())
}
