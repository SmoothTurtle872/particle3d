pub mod states {
    use super::app;
    use eframe::egui;

    impl app::App {
        pub fn unloaded(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label("No File Loaded");
                if ui.button("Load OBJ").clicked() {}
            });
        }
    }
}

pub mod app {
    use crate::cloud::*;
    use eframe::egui;
    #[derive(Default)]
    pub struct App {
        cloud: Option<ParticleCloud>,
    }

    impl App {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            Self::default()
        }
    }

    impl eframe::App for App {
        fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
            ui.vertical_centered(|ui| {
                ui.label("Particle 3D");
            });
            match &self.cloud {
                None => {
                    self.unloaded(ui, frame);
                }
                Some(cloud) => {}
            }
        }
    }
}
