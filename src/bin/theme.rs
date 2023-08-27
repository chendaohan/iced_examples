use iced::{font::Family, widget, Font, Length, Sandbox, Settings, window, theme::Palette};

fn main() -> iced::Result {
    ExampleApp::run(Settings {
        default_font: Font {
            family: Family::Name("Noto Sans CJK HK"),
            ..Default::default()
        },
        window: window::Settings {
            size: (400, 400),
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}

struct ExampleApp;

impl Sandbox for ExampleApp {
    type Message = ();

    fn new() -> Self {
        ExampleApp
    }

    fn title(&self) -> String {
        String::from("Example App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        widget::container(
            widget::column!(
                widget::text("This is a text widget."),
                widget::checkbox("Checkbox", true, |_| ()),
                widget::text_input("This is text input's placeholder.", "Some Text")
                    .width(Length::Fixed(300.0))
                    .on_input(|_| ()),
                widget::progress_bar(0.0..=100.0, 50.0).width(Length::Fixed(300.0)),
            )
            .spacing(30),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::custom(Palette {
            background: iced::color!(0x1E90FF),
            text: iced::color!(0xFFFFFF),
            primary: iced::color!(0x32CD32),
            success: iced::color!(0xC71585),
            danger: iced::color!(0xFF0000),
        })
    }
}
