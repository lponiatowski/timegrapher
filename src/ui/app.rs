use crate::audio::io::{AudioStreamBuilder, Connector};
use eframe::egui::{emath::Vec2b, Align, ComboBox, Layout, Style, TextStyle, Visuals};
use eframe::{egui, App};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};
use std::sync::Arc;
use std::time::Duration;
use tokio::{spawn, sync::Mutex, task::JoinHandle};

pub struct TimeGrapherUi {
    host: Connector,
    device: String,
    device_list: Vec<String>,
    audio_taskhanle: Option<JoinHandle<()>>,
    audiolength: usize,
    start_btn: bool,
    stop_btn: bool,
    clear_btn: bool,
    linedata: Arc<Mutex<Vec<(f64, f64)>>>,
    y_min: String,
    y_max: String,
}

impl TimeGrapherUi {
    pub fn new(mut cons: Vec<Connector>) -> Self {
        let host = cons.remove(0);
        let devices = host
            .list_device_names()
            .unwrap_or(vec!["Devices not found!".to_string()]);

        Self {
            host: host,
            device: devices[0].clone(),
            device_list: devices,
            audio_taskhanle: None,
            audiolength: 3,
            start_btn: true,
            stop_btn: false,
            clear_btn: true,
            linedata: Arc::new(Mutex::new(Vec::new())),
            y_min: "-0.01".to_string(),
            y_max: "0.01".to_string(),
        }
    }
}

impl App for TimeGrapherUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply the updated style to the context and turn on dark mode
        ctx.set_style(Style {
            visuals: Visuals::dark(),
            ..Style::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Get the total available width for the UI
            let available_width = ui.available_width();

            // Calculate the left and right column widths
            let left_column_width = available_width * 0.30;
            let right_column_width = available_width * 0.70;

            ui.horizontal(|ui| {
                // Left column (30% width)
                ui.allocate_ui_with_layout(
                    egui::vec2(left_column_width, ui.available_height()),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.vertical(|ui| {
                            ui.label("select audio device: ");
                            ComboBox::new("Audio device:", "")
                                .selected_text(&self.device)
                                .show_ui(ui, |ui| {
                                    let _ = &self.device_list.iter().for_each(|dev| {
                                        ui.selectable_value(&mut self.device, dev.clone(), dev);
                                    });
                                });

                            ui.add_space(50.);

                            ui.horizontal(|ui| {
                                if ui
                                    .add_enabled(
                                        self.start_btn,
                                        egui::Button::new("Start sampling"),
                                    )
                                    .clicked()
                                {
                                    // toggle buttons
                                    self.stop_btn = true;
                                    self.start_btn = false;
                                    self.clear_btn = false;
                                    // start process if not present
                                    if self.audio_taskhanle.is_none() {
                                        // here goes the code that creates stream and every
                                        println!(
                                            "Creating audio stream on device {:}:{:}",
                                            &self.host, &self.device
                                        );
                                        let audiostream =
                                            AudioStreamBuilder::new(&self.host, &self.device)
                                                .unwrap()
                                                .build()
                                                .unwrap();
                                        let data = Arc::clone(&self.linedata);
                                        let duration = self.audiolength.clone() as u64;
                                        self.audio_taskhanle = Some(spawn(async move {
                                            println!("Sampling initiated");
                                            loop {
                                                let track = audiostream
                                                    .get_track_by_duration(Duration::from_secs(
                                                        duration,
                                                    ))
                                                    .await;
                                                // println!("{:?}", &track.get_volume()[1..10]);
                                                let mut data = data.lock().await;
                                                *data = track.track;
                                                drop(data);
                                                println!("Data ready")
                                            }
                                        }));
                                    };
                                }

                                if ui
                                    .add_enabled(self.stop_btn, egui::Button::new("Stop sampling"))
                                    .clicked()
                                {
                                    self.stop_btn = false;
                                    self.start_btn = true;
                                    self.clear_btn = true;
                                    if let Some(task) = &self.audio_taskhanle {
                                        task.abort();
                                        self.audio_taskhanle = None;
                                    }
                                }
                            });
                            if ui
                                .add_enabled(self.clear_btn, egui::Button::new("Clear data"))
                                .clicked()
                            {
                                self.linedata = Arc::new(Mutex::new(Vec::new()));
                            }
                        });
                    },
                );

                // Right column 70%
                ui.allocate_ui_with_layout(
                    egui::vec2(right_column_width, ui.available_height()),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(20.0);

                            // check if data is ready
                            if let Ok(data) = self.linedata.try_lock() {
                                // transforme data into line
                                let points: PlotPoints =
                                    data.iter().map(|&(t, v)| [t, v]).collect();
                                let line = Line::new(points);

                                // set y axis bounds
                                let y_min: f64 = self.y_min.clone().parse().unwrap_or(-1.);
                                let y_max: f64 = self.y_max.clone().parse().unwrap_or(1.);
                                let bounds = PlotBounds::from_min_max([0., y_min], [0., y_max]);
                                Plot::new("Audio signal")
                                    .view_aspect(2.0)
                                    .show(ui, |plot_ui| {
                                        plot_ui.set_plot_bounds(bounds);
                                        plot_ui.set_auto_bounds(Vec2b::new(true, false));
                                        plot_ui.line(line)
                                    });
                            }
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.label("Set Y limits:");

                                ui.add_space(20.);

                                ui.label("min: ");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.y_min)
                                        .hint_text("Minimum Volume")
                                        .desired_width(20.0),
                                );

                                ui.add_space(20.);
                                ui.label("max: ");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.y_max)
                                        .hint_text("Maximum Volume")
                                        .desired_width(20.0),
                                );
                            });
                        });
                    },
                );
            });
        });

        // Trigger repaint at regular intervals to keep the plot updating
        ctx.request_repaint();
    }
}
