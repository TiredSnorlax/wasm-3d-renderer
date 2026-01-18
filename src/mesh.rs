use iced::Point;
use std::f32::consts::PI;

// For the formula to work, the center of the screen is at (0, 0)
// However, the canvas(0, 0) is at the top left corner
// The range for x, y are -1.0 to 1.0
#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Start and End are indexes of points
#[derive(Clone, Copy)]
pub struct Edge {
    pub start: usize,
    pub end: usize,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_point(&self, width: f32, height: f32, additional_y: f32) -> Point {
        // Apply to formula
        let x = (self.x) / (self.z + additional_y);
        let y = (self.y) / (self.z + additional_y);

        // Convert to canvas coordinates
        let x = (x + 1.0) / 2.0 * width;
        let y = (1.0 - (y + 1.0) / 2.0) * height;

        Point::new(x, y)
    }

    pub fn rotate_y_axis(&mut self, by: f32) {
        let cos = by.cos();
        let sin = by.sin();

        let new_x = self.x * cos - self.z * sin;
        let new_z = self.x * sin + self.z * cos;

        self.x = new_x;
        self.z = new_z;
    }

    pub fn rotate_x_axis(&mut self, by: f32) {
        let cos = by.cos();
        let sin = by.sin();

        let new_y = self.y * cos - self.z * sin;
        let new_z = self.y * sin + self.z * cos;

        self.y = new_y;
        self.z = new_z;
    }

    pub fn rotate_z_axis(&mut self, by: f32) {
        let cos = by.cos();
        let sin = by.sin();

        let new_x = self.x * cos - self.y * sin;
        let new_y = self.x * sin + self.y * cos;

        self.x = new_x;
        self.y = new_y;
    }
}

pub struct Mesh {
    pub points: Vec<Point3D>,
    pub edges: Vec<Edge>,
}

impl Mesh {
    pub fn cube() -> Self {
        Self {
            points: vec![
                Point3D::new(-0.25, 0.25, 0.25),
                Point3D::new(0.25, 0.25, 0.25),
                Point3D::new(-0.25, -0.25, 0.25),
                Point3D::new(0.25, -0.25, 0.25),
                Point3D::new(-0.25, 0.25, -0.25),
                Point3D::new(0.25, 0.25, -0.25),
                Point3D::new(-0.25, -0.25, -0.25),
                Point3D::new(0.25, -0.25, -0.25),
            ],
            edges: vec![
                Edge { start: 0, end: 1 },
                Edge { start: 0, end: 2 },
                Edge { start: 0, end: 4 },
                Edge { start: 1, end: 3 },
                Edge { start: 1, end: 5 },
                Edge { start: 2, end: 3 },
                Edge { start: 2, end: 6 },
                Edge { start: 3, end: 7 },
                Edge { start: 4, end: 5 },
                Edge { start: 4, end: 6 },
                Edge { start: 5, end: 7 },
                Edge { start: 6, end: 7 },
            ],
        }
    }

    pub fn pyramid() -> Self {
        Self {
            points: vec![
                Point3D::new(0.0, 0.25, 0.0),      // Top point (0)
                Point3D::new(-0.25, -0.25, 0.25),  // Base front-left (1)
                Point3D::new(0.25, -0.25, 0.25),   // Base front-right (2)
                Point3D::new(0.25, -0.25, -0.25),  // Base back-right (3)
                Point3D::new(-0.25, -0.25, -0.25), // Base back-left (4)
            ],
            edges: vec![
                // Connect top to base
                Edge { start: 0, end: 1 },
                Edge { start: 0, end: 2 },
                Edge { start: 0, end: 3 },
                Edge { start: 0, end: 4 },
                // Connect base points
                Edge { start: 1, end: 2 },
                Edge { start: 2, end: 3 },
                Edge { start: 3, end: 4 },
                Edge { start: 4, end: 1 },
            ],
        }
    }

    pub fn tetrahedron() -> Self {
        Self {
            points: vec![
                Point3D::new(0.25, 0.25, 0.25),
                Point3D::new(-0.25, -0.25, 0.25),
                Point3D::new(-0.25, 0.25, -0.25),
                Point3D::new(0.25, -0.25, -0.25),
            ],
            edges: vec![
                Edge { start: 0, end: 1 },
                Edge { start: 0, end: 2 },
                Edge { start: 0, end: 3 },
                Edge { start: 1, end: 2 },
                Edge { start: 1, end: 3 },
                Edge { start: 2, end: 3 },
            ],
        }
    }

    pub fn octahedron() -> Self {
        Self {
            points: vec![
                Point3D::new(0.0, 0.25, 0.0),  // Top
                Point3D::new(0.0, -0.25, 0.0), // Bottom
                Point3D::new(0.25, 0.0, 0.0),  // Right
                Point3D::new(-0.25, 0.0, 0.0), // Left
                Point3D::new(0.0, 0.0, 0.25),  // Front
                Point3D::new(0.0, 0.0, -0.25), // Back
            ],
            edges: vec![
                // Top pyramid
                Edge { start: 0, end: 2 },
                Edge { start: 0, end: 3 },
                Edge { start: 0, end: 4 },
                Edge { start: 0, end: 5 },
                // Bottom pyramid
                Edge { start: 1, end: 2 },
                Edge { start: 1, end: 3 },
                Edge { start: 1, end: 4 },
                Edge { start: 1, end: 5 },
                // Middle ring
                Edge { start: 2, end: 4 },
                Edge { start: 4, end: 3 },
                Edge { start: 3, end: 5 },
                Edge { start: 5, end: 2 },
            ],
        }
    }

    pub fn sphere() -> Self {
        let mut points = vec![];
        let mut edges = vec![];
        let radius = 0.3;
        let n1 = 10; // parallels
        let n2 = 10; // meridians

        for i in 0..=n1 {
            let phi = PI * i as f32 / n1 as f32;
            for j in 0..n2 {
                let theta = 2.0 * PI * j as f32 / n2 as f32;
                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos();
                let z = radius * phi.sin() * theta.sin();
                points.push(Point3D::new(x, y, z));
            }
        }

        for i in 0..n1 {
            for j in 0..n2 {
                let current = i * n2 + j;
                let next_j = i * n2 + (j + 1) % n2;
                let next_i = (i + 1) * n2 + j;
                edges.push(Edge {
                    start: current,
                    end: next_j,
                });
                if i < n1 - 1 {
                    edges.push(Edge {
                        start: current,
                        end: next_i,
                    });
                }
            }
        }

        Self { points, edges }
    }

    pub fn torus() -> Self {
        let mut points = vec![];
        let mut edges = vec![];

        let r1 = 0.3; // Major radius
        let r2 = 0.1; // Minor radius
        let n1 = 20; // Number of segments for the major circle
        let n2 = 10; // Number of segments for the minor circle

        for i in 0..n1 {
            let phi = 2.0 * PI * i as f32 / n1 as f32;
            for j in 0..n2 {
                let theta = 2.0 * PI * j as f32 / n2 as f32;
                let x = (r1 + r2 * theta.cos()) * phi.cos();
                let y = r2 * theta.sin();
                let z = (r1 + r2 * theta.cos()) * phi.sin();
                points.push(Point3D::new(x, y, z));
            }
        }

        for i in 0..n1 {
            for j in 0..n2 {
                let current = i * n2 + j;
                let next_j = i * n2 + (j + 1) % n2;
                let next_i = (i + 1) % n1 * n2 + j;
                edges.push(Edge {
                    start: current,
                    end: next_j,
                });
                edges.push(Edge {
                    start: current,
                    end: next_i,
                });
            }
        }

        Self { points, edges }
    }
}

#[derive(Clone, Copy)]
pub enum MeshPreset {
    Cube,
    Pyramid,
    Tetrahedron,
    Octahedron,
    Sphere,
    Torus,
}
