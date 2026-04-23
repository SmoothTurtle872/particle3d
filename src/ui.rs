pub mod app {
    use iced::{
        Element,
        widget::{button, center, column, text},
    };

    use rfd::FileDialog;

    use super::super::{
        cloud::ParticleCloud,
        visuals::particle::{Particle, ParticleGroupType},
    };
    #[derive(Default)]
    pub struct App {
        cloud: State,
    }

    impl App {
        pub fn view(state: &App) -> Element<'_, Message> {
            match &state.cloud {
                State::Unloaded => state.unloaded(),
                State::Error => state.error(),
                State::Loaded(cloud) => {
                    todo!()
                }
            }
        }

        pub fn update(state: &mut App, message: Message) {
            match message {
                Message::Load => {
                    state.load();
                }
            }
        }

        fn unloaded(&self) -> Element<'_, Message> {
            center(
                column![
                    text("No OBJ Loaded"),
                    button("Load OBJ").on_press(Message::Load)
                ]
                .spacing(10),
            )
            .into()
        }

        fn error(&self) -> Element<'_, Message> {
            center(
                column![
                    text("Error Loading File"),
                    button("Load OBJ").on_press(Message::Load)
                ]
                .spacing(10),
            )
            .into()
        }

        fn load(&mut self) {
            let files = FileDialog::new()
                .add_filter("Wavefront Files", &["obj"])
                .pick_file();

            match files {
                Some(path) => {
                    let cloud_loader = ParticleCloud::new(
                        path,
                        ParticleGroupType::Single(Particle::AngryVillager),
                    );
                    match cloud_loader {
                        Ok(cloud) => self.cloud = State::Loaded(cloud),
                        Err(_) => self.cloud = State::Error,
                    }
                }
                None => {}
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        Load,
    }

    #[derive(Debug, Clone, Default)]
    pub enum State {
        #[default]
        Unloaded,
        Error,
        Loaded(ParticleCloud),
    }
}
