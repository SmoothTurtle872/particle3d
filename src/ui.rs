pub mod app {
    use iced::{
        Element,
        widget::{bottom, button, center, center_x, checkbox, column, pick_list, row, text},
    };

    use iced_aw::NumberInput;
    use rfd::FileDialog;
    use std::fs;

    use crate::{cloud::RotationSpace, particle::ParticleSetting};

    use super::{
        super::{
            cloud::ParticleCloud,
            visuals::particle::{Particle, ParticleGroupType},
        },
        widgets::*,
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
                            list[id].1 = ParticleSetting::from_particle(&particle);
                            list[id].0 = particle;
                        }
                    }
                }
                Message::SetParticle(particle) => {
                    if let State::Loaded(cloud) = &mut state.cloud {
                        let setting = ParticleSetting::from_particle(&particle);
                        cloud.particle = ParticleGroupType::Single(particle, setting);
                    }
                }
                Message::SetSubdivideEdgeState(toggle) => {
                    state.update_edge_division_state(toggle);
                }
                Message::SetEdgeSubdivisions(divisions) => {
                    state.edge_subdivisions = Some(divisions)
                }
                Message::SetRotationSpace(space) => state.rotation_space = space,
                Message::Export => state.export(),
                Message::SetParticleSetting(setting) => {
                    if let State::Loaded(cloud) = &mut state.cloud {
                        if let ParticleGroupType::Single(particle, _) = &cloud.particle {
                            cloud.particle = ParticleGroupType::Single(particle.clone(), setting);
                        }
                    }
                }
                Message::SetParticleSettingInList(idx, setting) => {
                    if let State::Loaded(cloud) = &mut state.cloud {
                        if let ParticleGroupType::Multi(group) = &mut cloud.particle {
                            group[idx] = (group[idx].0.clone(), setting);
                        }
                    }
                }
            }
        }

        fn export(&mut self) {
            if let State::Loaded(cloud) = &mut self.cloud {
                cloud.cache_points(self.subdivide_edges, self.edge_subdivisions);

                let export_path = rfd::FileDialog::new()
                    .add_filter("Mcfunction File", &["mcfunction"])
                    .set_can_create_directories(true)
                    .set_directory("/")
                    .set_title("Export to MCfunction")
                    .save_file();

                if let Some(path) = export_path {
                    let output = cloud.output(self.rotation_space.clone());
                    if let Ok(data) = output {
                        fs::write(path, data);
                    }
                }
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
                    let mut particles: Vec<(Particle, ParticleSetting)> = vec![];
                    for _ in 0..cloud.obj.len() {
                        particles.push((Particle::AngryVillager, ParticleSetting::None));
                    }
                    cloud.particle = ParticleGroupType::Multi(particles);
                } else {
                    cloud.particle =
                        ParticleGroupType::Single(Particle::AngryVillager, ParticleSetting::None);
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
                        ParticleGroupType::Single(Particle::AngryVillager, ParticleSetting::None),
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
        SetParticleSetting(ParticleSetting),
        SetParticleSettingInList(usize, ParticleSetting),
        SetSubdivideEdgeState(bool),
        SetEdgeSubdivisions(i32),
        SetRotationSpace(RotationSpace),
        Export,
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
            center(self.load_button("No OBJ File Loaded".to_string(), false)).into()
        }
        fn loaded(&self, cloud: &ParticleCloud) -> Element<'_, Message> {
            let column = column![
                self.particle_selector(cloud),
                self.edge_subdividor(),
                self.rotation_selector()
            ]
            .spacing(10);
            column![
                center(column),
                center_x(bottom(
                    row![
                        self.load_button("Change OBJ File: ".to_string(), true),
                        " | ",
                        row![
                            "Export to mcfunction: ",
                            button("Export").on_press(Message::Export)
                        ]
                        .spacing(10)
                    ]
                    .spacing(10)
                ))
            ]
            .into()
        }
        fn error(&self) -> Element<'_, Message> {
            center(self.load_button("Error Loading OBJ".to_string(), false)).into()
        }
    }

    // UI - Widgets
    impl App {
        fn load_button(&self, info_message: String, inline: bool) -> Element<'_, Message> {
            if !inline {
                column![
                    text(info_message),
                    button("Load OBJ").on_press(Message::Load)
                ]
                .spacing(10)
                .into()
            } else {
                row![
                    text(info_message),
                    button("Load OBJ").on_press(Message::Load)
                ]
                .spacing(10)
                .into()
            }
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
                        inputs = inputs.push(particle_selector(
                            particles[idx].0.clone(),
                            particles[idx].1.clone(),
                            Some(idx),
                        ));
                    }
                }
            } else {
                if let ParticleGroupType::Single(particle, setting) = &cloud.particle {
                    inputs =
                        inputs.push(particle_selector(particle.clone(), setting.clone(), None));
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

mod widgets {
    use super::app::Message;
    use iced::{
        Element,
        widget::{
            bottom, button, center, center_x, checkbox, column, combo_box, container, pick_list,
            row, text, text_input,
        },
    };

    use iced_aw::{ColorPicker, NumberInput};

    use crate::particle::{Particle, ParticleSetting};

    pub fn particle_selector(
        particle: Particle,
        setting: ParticleSetting,
        index: Option<usize>,
    ) -> Element<'static, Message> {
        match index {
            None => {
                let mut setter = column![
                    row![
                        "Particle: ",
                        pick_list(
                            Particle::get_option_list(),
                            Some(particle.clone()),
                            Message::SetParticle
                        )
                    ]
                    .spacing(10)
                ];
                match setting {
                    ParticleSetting::None => {}
                    ParticleSetting::Other(value) => {
                        setter = setter.push(row![
                            "Particle: ",
                            text_input("custom_particle", &value)
                                .on_input(|value| {
                                    Message::SetParticleSetting(ParticleSetting::Other(
                                        value.to_string(),
                                    ))
                                })
                                .width(200)
                        ]);
                    }
                    ParticleSetting::SingleColorSized(color, size) => {
                        setter = setter.push(row![
                            "Color: ",
                            "Size: ",
                            NumberInput::new(&size, 0.01..5.0, move |num| {
                                Message::SetParticleSetting(ParticleSetting::SingleColorSized(
                                    color, num,
                                ))
                            })
                        ]);
                    }
                    ParticleSetting::SingleColorTransp(color) => {
                        setter = setter.push(row!["Color: "]);
                    }
                    ParticleSetting::SingleColor(color) => {
                        setter = setter.push(row!["Color: "]);
                    }
                }
                setter.spacing(10).into()
            }
            Some(idx) => {
                let mut setter = column![
                    row!["Object", text(idx)].spacing(10),
                    row![
                        "Particle: ",
                        pick_list(
                            Particle::get_option_list(),
                            Some(particle.clone()),
                            move |particle| { Message::SetParticleInList(idx, particle) }
                        )
                    ]
                ];
                match setting {
                    ParticleSetting::None => {}
                    ParticleSetting::Other(value) => {
                        setter = setter.push(row![
                            "Particle: ",
                            text_input("custom_particle", &value)
                                .on_input(move |value| {
                                    Message::SetParticleSettingInList(
                                        idx,
                                        ParticleSetting::Other(value.to_string()),
                                    )
                                })
                                .width(200)
                        ]);
                    }
                    ParticleSetting::SingleColorSized(color, size) => {
                        setter = setter.push(row![
                            "Color: ",
                            "Size: ",
                            NumberInput::new(&size, 0.01..5.0, move |num| {
                                Message::SetParticleSettingInList(
                                    idx,
                                    ParticleSetting::SingleColorSized(color, num),
                                )
                            })
                        ]);
                    }
                    ParticleSetting::SingleColorTransp(color) => {
                        setter = setter.push(row!["Color: "]);
                    }
                    ParticleSetting::SingleColor(color) => {
                        setter = setter.push(row!["Color: "]);
                    }
                }
                setter.into()
            }
        }
    }
}
