pub mod app {
    use iced::{
        Element,
        widget::{button, center, column, text},
    };

    use super::super::cloud::ParticleCloud;
    #[derive(Default)]
    pub struct App {
        cloud: Option<ParticleCloud>,
    }

    impl App {
        pub fn view(state: &App) -> Element<'_, Message> {
            center(
                column![
                    text("No OBJ Loaded"),
                    button("Load OBJ").on_press(Message::Load)
                ]
                .spacing(10),
            )
            .into()
        }

        pub fn update(state: &mut App, message: Message) {}
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        Load,
    }
}
