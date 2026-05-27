pub mod particle {

    use color_art::Color;

    use crate::particle::ParticleSetting::SingleColorSized;

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

    #[derive(Debug, Clone, PartialEq, Default)]
    pub enum Particle {
        #[default]
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
        Dust,
        Effect,
        EggCrack,
        ElderGuardian,
        ElectricSpark,
        Enchant,
        EndRod,
        EntityEffect,
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
        Flash,
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
        Other,
    }

    impl std::fmt::Display for Particle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                // ---------------------custom--------------------- //
                Self::Other => write!(f, "other"),
                Self::Dust => write!(f, "dust"),
                Self::Effect => {
                    write!(f, "effect")
                }
                Self::EntityEffect => {
                    write!(f, "entity_effect")
                }
                Self::Flash => write!(f, "flash"),
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

    impl Particle {
        pub fn output(&self) -> String {
            match self {
                Self::Other => "".to_string(),
                // --------------------- Normal ----------------------- //
                _ => {
                    format!("{}", self)
                }
            }
        }

        pub fn get_option_list() -> [Particle; 97] {
            [
                Self::AngryVillager,
                Self::Ash,
                Self::Bubble,
                Self::BubbleColumnUp,
                Self::BubblePop,
                Self::CampfireCosySmoke,
                Self::CampfireSginalSmoke,
                Self::CherryLeaves,
                Self::Cloud,
                Self::Composter,
                Self::CopperFireFlame,
                Self::CrimsonSpore,
                Self::Crit,
                Self::CurrentDown,
                Self::DamageIndicator,
                Self::Dolphin,
                Self::DragonBreath,
                Self::DrippingDripstoneLava,
                Self::DrippingDripstoneWater,
                Self::DrippingHoney,
                Self::DrippingLava,
                Self::DrippingObsidianTear,
                Self::DrippingWater,
                Self::EggCrack,
                Self::ElderGuardian,
                Self::ElectricSpark,
                Self::Enchant,
                Self::EndRod,
                Self::Explosion,
                Self::ExplosionEmitter,
                Self::FallingDripstoneLava,
                Self::FallingDripstoneWater,
                Self::FallingHoney,
                Self::FallingLava,
                Self::FallingNectar,
                Self::FallingObsidianTear,
                Self::FallingSporeBlossom,
                Self::FallingWater,
                Self::Firefly,
                Self::Fishing,
                Self::Flame,
                Self::Glow,
                Self::GlowSquidInk,
                Self::Gust,
                Self::GustEmitter,
                Self::HappyVillager,
                Self::Heart,
                Self::Infested,
                Self::ItemCobweb,
                Self::ItemSlime,
                Self::ItemSnowball,
                Self::LandingHoney,
                Self::LandingLava,
                Self::LandingObsidianTear,
                Self::LargeSmoke,
                Self::Lava,
                Self::Mycelium,
                Self::Nautilus,
                Self::OminousSpawning,
                Self::PaleOakLeaves,
                Self::PauseMobGrowth,
                Self::Poof,
                Self::Portal,
                Self::RaidOmen,
                Self::Rain,
                Self::ResetMobGrowth,
                Self::ReversePortal,
                Self::Scrape,
                Self::SculkChargePop,
                Self::SculkSoul,
                Self::Shriek,
                Self::SmallFlame,
                Self::SmallGust,
                Self::Smoke,
                Self::Sneeze,
                Self::Snowflake,
                Self::SonicBoom,
                Self::Soul,
                Self::SoulFireFlame,
                Self::Spit,
                Self::Splash,
                Self::SporeBlossomAir,
                Self::SquidInk,
                Self::SweepAttack,
                Self::TotemOfUndying,
                Self::TrialOmen,
                Self::TrialSpawnerDetection,
                Self::TrialSpawnerDetectionOminous,
                Self::Underwater,
                Self::VaultConnection,
                Self::WarpedSpore,
                Self::WaxOff,
                Self::WaxOn,
                Self::WhiteAsh,
                Self::WhiteSmoke,
                Self::Witch,
                Self::Other,
            ]
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ParticleGroupType {
        Single(Particle, ParticleSetting),
        Multi(Vec<(Particle, ParticleSetting)>),
    }

    impl Default for ParticleGroupType {
        fn default() -> Self {
            ParticleGroupType::Single(Particle::AngryVillager, ParticleSetting::None)
        }
    }

    impl std::fmt::Display for ParticleGroupType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Multi(_) => {
                    write!(f, "Multiple")
                }
                Self::Single(_, _) => {
                    write!(f, "Single")
                }
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    pub enum ParticleSetting {
        #[default]
        None,
        SingleColor(Color),
        SingleColorTransp(Color),
        SingleColorSized(Color, f32),
        Other(String),
    }

    impl ParticleSetting {
        pub fn from_particle(particle: &Particle) -> Self {
            match particle {
                Particle::Flash | Particle::EntityEffect => {
                    Self::SingleColorTransp(Color::default())
                }
                Particle::Dust => SingleColorSized(Color::default(), 1.0),
                Particle::Other => Self::Other("".to_string()),
                _ => Self::None,
            }
        }

        pub fn output(&self) -> String {
            match self {
                Self::None => "".to_string(),
                Self::Other(value) => value.clone(),
                Self::SingleColor(color) => {
                    format!("{{color:{}}}", parse_mc_color(color, false))
                }
                Self::SingleColorTransp(color) => {
                    format!("{{color:{}}}", parse_mc_color(color, true))
                }
                Self::SingleColorSized(color, size) => {
                    format!("{{color:{}, scale:{}}}", parse_mc_color(color, false), size)
                }
            }
        }
    }
}
