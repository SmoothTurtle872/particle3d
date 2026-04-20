use particle3d::gui::app::App;

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
