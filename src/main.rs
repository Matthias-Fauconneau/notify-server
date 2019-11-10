#!/usr/bin/env -S cargo run

struct Notifications {
    notification : notify_rust::Notification
}

#[derive(Debug)]
pub enum Message {}

impl iced::Application for Notifications {
    type Message = Message;
    type Renderer = iced::Renderer;

    fn title(&self) -> std::string::String { self.notification.summary.clone() }

    fn update(&mut self, message: Message) {
        match message {
        }
    }

    fn view(&mut self) -> iced::Element<Message> {
        let content = iced::Text::new(&self.notification.body);
        iced::Column::new()
            .width(iced::Length::Shrink)
            .align_self(iced::Align::Center)
            .align_items(iced::Align::Center)
            .height(iced::Length::Fill)
            .justify_content(iced::Justify::Center)
            .push(content)
            .into()
    }
}

fn main() {
    env_logger::init();
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(500)); // FIXME: wait for dbus signal
        notify_rust::Notification::new().summary("Notification Test").body("This is a test notification.").show().unwrap();
    });
    notify_rust::server::NotificationServer::start(&notify_rust::server::NotificationServer::create(), |notification| {
        println!("{:#?}", notification);

        let mut instance = iced::Instance::new(Notifications{notification:notification.clone()});
        let monitor = instance.platform.event_loop.primary_monitor(); // FIXME: get where window would map
        let size = monitor.size();
        instance.platform.window_builder = instance.platform.window_builder
                                                            .with_inner_size(iced::dpi::LogicalSize::from_physical(iced::dpi::PhysicalSize{width: size.width/3., height: size.height/8.}, monitor.hidpi_factor()));
        instance.run().unwrap()
    });
}
