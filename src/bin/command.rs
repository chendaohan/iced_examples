use std::{fs, path::Path};

use iced::{
    executor, theme,
    widget::{
        self, image,
        scrollable::{self, AbsoluteOffset},
    },
    Application, Command, Length, Settings,
};

fn main() -> iced::Result {
    ExampleApp::run(Settings::default())?;

    Ok(())
}

async fn read_image(path: impl AsRef<Path>) -> image::Handle {
    image::Handle::from_memory(fs::read(path).unwrap())
}

struct ExampleApp {
    images: Vec<image::Handle>,
    id: scrollable::Id,
}

#[derive(Debug, Clone)]
enum Message {
    AddImage(image::Handle),
    ScrollTo(scrollable::AbsoluteOffset),
}

impl Application for ExampleApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let commands = Command::batch([
            Command::perform(read_image("assets/images/0.jpg"), Message::AddImage),
            Command::perform(read_image("assets/images/1.jpg"), Message::AddImage),
            Command::perform(read_image("assets/images/2.png"), Message::AddImage),
        ]);
        (
            Self {
                images: vec![],
                id: scrollable::Id::unique(),
            },
            commands,
        )
    }

    fn title(&self) -> String {
        String::from("Example App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AddImage(image) => self.images.push(image),
            Message::ScrollTo(offset) => return scrollable::scroll_to(self.id.clone(), offset),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        widget::container(widget::column!(
            widget::scrollable(widget::column(
                self.images
                    .clone()
                    .into_iter()
                    .map(|image| widget::image(image).into())
                    .collect()
            ))
            .id(self.id.clone())
            .height(Length::FillPortion(8)),
            widget::container(
                widget::button("Scroll To")
                    .on_press(Message::ScrollTo(AbsoluteOffset { x: 0.0, y: 1000.0 }))
            )
            .width(Length::Fill)
            .height(Length::FillPortion(2))
            .center_x()
            .center_y()
        ))
        .width(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .into()
    }
}
