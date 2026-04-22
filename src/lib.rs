pub mod visuals;
pub use crate::visuals::particle;

pub mod loader;
pub use crate::loader::cloud;

pub mod ui;

#[cfg(test)]
mod tests {

    use color_art::Color;
    use wavefront_obj::obj::Primitive;

    use super::{cloud::*, particle::*};
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
