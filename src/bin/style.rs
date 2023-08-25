use iced::{
    theme,
    widget::{self, button, container},
    Background, BorderRadius, Color, Length, Padding, Sandbox, Settings,
};

fn main() -> iced::Result {
    ExampleApp::run(Settings::default())?;

    Ok(())
}

struct ExampleApp;

impl Sandbox for ExampleApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Example App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        widget::container(
            widget::column({
                let mut column = vec![];
                for (container_style, button_style) in [
                    (ContainerStyle::BlackBorder, ButtonStyle::BlueBackground),
                    (ContainerStyle::BlueBorder, ButtonStyle::GreenBackground),
                    (ContainerStyle::RedBorder, ButtonStyle::RedBackground),
                ] {
                    column.push(
                        widget::container(
                            widget::button("Button")
                                .style(theme::Button::custom(button_style))
                                .on_press(()),
                        )
                        .padding(Padding::from(20))
                        .style(theme::Container::Custom(Box::new(container_style)))
                        .into(),
                    );
                }
                column
            })
            .spacing(30),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn scale_factor(&self) -> f64 {
        2.0
    }
}

enum ContainerStyle {
    BlackBorder,
    BlueBorder,
    RedBorder,
}

impl container::StyleSheet for ContainerStyle {
    type Style = theme::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let palette = style.palette();
        container::Appearance {
            text_color: Some(palette.text),
            background: Some(Background::Color(palette.background)),
            border_width: 5.0,
            border_color: match self {
                Self::BlackBorder => Color::BLACK,
                Self::BlueBorder => iced::color!(0, 0, 255),
                Self::RedBorder => iced::color!(255, 0, 0),
            },
            border_radius: BorderRadius::from(5.0),
        }
    }
}

enum ButtonStyle {
    RedBackground,
    GreenBackground,
    BlueBackground,
}

impl button::StyleSheet for ButtonStyle {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::RedBackground => iced::color!(0xff0033),
                Self::GreenBackground => iced::color!(0x99cc00),
                Self::BlueBackground => iced::color!(0x0099cc),
            })),
            text_color: palette.text,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::RedBackground => iced::color!(0xcc0033),
                Self::GreenBackground => iced::color!(0x336633),
                Self::BlueBackground => iced::color!(0x0066cc),
            })),
            text_color: palette.text,
            ..Default::default()
        }
    }
}
