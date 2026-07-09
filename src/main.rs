use iced::window::{Settings, icon};
use image::ImageFormat;
use particle3d::ui::app::App;

static APP_NAME: &str = "Particle 3D";
static VERSION: &str = "2.0.0";

fn main() -> iced::Result {
    let settings = Settings {
        icon: Some(
            icon::from_file_data(
                include_bytes!("./resources/icon.ico"),
                Some(ImageFormat::Ico),
            )
            .unwrap(),
        ),
        ..Default::default()
    };
    iced::application(new, App::update, App::view)
        .title(App::get_title)
        .theme(iced::Theme::Ferra)
        .window(settings)
        .run()
}

fn new() -> App {
    let mut app = App::default();
    app.title = format!("{} | {}", APP_NAME, VERSION);
    app
}
