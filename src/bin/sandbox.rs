use iced::{Settings, widget, Length, Sandbox};

fn main() -> iced::Result {
    Counter::run(Settings::default())?;

    Ok(())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        widget::container(widget::column!(
            widget::button("Increment").on_press(Message::IncrementPressed),
            widget::text(self.value).size(50),
            widget::button("Decrement").on_press(Message::DecrementPressed),
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
