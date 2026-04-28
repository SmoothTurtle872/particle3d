pub mod app {
    use iced::{
        Element,
        widget::{button, center, checkbox, column, pick_list, row, text},
    };

    use rfd::FileDialog;

    use super::super::{
        cloud::ParticleCloud,
        visuals::particle::{Particle, ParticleGroupType},
    };
    #[derive(Default)]
    pub struct App {
        cloud: State,
        pub title: String,
        is_multi: bool,
    }

    impl App {
        pub fn get_title(state: &App) -> String {
            state.title.clone()
        }

        pub fn view(state: &App) -> Element<'_, Message> {
            match &state.cloud {
                State::Unloaded => state.unloaded(),
                State::Error => state.error(),
                State::Loaded(cloud) => state.loaded(cloud),
            }
        }

        pub fn update(state: &mut App, message: Message) {
            match message {
                Message::Load => {
                    state.load();
                }
                Message::ToggleParticleGroupMultiState(toggle) => state.update_multi(toggle),
                Message::SetParticleInList(id, particle) => {
                    if let State::Loaded(cloud) = &mut state.cloud {
                        if let ParticleGroupType::Multi(list) = &mut cloud.particle {
                            list[id] = particle;
                        }
                    }
                }
                _ => {}
            }
        }

        fn update_multi(&mut self, state: bool) {
            self.is_multi = state;
            if let State::Loaded(cloud) = &mut self.cloud {
                if state {
                    let mut particles: Vec<Particle> = vec![];
                    for _ in 0..cloud.obj.len() {
                        particles.push(Particle::AngryVillager);
                    }
                    cloud.particle = ParticleGroupType::Multi(particles);
                } else {
                    cloud.particle = ParticleGroupType::Single(Particle::AngryVillager);
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

        fn loaded(&self, cloud: &ParticleCloud) -> Element<'_, Message> {
            let multi_object = cloud.obj.len() > 1;

            let mut particle_settings = if multi_object {
                column![row![
                    "Individual Particles Per Object:   ",
                    checkbox(self.is_multi).on_toggle(Message::ToggleParticleGroupMultiState),
                ]]
            } else {
                column![row!["Particle Input"]]
            };

            if multi_object && self.is_multi {
                let mut inputs = column![];
                if let ParticleGroupType::Multi(particles) = &cloud.particle {
                    for (idx, _) in cloud.obj.iter().enumerate() {
                        inputs = inputs.push(row![
                            text(format!("Object {idx}: ")),
                            pick_list(
                                Particle::get_option_list(),
                                Some(particles[idx].clone()),
                                move |particle| { Message::SetParticleInList(idx, particle) }
                            )
                        ]);
                    }
                }
                particle_settings = particle_settings.push(inputs);
            } else {
                particle_settings = particle_settings.push(row!["Particle Input"])
            }

            let column = column![particle_settings];

            column.into()
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
        ToggleParticleGroupMultiState(bool),
        SetParticle(Particle),
        SetParticleInList(usize, Particle),
    }

    #[derive(Debug, Clone, Default)]
    pub enum State {
        #[default]
        Unloaded,
        Error,
        Loaded(ParticleCloud),
    }
}
