#!/usr/bin/env -S cargo run

struct Notifications {
    notification : notify_rust::Notification
}

#[derive(Debug)]
pub enum Message {}

impl iced::Application for Notifications {
    type Message = Message;

    fn title(&self) -> std::string::String {
        self.notification.summary.clone()
    }

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

        let event_loop = iced::platform::EventLoop::new();
        let window = iced::platform::Window::new(&event_loop).unwrap(); // FIXME: do not show. only to get on which current_monitor WM would map
        let size = window.current_monitor().size();
        window.set_inner_size(iced::platform::dpi::LogicalSize::from_physical(iced::platform::dpi::PhysicalSize{width: size.width/2., height: size.height/2.}, window.hidpi_factor()));
        use iced::Application;
        Notifications{notification:notification.clone()}.run(event_loop, window);
    });
}
