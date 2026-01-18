use iced::{
    Font, Length, Size, Task,
    widget::{button, column, container, row, text},
};

use crate::{
    mesh::MeshPreset,
    renderer::{MyRenderer, RendererMessage},
};

mod mesh;
mod renderer;

const FRAME_RATE: f32 = 120.0;

pub const FONT: Font = Font::with_name("DepartureMono Nerd Font");

fn main() -> iced::Result {
    iced::application(MyApp::new, MyApp::update, MyApp::view)
        .font(include_bytes!("../fonts/DepartureMonoNerdFont-Regular.otf"))
        .default_font(FONT)
        .window_size(Size::new(800.0, 800.0))
        .subscription(MyApp::subscription)
        .run()
}

#[derive(Clone)]
enum Message {
    App(AppMessage),
    Renderer(RendererMessage),
}

#[derive(Clone)]
enum AppMessage {
    Tick,
    StartAnimations(MeshPreset),
    StopAnimations,
}

struct MyApp {
    renderer: MyRenderer,
    is_animating: bool,
}

impl MyApp {
    fn new() -> Self {
        Self {
            renderer: MyRenderer::new(),
            is_animating: false,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::App(app_message) => match app_message {
                AppMessage::Tick => {
                    self.renderer.tick();
                }
                AppMessage::StartAnimations(preset) => {
                    self.is_animating = true;
                    self.renderer.set_mesh(preset);
                }
                AppMessage::StopAnimations => {
                    self.is_animating = false;
                    self.renderer.clear();
                }
            },
            Message::Renderer(renderer_message) => {
                return self
                    .renderer
                    .p_update(renderer_message)
                    .map(Message::Renderer);
            }
        }
        Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        if self.is_animating {
            iced::time::every(std::time::Duration::from_millis(1000 / FRAME_RATE as u64))
                .map(|_| Message::App(AppMessage::Tick))
        } else {
            iced::Subscription::none()
        }
    }

    fn view(&'_ self) -> iced::Element<'_, Message> {
        if self.is_animating {
            self.renderer_view()
        } else {
            self.select_view()
        }
    }

    fn renderer_view(&self) -> iced::Element<'_, Message> {
        let content = column![
            self.renderer.view().map(Message::Renderer),
            button("Stop").on_press(Message::App(AppMessage::StopAnimations))
        ]
        .width(Length::Fill)
        .height(Length::Fill);
        content.into()
    }

    fn select_view(&'_ self) -> iced::Element<'_, Message> {
        let buttons = container(
            column![
                row![
                    button("Cube")
                        .on_press(Message::App(AppMessage::StartAnimations(MeshPreset::Cube))),
                    button("Pyramid").on_press(Message::App(AppMessage::StartAnimations(
                        MeshPreset::Pyramid
                    ))),
                    button("Tetrahedron").on_press(Message::App(AppMessage::StartAnimations(
                        MeshPreset::Tetrahedron
                    ))),
                ]
                .spacing(20),
                row![
                    button("Octahedron").on_press(Message::App(AppMessage::StartAnimations(
                        MeshPreset::Octahedron
                    ))),
                    button("Sphere").on_press(Message::App(AppMessage::StartAnimations(
                        MeshPreset::Sphere
                    ))),
                    button("Torus")
                        .on_press(Message::App(AppMessage::StartAnimations(MeshPreset::Torus))),
                ]
                .spacing(20),
            ]
            .padding(20.0)
            .spacing(20),
        )
        .center_x(Length::Fill);

        let prompt = text!("Select a mesh to display")
            .size(30.0)
            .width(Length::Fill)
            .center();

        let content = container(column![prompt, buttons])
            .width(Length::Fill)
            .center_y(Length::Fill);

        content.into()
    }
}
