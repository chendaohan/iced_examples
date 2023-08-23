use iced::{
    font, theme,
    widget::{self, image},
    Font, Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    ExampleApp::run(Settings::default())?;

    Ok(())
}

const LENGTH: usize = 10;
const NUMBERS: [&str; LENGTH] =
    ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
const IMAGES: [&str; LENGTH] = [
    "0.jpg", "1.jpg", "2.png", "3.png", "4.jpeg", "5.jpeg", "6.jpeg", "7.jpeg", "8.jpeg", "9.jpeg",
];

struct ExampleApp {
    current_number: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Previous,
    Next,
}

impl Sandbox for ExampleApp {
    type Message = Message;

    fn new() -> Self {
        Self { current_number: 0 }
    }

    fn title(&self) -> String {
        String::from("Example App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Previous => {
                if self.current_number == 0 {
                    self.current_number = LENGTH - 1;
                } else {
                    self.current_number -= 1;
                }
            }
            Message::Next => {
                if self.current_number == LENGTH - 1 {
                    self.current_number = 0;
                } else {
                    self.current_number += 1;
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        widget::container(widget::column!(
            widget::text(format!("第{}张图片", NUMBERS[self.current_number]))
                .size(25.0)
                .font(Font {
                    family: font::Family::Name("Noto Sans CJK HK"),
                    ..Default::default()
                })
                .style(theme::Text::Color(iced::color!(255, 0, 0))),
            widget::image(image::Handle::from_path(format!(
                "assets/images/{}",
                IMAGES[self.current_number]
            )))
            .width(Length::Fixed(853.33))
            .height(Length::Fixed(480.0)),
            widget::row!(
                widget::button("Previous").on_press(Message::Previous),
                widget::button(
                    widget::text("Next")
                        .size(25)
                        .style(theme::Text::Color(iced::color!(255, 0, 0)))
                )
                .on_press(Message::Next)
            )
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
