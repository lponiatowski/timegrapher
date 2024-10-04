use crate::audio::io::{AudioStreamBuilder, Connector};
use crate::audio::track::AudioTrack;
use crate::ui::extras;
use crate::ui::defs::*;
use crate::ui::executor::{spawn_executor, ExecutorCTL};


use eframe::egui::{emath::Vec2b, Align, ComboBox, Layout, Style, Visuals};
use eframe::{egui, App};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};
use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};

#[derive(PartialEq)]
pub enum ShowData {
    Raw,
    Processed,
}

pub struct TimeGrapherUi {
    process_error: extras::NewError,
    host: Connector,
    device: String,
    device_list: Vec<String>,
    audio_taskhanle: Option<JoinHandle<()>>,
    start_btn: bool,
    stop_btn: bool,
    clear_btn: bool,
    show_data_type: ShowData,
    rawdata: Arc<Mutex<AudioTrack>>,
    data: Arc<Mutex<AudioTrack>>,
    last_data: AudioTrack,
    audio_settings: extras::AudioSettings,
    plot_settings: extras::PlotSettings
}

impl TimeGrapherUi {
    pub fn new(mut cons: Vec<Connector>) -> Self {
        let host = cons.remove(0);
        let devices = host
            .list_device_names()
            .unwrap_or(vec!["Devices not found!".to_string()]);

        Self {
            process_error: extras::NewError::default(),
            host: host,
            device: devices[0].clone(),
            device_list: devices,
            audio_taskhanle: None,
            start_btn: true,
            stop_btn: false,
            clear_btn: true,
            show_data_type: ShowData::Processed,
            rawdata: Arc::new(Mutex::new(AudioTrack::new())),
            data: Arc::new(Mutex::new(AudioTrack::new())),
            last_data: AudioTrack::new(),
            audio_settings: extras::AudioSettings::default(),
            plot_settings: extras::PlotSettings::default(),
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
                                                        // executor
                                                        self.audio_taskhanle = spawn_executor(audiostream,
                                                            ExecutorCTL{
                                                                rawdata: Arc::clone(&self.rawdata),
                                                                data: Arc::clone(&self.data),
                                                                duration: self.audio_settings.sample_size.get_value().clone(),
                                                                use_denoiser: if *self.audio_settings.use_denoiser.get_value() { 1.into() } else { 0.into() },
                                                                noise_supr_level: self.audio_settings.noise_supr_level.get_value().clone(),
                                                                use_agc: if *self.audio_settings.use_agc.get_value() { 1.into() } else { 0.into() },
                                                                agc_level: self.audio_settings.agc_level.get_value().clone(),
                                                            }
                                                        );      
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
                                    self.rawdata = Arc::new(Mutex::new(AudioTrack::new()));
                                    self.data = Arc::new(Mutex::new(AudioTrack::new()));
                                }
                                if ui.add(egui::Button::new("Audio Settings")).clicked() {
                                    self.audio_settings.open();
                                }
                                if ui.add(egui::Button::new("Plot Settings")).clicked() {
                                    self.plot_settings.open();
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

                            let data = match self.show_data_type {
                                ShowData::Raw => {
                                    // here goes code for the unprocessd data
                                    if let Ok(data) = self.rawdata.try_lock() {
                                        data.to_owned()
                                    } else {
                                        self.last_data.clone()
                                    }
                                }
                                ShowData::Processed => {
                                    // here goes code for the processed data
                                    if let Ok(data) = self.data.try_lock() {
                                        data.to_owned()
                                    } else {
                                        self.last_data.clone()
                                    }
                                }
                            };

                            // transforme data into line
                            let points: PlotPoints =
                                data.track.iter().map(|&(t, v)| [t, v]).collect();
                            let line = Line::new(points);
                            // save for later 
                            self.last_data = data;

                            // set y axis bounds
                            let y_min: f64 = -1. * self.plot_settings.y_limit.get_value().clone();
                            let y_max: f64 = self.plot_settings.y_limit.get_value().clone();
                            let bounds = PlotBounds::from_min_max([0., y_min], [0., y_max]);
                            Plot::new("Audio signal")
                                .view_aspect(3.0)
                                .show(ui, |plot_ui| {
                                    plot_ui.set_plot_bounds(bounds);
                                    plot_ui.set_auto_bounds(Vec2b::new(true, false));
                                    plot_ui.line(line)
                                });

                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.radio_value(&mut self.show_data_type, ShowData::Raw, "Raw");
                                ui.add_space(10.0);
                                ui.radio_value(
                                    &mut self.show_data_type,
                                    ShowData::Processed,
                                    "Processed",
                                );
                            });

                            ui.add_space(20.);

                            // transforme data into line
                            let data = vec![(1.,1.),(2.,2.),(3.,3.)];
                            let points: PlotPoints =
                                data.iter().map(|&(t, v)| [t, v]).collect();
                            let line = Line::new(points);
                            Plot::new("Timegrapher")
                                .view_aspect(3.0)
                                .show(ui, |plot_ui| {
                                    plot_ui.set_auto_bounds(Vec2b::new(true, true));
                                    plot_ui.line(line)
                                });

                            ui.add_space(20.);
                        });
                    },
                );
            });
        });

        // // Dialogues
        // Error
        let message = self.process_error.get_message().to_owned();
        egui::Window::new("Process Error")
            .open(&mut self.process_error.is_error_mut())
            .show(ctx, |ui| {
                ui.label(message);
            });

        if !self.process_error.is_error() {
            self.process_error.close();
        }

        // Audio settings section
        let mut samplen_text = format!("{:.2}", self.audio_settings.sample_size.get_value());
        let mut use_denoiser  = self.audio_settings.use_denoiser.get_value().clone();
        let mut noise_supr_level_text  = format!("{:}", self.audio_settings.noise_supr_level.get_value());
        let mut use_agc = self.audio_settings.use_agc.get_value().clone();
        let mut agc_level_text = format!("{:}", self.audio_settings.agc_level.get_value());
        let mut is_open = self.audio_settings.is_open_mut();

        egui::Window::new("Audio Settings")
            .open(&mut is_open)
            .show(ctx, |ui| {
                ui.columns(2, |clo_ui| {
                    clo_ui[0].vertical(|ui| {
                        ui.label("Sample duration:");
                        ui.add_space(3.0);
                        ui.label("Use denoiser:");
                        ui.add_space(3.0);
                        ui.label("Noise suppression level");
                        ui.add_space(3.0);
                        ui.label("Use Auto.Gain.Contr.");
                        ui.add_space(3.0);
                        ui.label("A.G.C. level");
                    });

                    clo_ui[1].vertical(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut samplen_text)
                                .hint_text("Simetric limit On Y axis")
                                .desired_width(50.0),
                        );
                        ui.add(egui::Checkbox::new(&mut use_denoiser, ""));
                        ui.add(
                            egui::TextEdit::singleline(&mut noise_supr_level_text)
                                .hint_text("Input gain")
                                .desired_width(50.0),
                        );
                        ui.add(egui::Checkbox::new(&mut use_agc, ""));
                        ui.add(
                            egui::TextEdit::singleline(&mut agc_level_text)
                                .hint_text("Signal cutoff")
                                .desired_width(50.0),
                        );
                    });
                });
            });

        self.audio_settings.sample_size.parse(samplen_text);
        self.audio_settings.use_denoiser.update_value(use_denoiser);
        self.audio_settings.noise_supr_level.parse(noise_supr_level_text);
        self.audio_settings.use_agc.update_value(use_agc);
        self.audio_settings.agc_level.parse(agc_level_text);
    

        // Plot settings section
        let mut ytext = format!("{:.2}", self.plot_settings.y_limit.get_value());
        egui::Window::new("Plot Settings")
            .open(&mut self.plot_settings.is_open_mut())
            .show(ctx, |ui| {
                ui.columns(2, |clo_ui| {
                    clo_ui[0].vertical(|ui| {
                        ui.label("Y limits:");
                    });

                    clo_ui[1].vertical(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut ytext)
                                .hint_text("Simetric limit On Y axis")
                                .desired_width(50.0),
                        );
                    });
                });
            });
        self.plot_settings.y_limit.parse(ytext);

        // Trigger repaint at regular intervals to keep the plot updating
        ctx.request_repaint();
    }
}
