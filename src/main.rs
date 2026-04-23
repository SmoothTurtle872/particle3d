use particle3d::ui::app::App;

static APP_NAME: &str = "Particle 3D";
static VERSION: &str = "2.0.0";

fn main() -> iced::Result {
    iced::application(new, App::update, App::view)
        .title(App::get_title)
        .theme(iced::Theme::Ferra)
        .run()
}

fn new() -> App {
    let mut app = App::default();
    app.title = format!("{} | {}", APP_NAME, VERSION);
    app
}
