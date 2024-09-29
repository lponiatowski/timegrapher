use crate::audio::io::{AudioStreamBuilder, Connector};
use crate::signal::utils;
use crate::ui::extras;
use eframe::egui::{emath::Vec2b, Align, ComboBox, Layout, Style, TextStyle, Visuals};
use eframe::{egui, App};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};
use std::sync::Arc;
use std::time::Duration;
use tokio::{spawn, sync::Mutex, task::JoinHandle};

pub struct TimeGrapherUi {
    process_error: extras::ProcessError,
    host: Connector,
    device: String,
    device_list: Vec<String>,
    audio_taskhanle: Option<JoinHandle<()>>,
    start_btn: bool,
    stop_btn: bool,
    clear_btn: bool,
    linedata: Arc<Mutex<Vec<(f64, f64)>>>,
    settings: extras::Settings,
}

impl TimeGrapherUi {
    pub fn new(mut cons: Vec<Connector>) -> Self {
        let host = cons.remove(0);
        let devices = host
            .list_device_names()
            .unwrap_or(vec!["Devices not found!".to_string()]);

        Self {
            process_error: extras::ProcessError::default(),
            host: host,
            device: devices[0].clone(),
            device_list: devices,
            audio_taskhanle: None,
            start_btn: true,
            stop_btn: false,
            clear_btn: true,
            linedata: Arc::new(Mutex::new(Vec::new())),
            settings: extras::Settings::default(),
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

                                        match AudioStreamBuilder::new(&self.host, &self.device) {
                                            Ok(streambuilder) => {
                                                match streambuilder.build() {
                                                    Ok(audiostream) => {
                                                        let data = Arc::clone(&self.linedata);
                                                        let duration =
                                                            self.settings.sample_size.clone();
                                                        self.audio_taskhanle =
                                                            Some(spawn(async move {
                                                                println!("Sampling initiated");
                                                                loop {
                                                                    let track = audiostream
                                                                        .get_track_by_duration(duration)
                                                                        .await;
                                                                    
                                                                    let track = utils::remove_mean(track);
                                                                    let track = utils::cutt_off(track, 0.005);
                                                                    let mut data =
                                                                        data.lock().await;
                                                                    *data = track.track;
                                                                    drop(data);
                                                                }
                                                            }));
                                                    }
                                                    Err(e) => {
                                                        self.process_error.rais(format!(
                                                            "Error While building stream: {:}",
                                                            e
                                                        ));
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                // rais error
                                                self.process_error.rais(format!(
                                                    "Error While building stream: {:}",
                                                    e
                                                ));
                                            }
                                        };
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

                            ui.horizontal(|ui| {
                                if ui
                                    .add_enabled(self.clear_btn, egui::Button::new("Clear data"))
                                    .clicked()
                                {
                                    self.linedata = Arc::new(Mutex::new(Vec::new()));
                                }
                                if ui.add(egui::Button::new("Settings")).clicked(){
                                    self.settings.open();
                                }
                            });
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
                                let gain = self.settings.gain;
                                let points: PlotPoints =
                                    data.iter().map(|&(t, v)| [t, gain*v]).collect();
                                let line = Line::new(points);

                                // set y axis bounds
                                let y_min: f64 = -1. * self.settings.y_limits;
                                let y_max: f64 = self.settings.y_limits;
                                let bounds = PlotBounds::from_min_max([0., y_min], [0., y_max]);
                                Plot::new("Audio signal")
                                    .view_aspect(2.0)
                                    .show(ui, |plot_ui| {
                                        plot_ui.set_plot_bounds(bounds);
                                        plot_ui.set_auto_bounds(Vec2b::new(true, false));
                                        plot_ui.line(line)
                                    });
                            }
                        });
                    },
                );
            });
        });

        // // Dialogues
        // Error
        let message = self.process_error.message().to_owned();
        egui::Window::new("Process Error")
            .open(&mut self.process_error.is_error_mut())
            .show(ctx, |ui| {
                ui.label(message);
            });

        if !self.process_error.is_error() {
            self.process_error.close();
        }

        // Settings
        let mut ytext = format!("{:.2}", self.settings.y_limits);
        let mut samplentext = format!("{:.2}", self.settings.sample_size);
        let mut gaintext = format!("{:.2}", self.settings.gain);

        egui::Window::new("Settings")
            .open(&mut self.settings.is_open_mut())
            .show(ctx, |ui| {
                ui.columns(2, |clo_ui| {
                    clo_ui[0].vertical(|ui| {
                        ui.label("Y limits:");
                        ui.label("Sample duration:");
                        ui.label("Gain:");
                    });

                    clo_ui[1].vertical(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut ytext)
                                .hint_text("Simetric limit On Y axis")
                                .desired_width(50.0),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut samplentext)
                                .hint_text("Simetric limit On Y axis")
                                .desired_width(50.0),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut gaintext)
                                .hint_text("Input gain")
                                .desired_width(50.0),
                        );
                    });

                });
            });
        self.settings.y_limits = extras::Settings::parse_f64(ytext);
        self.settings.sample_size = extras::Settings::parse_f64(samplentext);
        self.settings.gain = extras::Settings::parse_f64(gaintext);


        // Trigger repaint at regular intervals to keep the plot updating
        ctx.request_repaint();
    }
}
