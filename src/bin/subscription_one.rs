use iced::{executor, subscription, theme, widget, Application, Command, Length, Settings};

fn main() -> iced::Result {
    ExampleApp::run(Settings::default())?;

    Ok(())
}

struct ExampleApp {
    events: Vec<iced::Event>,
}

#[derive(Debug, Clone)]
struct PushEvent(iced::Event);

impl Application for ExampleApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = PushEvent;
    type Theme = theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { events: vec![] }, Command::none())
    }

    fn title(&self) -> String {
        "Example App".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.events.push(message.0);
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        widget::container(
            widget::scrollable(
                widget::column(
                    self.events
                        .clone()
                        .into_iter()
                        .map(|event| widget::text(format!("{event:?}")).into())
                        .collect(),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events_with(|event, _| Some(PushEvent(event)))
    }
}
