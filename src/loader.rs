pub mod cloud {
    use std::{fs, path::Path};

    use crate::visuals::particle::{Particle, ParticleGroupType};

    use wavefront_obj::{
        ParseError,
        obj::{Object, Primitive, Vertex, parse},
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vert {
        pub vertex: Vertex,
        pub particle: Particle,
    }

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
        pub cached_points: Option<Vec<Vert>>,
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
                    cached_points: None,
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

        pub fn output(&self, rotation_space: RotationSpace) -> Result<String, &str> {
            let mut output: String = String::from(
                "# File Generated with Particle3D - https://github.com/SmoothTurtle872/particle3d \n",
            );
            let rotation_space = match rotation_space {
                RotationSpace::Global => "~",
                RotationSpace::Local => "^",
            };

            match &self.cached_points {
                Some(verts) => {
                    for vert in verts {
                        let line = format!(
                            "particle {} {rotation_space}{} {rotation_space}{} {rotation_space}{} 0 0 0 0 1 force @a\n",
                            vert.particle, vert.vertex.x, vert.vertex.y, vert.vertex.z
                        );
                        output += &line;
                    }
                    return Ok(output);
                }
                None => return Err("No Cached Points!"),
            }
        }

        pub fn cache_points(&mut self, subdivide_edges: bool, points_per_edge: Option<i32>) {
            let mut edge_divisor = 1;
            if subdivide_edges {
                match points_per_edge {
                    Some(x) => edge_divisor = x,
                    None => {}
                }
            }
            let mut verts: Vec<Vert> = vec![];
            match &self.particle {
                ParticleGroupType::Single(particle) => {
                    let vertecies = self.get_flattened_vertex_list();
                    for vertex in vertecies {
                        verts.push(Vert {
                            vertex: vertex.clone(),
                            particle: particle.clone(),
                        });
                    }
                    if subdivide_edges {
                        for edge in self.get_edges() {
                            let obj = &self.obj[edge.object];
                            let end = &obj.vertices[edge.end];
                            let start = &obj.vertices[edge.start];

                            let x_dir = end.x - start.x;
                            let y_dir = end.y - start.y;
                            let z_dir = end.z - start.z;
                            let dist =
                                (x_dir.powf(2_f64) + y_dir.powf(2_f64) + z_dir.powf(2_f64)).sqrt();

                            let (x_dir, y_dir, z_dir) = (
                                x_dir / ((dist / edge_divisor as f64) / dist),
                                y_dir / ((dist / edge_divisor as f64) / dist),
                                z_dir / ((dist / edge_divisor as f64) / dist),
                            );

                            let mut current_pos: Vertex = start.clone();

                            for _ in 0..edge_divisor {
                                current_pos.x += x_dir;
                                current_pos.y += y_dir;
                                current_pos.z += z_dir;
                                verts.push(Vert {
                                    vertex: current_pos.clone(),
                                    particle: particle.clone(),
                                });
                            }
                        }
                    }
                }
                ParticleGroupType::Multi(particles) => {
                    for (particle, vertex_list) in
                        particles.iter().zip(self.get_object_vertex_list())
                    {
                        for vertex in vertex_list {
                            verts.push(Vert {
                                vertex: vertex.clone(),
                                particle: particle.clone(),
                            });
                        }
                    }
                    if subdivide_edges {
                        for edge in self.get_edges() {
                            let obj = &self.obj[edge.object];
                            let end = &obj.vertices[edge.end];
                            let start = &obj.vertices[edge.start];

                            let x_dir = end.x - start.x;
                            let y_dir = end.y - start.y;
                            let z_dir = end.z - start.z;
                            let dist =
                                (x_dir.powf(2_f64) + y_dir.powf(2_f64) + z_dir.powf(2_f64)).sqrt();

                            let (x_dir, y_dir, z_dir) = (
                                x_dir / ((dist / edge_divisor as f64) / dist),
                                y_dir / ((dist / edge_divisor as f64) / dist),
                                z_dir / ((dist / edge_divisor as f64) / dist),
                            );

                            let mut current_pos: Vertex = start.clone();

                            for _ in 0..edge_divisor {
                                current_pos.x += x_dir;
                                current_pos.y += y_dir;
                                current_pos.z += z_dir;
                                verts.push(Vert {
                                    vertex: current_pos.clone(),
                                    particle: particles[edge.object].clone(),
                                });
                            }
                        }
                    }
                }
            };
            self.cached_points = Some(verts);
        }
    }

    #[derive(Debug, Default, PartialEq, Clone)]
    pub enum RotationSpace {
        #[default]
        Global,
        Local,
    }

    impl std::fmt::Display for RotationSpace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Global => write!(f, "Global"),
                Self::Local => write!(f, "Local"),
            }
        }
    }
}
