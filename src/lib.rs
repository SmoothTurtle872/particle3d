pub mod visuals {

    use color_art::Color;

    pub fn parse_mc_color(color: &Color, transparency: bool) -> u32 {
        let r = (color.red() as u32) << 16;
        let g = (color.green() as u32) << 8;
        let b = color.blue() as u32;
        let opaque = r + b + g;
        if !transparency {
            return opaque;
        } else {
            let a = ((color.alpha() * 255.0) as u32) << 24;
            let color = opaque + a;
            return color;
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Point(pub f32, pub f32, pub f32);

    #[derive(Debug, Clone, PartialEq)]
    pub enum Particle {
        AngryVillager,
        Ash,
        Bubble,
        BubbleColumnUp,
        BubblePop,
        CampfireCosySmoke,
        CampfireSginalSmoke,
        CherryLeaves,
        Cloud,
        Composter,
        CopperFireFlame,
        CrimsonSpore,
        Crit,
        CurrentDown,
        DamageIndicator,
        Dolphin,
        DragonBreath,
        DrippingDripstoneLava,
        DrippingDripstoneWater,
        DrippingHoney,
        DrippingLava,
        DrippingObsidianTear,
        DrippingWater,
        Dust(Color, bool),
        Effect(Color),
        EggCrack,
        ElderGuardian,
        ElectricSpark,
        Enchant,
        EndRod,
        EntityEffect(Color),
        Explosion,
        ExplosionEmitter,
        FallingDripstoneLava,
        FallingDripstoneWater,
        FallingHoney,
        FallingLava,
        FallingNectar,
        FallingObsidianTear,
        FallingSporeBlossom,
        FallingWater,
        Firefly,
        Fishing,
        Flame,
        Flash(Color),
        Glow,
        GlowSquidInk,
        Gust,
        GustEmitter,
        HappyVillager,
        Heart,
        Infested,
        ItemCobweb,
        ItemSlime,
        ItemSnowball,
        LandingHoney,
        LandingLava,
        LandingObsidianTear,
        LargeSmoke,
        Lava,
        Mycelium,
        Nautilus,
        OminousSpawning,
        PaleOakLeaves,
        PauseMobGrowth,
        Poof,
        Portal,
        RaidOmen,
        ResetMobGrowth,
        Rain,
        ReversePortal,
        Scrape,
        SculkChargePop,
        SculkSoul,
        Shriek,
        SmallFlame,
        SmallGust,
        Smoke,
        Sneeze,
        Snowflake,
        SonicBoom,
        Soul,
        SoulFireFlame,
        Spit,
        Splash,
        SporeBlossomAir,
        SquidInk,
        SweepAttack,
        TotemOfUndying,
        TrialOmen,
        TrialSpawnerDetection,
        TrialSpawnerDetectionOminous,
        Underwater,
        VaultConnection,
        WarpedSpore,
        WaxOff,
        WaxOn,
        WhiteAsh,
        WhiteSmoke,
        Witch,
        Other(String),
    }

    impl std::fmt::Display for Particle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                // ---------------------custom--------------------- //
                Self::Other(v) => write!(f, "{}", v),
                Self::Dust(c, b) => {
                    if *b {
                        write!(f, "dust{{color:{},scale:1.0}}", parse_mc_color(c, false))
                    } else {
                        write!(
                            f,
                            "dust{{color:{},scale:{}}}",
                            parse_mc_color(c, false),
                            c.alpha()
                        )
                    }
                }
                Self::Effect(c) => {
                    write!(f, "effect{{color:{}}}", parse_mc_color(c, false))
                }
                Self::EntityEffect(c) => {
                    write!(f, "entity_effect{{color:{}}}", parse_mc_color(c, true))
                }
                Self::Flash(c) => write!(f, "flash{{color:{}}}", parse_mc_color(c, true)),
                // ----------------------standard------------------- //
                Self::AngryVillager => write!(f, "angry_villager"),
                Self::Ash => write!(f, "ash"),
                Self::Bubble => write!(f, "bubble"),
                Self::BubbleColumnUp => write!(f, "bubble_column_up"),
                Self::BubblePop => write!(f, "bubble_pop"),
                Self::CampfireCosySmoke => write!(f, "campfire_cosy_smoke"),
                Self::CampfireSginalSmoke => write!(f, "campfire_signal_smoke"),
                Self::CherryLeaves => write!(f, "cherry_smoke"),
                Self::Cloud => write!(f, "cloud"),
                Self::Composter => write!(f, "composter"),
                Self::CopperFireFlame => write!(f, "copper_fire_flame"),
                Self::CrimsonSpore => write!(f, "crimson_spore"),
                Self::Crit => write!(f, "crit"),
                Self::CurrentDown => write!(f, "current_down"),
                Self::DamageIndicator => write!(f, "damage_indicator"),
                Self::Dolphin => write!(f, "dolphin"),
                Self::DragonBreath => write!(f, "dragon_breath"),
                Self::DrippingDripstoneLava => write!(f, "dripping_dripstone_lava"),
                Self::DrippingDripstoneWater => write!(f, "dripping_dripstone_water"),
                Self::DrippingHoney => write!(f, "dripstone_honey"),
                Self::DrippingLava => write!(f, "dripstone_lava"),
                Self::DrippingObsidianTear => write!(f, "dripstone_obsidian_tear"),
                Self::DrippingWater => write!(f, "dripstone_water"),
                Self::EggCrack => write!(f, "egg_crack"),
                Self::ElderGuardian => write!(f, "elder_guardian"),
                Self::ElectricSpark => write!(f, "electric_spark"),
                Self::Enchant => write!(f, "enchant"),
                Self::EndRod => write!(f, "end_rod"),
                Self::Explosion => write!(f, "explosion"),
                Self::ExplosionEmitter => write!(f, "explosion_emitter"),
                Self::FallingDripstoneLava => write!(f, "falling_dripstone_lava"),
                Self::FallingDripstoneWater => write!(f, "falling_dripstone_water"),
                Self::FallingHoney => write!(f, "falling_honey"),
                Self::FallingLava => write!(f, "falling_lava"),
                Self::FallingNectar => write!(f, "falling_nectar"),
                Self::FallingObsidianTear => write!(f, "falling_obsidian_tear"),
                Self::FallingSporeBlossom => write!(f, "falling_spore_blossom"),
                Self::FallingWater => write!(f, "falling_water"),
                Self::Firefly => write!(f, "firefly"),
                Self::Fishing => write!(f, "fishing"),
                Self::Flame => write!(f, "flame"),
                Self::Glow => write!(f, "glow"),
                Self::GlowSquidInk => write!(f, "glow_squid_ink"),
                Self::Gust => write!(f, "gust"),
                Self::GustEmitter => write!(f, "gust_emitter"),
                Self::HappyVillager => write!(f, "happy_villager"),
                Self::Heart => write!(f, "heart"),
                Self::Infested => write!(f, "infested"),
                Self::ItemCobweb => write!(f, "item_cobweb"),
                Self::ItemSlime => write!(f, "item_slime"),
                Self::ItemSnowball => write!(f, "item_snowball"),
                Self::LandingHoney => write!(f, "landing_honey"),
                Self::LandingLava => write!(f, "landing_lava"),
                Self::LandingObsidianTear => write!(f, "landing_obsidian_tear"),
                Self::LargeSmoke => write!(f, "large_smoke"),
                Self::Lava => write!(f, "lava"),
                Self::Mycelium => write!(f, "mycelium"),
                Self::Nautilus => write!(f, "nautilus"),
                Self::OminousSpawning => write!(f, "ominous_spawning"),
                Self::PaleOakLeaves => write!(f, "pale_oak_leaves"),
                Self::PauseMobGrowth => write!(f, "pause_mob_growth"),
                Self::Poof => write!(f, "poof"),
                Self::Portal => write!(f, "portal"),
                Self::RaidOmen => write!(f, "raid_omen"),
                Self::Rain => write!(f, "rain"),
                Self::ResetMobGrowth => write!(f, "reset_mob_growth"),
                Self::ReversePortal => write!(f, "reverse_portal"),
                Self::Scrape => write!(f, "scrape"),
                Self::SculkChargePop => write!(f, "sculk_charge_pop"),
                Self::SculkSoul => write!(f, "sculk_soul"),
                Self::Shriek => write!(f, "shriek"),
                Self::SmallFlame => write!(f, "small_flame"),
                Self::SmallGust => write!(f, "small_gust"),
                Self::Smoke => write!(f, "smoke"),
                Self::Sneeze => write!(f, "sneeze"),
                Self::Snowflake => write!(f, "snow_flake"),
                Self::SonicBoom => write!(f, "sonic_boom"),
                Self::Soul => write!(f, "soul"),
                Self::SoulFireFlame => write!(f, "soul_fire_flame"),
                Self::Spit => write!(f, "spit"),
                Self::Splash => write!(f, "splash"),
                Self::SporeBlossomAir => write!(f, "spore_blossom_air"),
                Self::SquidInk => write!(f, "squid_ink"),
                Self::SweepAttack => write!(f, "sweep_attack"),
                Self::TotemOfUndying => write!(f, "totem_of_undying"),
                Self::TrialOmen => write!(f, "trial_omen"),
                Self::TrialSpawnerDetection => write!(f, "trial_spawner_detection"),
                Self::TrialSpawnerDetectionOminous => write!(f, "trial_spawner_detection_ominous"),
                Self::Underwater => write!(f, "underwater"),
                Self::VaultConnection => write!(f, "vault_connection"),
                Self::WarpedSpore => write!(f, "warped_spore"),
                Self::WaxOff => write!(f, "wax_off"),
                Self::WaxOn => write!(f, "wax_on"),
                Self::WhiteAsh => write!(f, "white_ash"),
                Self::WhiteSmoke => write!(f, "white_smoke"),
                Self::Witch => write!(f, "witch"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ParticleGroupType {
        Single(Particle),
        Multi(Vec<Particle>),
    }
}

pub mod loader {
    use std::{fs, path::Path};

    use crate::visuals::ParticleGroupType;

    use wavefront_obj::{
        ParseError,
        obj::{Geometry, Object, Primitive, Vertex, parse},
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Edge {
        pub object: usize,
        pub start: usize,
        pub end: usize,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ParticleCloud {
        pub obj: Vec<Object>,
        pub particle: ParticleGroupType,
    }

    impl ParticleCloud {
        pub fn new<T: AsRef<Path>>(
            path: T,
            particle: ParticleGroupType,
        ) -> Result<ParticleCloud, ParseError> {
            let file = fs::read_to_string(path).unwrap();
            let model = parse(file);
            match model {
                Ok(obj) => Ok(ParticleCloud {
                    obj: obj.objects,
                    particle,
                }),
                Err(err) => Err(err),
            }
        }

        pub fn get_flattened_vertex_list(&self) -> Vec<&Vertex> {
            let mut vertecies: Vec<&Vertex> = vec![];
            for object in &self.obj {
                for vertex in &object.vertices {
                    vertecies.push(vertex);
                }
            }
            return vertecies;
        }

        pub fn get_object_vertex_list(&self) -> Vec<Vec<&Vertex>> {
            let mut groups: Vec<Vec<&Vertex>> = vec![];
            for object in &self.obj {
                let mut verticies: Vec<&Vertex> = vec![];
                for vertex in &object.vertices {
                    verticies.push(vertex);
                }
                groups.push(verticies);
            }
            return groups;
        }

        pub fn get_edges(&self) -> Vec<Edge> {
            let mut edges: Vec<Edge> = vec![];

            for (idx, object) in self.obj.iter().enumerate() {
                for geo in &object.geometry {
                    for shape in &geo.shapes {
                        match shape.primitive {
                            Primitive::Triangle(a, b, c) => {
                                edges.push(Edge {
                                    object: idx,
                                    start: a.0,
                                    end: b.0,
                                });
                                edges.push(Edge {
                                    object: idx,
                                    start: a.0,
                                    end: c.0,
                                });
                                edges.push(Edge {
                                    object: idx,
                                    start: b.0,
                                    end: c.0,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }

            edges
        }

        pub fn output(&self) -> String {
            let mut output: String = String::from(
                "# File Generated with Particle3D - https://github.com/SmoothTurtle872/particle3d \n",
            );

            match &self.particle {
                ParticleGroupType::Single(particle) => {
                    let verts = self.get_flattened_vertex_list();
                    for vert in verts {
                        let line = format!(
                            "particle {} ~{} ~{} ~{}\n",
                            particle, vert.x, vert.y, vert.z
                        );
                        output = output + &line;
                    }
                }
                ParticleGroupType::Multi(particles) => {
                    let groups = self.get_object_vertex_list();
                    for (particle, verts) in particles.iter().zip(groups.iter()) {
                        for vert in verts {
                            let line = format!(
                                "particle {} ~{} ~{} ~{}\n",
                                particle, vert.x, vert.y, vert.z
                            );
                            output = output + &line;
                        }
                    }
                }
            }

            return output;
        }
    }
}

#[cfg(test)]
mod tests {

    use color_art::Color;
    use wavefront_obj::obj::{self, Primitive};

    use super::{loader::*, visuals::*};
    #[test]
    fn object_count() {
        let count = ParticleCloud::new(
            "test-object.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap()
        .obj
        .len();

        assert_eq!(count, 1);

        let multi_count = ParticleCloud::new(
            "test-object-multi.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap()
        .obj
        .len();

        assert_eq!(multi_count, 2);
    }

    #[test]
    fn vertex_count() {
        let count = ParticleCloud::new(
            "test-object.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap()
        .get_flattened_vertex_list()
        .len();

        assert_eq!(count, 507);

        let count = ParticleCloud::new(
            "test-object-multi.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap()
        .get_flattened_vertex_list()
        .len();

        assert_eq!(count, 549);

        let model = ParticleCloud::new(
            "test-object-multi.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap();

        let vertex_groups = model.get_object_vertex_list();

        assert_eq!(vertex_groups.len(), 2);

        let count_1 = vertex_groups[0].len();
        let count_2 = vertex_groups[1].len();
        assert_eq!(count_1, 507);
        assert_eq!(count_2, 42);
    }

    #[test]
    fn shapes() {
        let object = &ParticleCloud::new(
            "test-object.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap()
        .obj[0];

        let geo = &object.geometry;
        assert_eq!(geo[0].shapes.len(), 967);

        let first_shape = geo[0].shapes[0].primitive;
        match first_shape {
            Primitive::Triangle(a, b, c) => {
                assert_eq!(a.0, 44);
                assert_eq!(b.0, 46);
                assert_eq!(c.0, 2);
            }
            _ => {
                panic!("IMPOSSIBLE STATE")
            }
        };
    }

    #[test]
    fn edges() {
        let object = ParticleCloud::new(
            "test-object.obj",
            ParticleGroupType::Single(Particle::AngryVillager),
        )
        .unwrap()
        .get_edges();

        assert_eq!(object.len(), 2901)
    }

    #[test]
    fn color() {
        let color = Color::new(128, 74, 112, 0.7);
        let no_trans = parse_mc_color(&color, false);
        let with_trans = parse_mc_color(&color, true);

        assert_eq!(no_trans, 8407664);
        assert_eq!(with_trans, 2994752112);
    }
}
