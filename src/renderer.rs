use iced::widget::text::LineHeight;
use web_time::Instant;

use crate::FONT;
use crate::mesh::{Mesh, MeshPreset, Point3D};
use iced::widget::canvas::{self, Frame, Path, Program, Stroke, Style};
use iced::widget::{Canvas, checkbox, column, container, row, slider, text};
use iced::{Alignment, color};
use iced::{Color, Length, Point, Rectangle, Renderer, Task, Theme};

#[derive(Debug, Clone)]
pub enum RendererMessage {
    DXTChanged(f32),
    DYTChanged(f32),
    DZTChanged(f32),
    DistanceChanged(f32),
    RenderEdgesChanged(bool),
    RenderPointsChanged(bool),
}

impl Program<RendererMessage> for MyRenderer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let fps_counter = canvas::Text {
            content: format!("FPS: {}", self.fps),
            position: Point { x: 10.0, y: 10.0 },
            color: Color::WHITE,
            size: iced::Pixels(20.0),
            font: FONT,
            line_height: LineHeight::Absolute(iced::Pixels(20.0)),
            ..Default::default()
        };

        frame.fill_text(fps_counter);

        if let Some(mesh) = &self.mesh {
            if self.render_points {
                for point in &mesh.points {
                    self.draw_circle(&mut frame, point, 5.0);
                }
            }

            if self.render_edges {
                for edge in &mesh.edges {
                    let p1 = mesh.points[edge.start].to_point(
                        frame.width(),
                        frame.height(),
                        self.distance_to_screen,
                    );
                    let p2 = mesh.points[edge.end].to_point(
                        frame.width(),
                        frame.height(),
                        self.distance_to_screen,
                    );

                    self.draw_line(&mut frame, p1, p2);
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

pub struct MyRenderer {
    mesh: Option<Mesh>,
    dxt: f32,
    dyt: f32,
    dzt: f32,
    distance_to_screen: f32,
    render_points: bool,
    render_edges: bool,
    last_tick: Instant,
    frame_count: u32,
    fps: u32,
}

impl MyRenderer {
    pub fn new() -> Self {
        Self {
            mesh: None,
            dxt: 0.025,
            dyt: 0.025,
            dzt: 0.025,
            distance_to_screen: 1.0,
            render_edges: true,
            render_points: true,
            last_tick: Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }
}

impl MyRenderer {
    pub fn view(&'_ self) -> iced::Element<'_, RendererMessage> {
        let controls = container(
            column![
                text(format!("FPS: {}", self.fps)).color(color!(0xFFFFFF)),
                text(format!("Dx Theta: {:.2}", self.dxt)).color(color!(0xFFFFFF)),
                slider(-40.0..=40.0, self.dxt * 100.0, |v| {
                    RendererMessage::DXTChanged(v)
                }),
                text(format!("Dy Theta: {:.2}", self.dyt)),
                slider(-40.0..=40.0, self.dyt * 100.0, |v| {
                    RendererMessage::DYTChanged(v)
                }),
                text(format!("Dz Theta: {:.2}", self.dzt)),
                slider(-40.0..=40.0, self.dzt * 100.0, |v| {
                    RendererMessage::DZTChanged(v)
                }),
                text(format!("Distance: {:.2}", self.distance_to_screen)),
                slider(1.0..=500.0, self.distance_to_screen * 100.0, |v| {
                    RendererMessage::DistanceChanged(v)
                }),
                row![
                    text("Render Edges"),
                    checkbox(self.render_edges).on_toggle(RendererMessage::RenderEdgesChanged),
                    text("Render Points"),
                    checkbox(self.render_points).on_toggle(RendererMessage::RenderPointsChanged),
                ]
                .spacing(20.0)
                .align_y(Alignment::Center)
            ]
            .align_x(Alignment::Center)
            .spacing(10),
        )
        .padding(10.0);

        let canvas = Canvas::new(self).width(Length::Fill).height(Length::Fill);
        column![canvas, controls].into()
    }

    pub fn p_update(&mut self, message: RendererMessage) -> Task<RendererMessage> {
        match message {
            RendererMessage::DXTChanged(value) => {
                self.dxt = value / 100.0;
            }
            RendererMessage::DYTChanged(value) => {
                self.dyt = value / 100.0;
            }
            RendererMessage::DZTChanged(value) => {
                self.dzt = value / 100.0;
            }
            RendererMessage::DistanceChanged(value) => {
                self.distance_to_screen = value / 100.0;
            }
            RendererMessage::RenderEdgesChanged(v) => {
                self.render_edges = v;
            }
            RendererMessage::RenderPointsChanged(v) => {
                self.render_points = v;
            }
        }
        Task::none()
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now - self.last_tick;
        self.frame_count += 1;
        if elapsed.as_millis() >= 1000 {
            self.fps = (self.frame_count * 1000) / elapsed.as_millis() as u32;
            self.frame_count = 0;
            self.last_tick = now
        }

        if let Some(mesh) = &mut self.mesh {
            for point in &mut mesh.points {
                point.rotate_y_axis(self.dyt);
                point.rotate_x_axis(self.dxt);
                point.rotate_z_axis(self.dzt);
            }
        }
    }

    fn draw_circle(&self, frame: &mut Frame, center: &Point3D, radius: f32) {
        let circle = canvas::Path::circle(
            center.to_point(frame.width(), frame.height(), self.distance_to_screen),
            radius,
        );
        frame.fill(&circle, Color::WHITE);
    }

    fn draw_line(&self, frame: &mut Frame, p1: Point, p2: Point) {
        let line_path = Path::line(p1, p2);
        let stroke_style = Stroke {
            width: 2.0,
            style: Style::Solid(Color::WHITE),
            ..Stroke::default()
        };
        frame.stroke(&line_path, stroke_style);
    }

    pub fn clear(&mut self) {
        self.mesh = None;
    }

    pub fn set_mesh(&mut self, mesh_preset: MeshPreset) {
        let mesh = match mesh_preset {
            MeshPreset::Cube => Some(Mesh::cube()),
            MeshPreset::Pyramid => Some(Mesh::pyramid()),
            MeshPreset::Tetrahedron => Some(Mesh::tetrahedron()),
            MeshPreset::Octahedron => Some(Mesh::octahedron()),
            MeshPreset::Sphere => Some(Mesh::sphere()),
            MeshPreset::Torus => Some(Mesh::torus()),
        };
        self.mesh = mesh;
    }
}
