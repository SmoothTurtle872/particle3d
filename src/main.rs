use iced::run;
use particle3d::ui::app::App;

static APP_NAME: &str = "Particle 3D";
static VERSION: &str = "2.0.0";

fn main() -> iced::Result {
    run(App::update, App::view)
}
