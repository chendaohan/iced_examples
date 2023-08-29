use std::any::TypeId;

use iced::{
    executor,
    futures::SinkExt,
    subscription, theme,
    widget::{self, image},
    Application, Command, Length, Settings, Subscription,
};
use tonic::Request;

use crate::protos::image::{image_client::ImageClient, Empty};

pub mod protos;

fn main() -> iced::Result {
    ExampleApp::run(Settings::default())?;

    Ok(())
}

struct ExampleApp {
    images: Vec<image::Handle>,
    subscription: bool,
}

#[derive(Debug, Clone)]
enum Message {
    PushImage(image::Handle),
    StartSubscription,
    EndSubscription,
}

impl Application for ExampleApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                images: vec![],
                subscription: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Example App")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::PushImage(handle) => self.images.push(handle),
            Message::StartSubscription => self.subscription = true,
            Message::EndSubscription => self.subscription = false,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        widget::container(
            widget::column!(
                widget::scrollable(widget::column(
                    self.images
                        .clone()
                        .into_iter()
                        .map(|handle| widget::image(handle).into())
                        .collect(),
                ))
                .width(Length::Fill)
                .height(Length::FillPortion(8)),
                widget::container(widget::button("Get Image").on_press(Message::StartSubscription))
                    .width(Length::Fill)
                    .height(Length::FillPortion(2))
                    .center_x()
                    .center_y()
            )
            .width(Length::Fixed(600.0))
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if self.subscription {
            struct SubscriptionId;
            subscription::channel(TypeId::of::<SubscriptionId>(), 5, |mut sender| async move {
                let mut client = ImageClient::connect("http://[::1]:8080").await.unwrap();
                let mut image_stream = client
                    .image_stream(Request::new(Empty {}))
                    .await
                    .unwrap()
                    .into_inner();
                loop {
                    match image_stream.message().await {
                        Ok(Some(image_bytes)) => {
                            println!("Push Image");
                            sender
                                .send(Message::PushImage(image::Handle::from_memory(
                                    image_bytes.image,
                                )))
                                .await
                                .unwrap();
                        }
                        Ok(None) => {
                            println!("End Subscription");
                            sender.send(Message::EndSubscription).await.unwrap();
                        }
                        Err(err) => {
                            println!("Error = {err}");
                            sender.send(Message::EndSubscription).await.unwrap();
                        }
                    }
                }
            })
        } else {
            Subscription::none()
        }
    }
}
