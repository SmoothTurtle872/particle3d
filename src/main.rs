use eframe::egui;

use particle3d::cloud::ParticleCloud;

static APP_NAME: &str = "Particle 3D";
static VERSION: &str = "2.0.0";

fn main() {
    let native_options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        &format!("{} | {}", APP_NAME, VERSION),
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[derive(Default)]
struct App {
    cloud: Option<ParticleCloud>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}
