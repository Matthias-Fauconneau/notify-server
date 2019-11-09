#!/usr/bin/env -S cargo run
use iced::winit as winit;
struct Notifications {
    notification : notify_rust::Notification
}
#[derive(Debug)]
pub enum Message {}
impl iced::Application for Notifications {
    fn title(&self) -> std::string::String { self.notification.summary.clone() }
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

        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap(); // FIXME: do not show. only to get on which current_monitor WM would map
        let size = window.current_monitor().size();
        window.set_inner_size(winit::dpi::LogicalSize::from_physical(winit::dpi::PhysicalSize{width: size.width/2., height: size.height/2.}, window.hidpi_factor()));
        iced::platform::Application::run_winit(iced::Instance(Notifications{notification:notification.clone()}), event_loop, window);
    });
}
