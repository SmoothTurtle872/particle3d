pub mod app {
    use iced::{
        Element,
        widget::{button, center, checkbox, column, pick_list, row, text},
    };

    use iced_aw::NumberInput;
    use rfd::FileDialog;

    use crate::cloud::RotationSpace;

    use super::super::{
        cloud::ParticleCloud,
        visuals::particle::{Particle, ParticleGroupType},
    };
    #[derive(Default)]
    pub struct App {
        cloud: State,
        pub title: String,
        is_multi: bool,
        subdivide_edges: bool,
        edge_subdivisions: Option<i32>,
        rotation_space: RotationSpace,
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
                Message::SetParticle(particle) => {
                    if let State::Loaded(cloud) = &mut state.cloud {
                        cloud.particle = ParticleGroupType::Single(particle);
                    }
                }
                Message::SetSubdivideEdgeState(toggle) => {
                    state.update_edge_division_state(toggle);
                }
                Message::SetEdgeSubdivisions(divisions) => {
                    state.edge_subdivisions = Some(divisions)
                }
                Message::SetRotationSpace(space) => state.rotation_space = space,
                _ => {}
            }
        }

        fn update_edge_division_state(&mut self, state: bool) {
            self.subdivide_edges = state;
            if state {
                self.edge_subdivisions = Some(1)
            } else {
                self.edge_subdivisions = None
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
        SetSubdivideEdgeState(bool),
        SetEdgeSubdivisions(i32),
        SetRotationSpace(RotationSpace),
    }

    #[derive(Debug, Clone, Default)]
    pub enum State {
        #[default]
        Unloaded,
        Error,
        Loaded(ParticleCloud),
    }

    // UI - States
    impl App {
        fn unloaded(&self) -> Element<'_, Message> {
            center(self.load_button("Error Loading OBJ".to_string())).into()
        }
        fn loaded(&self, cloud: &ParticleCloud) -> Element<'_, Message> {
            let column = column![
                self.particle_selector(cloud),
                self.edge_subdividor(),
                self.rotation_selector()
            ]
            .spacing(10);
            column.into()
        }
        fn error(&self) -> Element<'_, Message> {
            center(self.load_button("Error Loading OBJ".to_string())).into()
        }
    }

    // UI - Widgets
    impl App {
        fn load_button(&self, info_message: String) -> Element<'_, Message> {
            column![
                text(info_message),
                button("Load OBJ").on_press(Message::Load)
            ]
            .spacing(10)
            .into()
        }
        fn particle_selector(&self, cloud: &ParticleCloud) -> Element<'_, Message> {
            let multi_object = cloud.obj.len() > 1;

            let mut inputs = if multi_object {
                column![
                    row![
                        "Individual Particles Per Object: ",
                        checkbox(self.is_multi).on_toggle(Message::ToggleParticleGroupMultiState)
                    ]
                    .spacing(10)
                ]
            } else {
                column![]
            };

            if self.is_multi && multi_object {
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
            } else {
                if let ParticleGroupType::Single(particle) = &cloud.particle {
                    inputs = inputs.push(row![
                        "Particle: ",
                        pick_list(
                            Particle::get_option_list(),
                            Some(particle.clone()),
                            Message::SetParticle
                        )
                    ]);
                }
            }

            inputs.spacing(10).into()
        }
        fn edge_subdividor(&self) -> Element<'_, Message> {
            let mut widget = column![
                row![
                    "Subdivide Edges: ",
                    checkbox(self.subdivide_edges).on_toggle(Message::SetSubdivideEdgeState)
                ]
                .spacing(10)
            ];
            if self.subdivide_edges
                && let Some(number) = self.edge_subdivisions
            {
                widget = widget.push(
                    row![
                        "Subdvisions: ",
                        NumberInput::new(&number, 1..=10, Message::SetEdgeSubdivisions)
                    ]
                    .spacing(10),
                );
            };
            widget.spacing(10).into()
        }
        fn rotation_selector(&self) -> Element<'_, Message> {
            row![
                "Rotation Space: ",
                pick_list(
                    [RotationSpace::Global, RotationSpace::Local],
                    Some(self.rotation_space.clone()),
                    Message::SetRotationSpace
                )
            ]
            .spacing(10)
            .into()
        }
    }
}
