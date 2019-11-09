#!/usr/bin/env -S cargo run
use iced::{Application};
struct Notifications {
    notification : notify_rust::Notification
}
#[derive(Debug)]
pub enum Message {}
impl iced::Application for Notifications {
    type Message = Message;
    fn update(&mut self, message: Message) {
        match message {
        }
    }
    fn view(&mut self) -> iced::Element<Message> {
        let content = iced::Text::new(&self.notification.body);
        iced::Column::new()
            .height(iced::Length::Fill)
            .justify_content(iced::Justify::Center)
            .push(content)
            .into()
    }
}

fn main() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(500)); // FIXME: wait for dbus signal
        notify_rust::Notification::new().summary("Notification Test").body("This is a test notification.").show().unwrap();
    });
    notify_rust::server::NotificationServer::start(&notify_rust::server::NotificationServer::create(), |notification| {
        println!("{:#?}", notification);
        Notifications{notification:notification.clone()}.run();
    });
}
