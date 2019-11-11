#!/usr/bin/env -S cargo run

struct Notification<'a>(&'a notify_rust::Notification);

#[derive(Debug, Clone)]
pub enum Message {}

impl<'a> iced::Application for Notification<'a> {
    type Message = Message;
    type Renderer = iced::Renderer;

    fn title(&self) -> std::string::String { self.0.summary.clone() }

    fn update(&mut self, message: Message) {
        match message {
        }
    }

    fn style(&self) -> iced::Style { iced::Style{window_background: iced::Color{r:0.,g:0.,b:0.,a:1./2.} } }
    fn view(&mut self) -> iced::Element<Message> { iced::Column::new().height(iced::Length::Fill).align_self(iced::Align::Center).justify_content(iced::Justify::Center).push(
        iced::Text::new(&self.0.body).color(iced::Color::WHITE).horizontal_alignment(iced::text::HorizontalAlignment::Center).vertical_alignment(iced::text::VerticalAlignment::Center)).into()
    }
}

fn main() {
    env_logger::init();
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(500)); // FIXME: wait for dbus signal
        notify_rust::Notification::new().summary("Notification Test").body("This is a test notification.").show().unwrap();
    });
    notify_rust::server::NotificationServer::start(&notify_rust::server::NotificationServer::create(), move |notification : &notify_rust::Notification| {
        println!("{:#?}", notification);

        let mut instance = iced::Instance::new(Notification(&notification));
        let monitor = instance.platform.event_loop.primary_monitor(); // FIXME: get where window would map
        let size = monitor.size();
        instance.platform.window_builder = instance.platform.window_builder
                                                            .with_inner_size(iced::dpi::LogicalSize::from_physical(iced::dpi::PhysicalSize{width: size.width/3., height: size.height/8.}, monitor.hidpi_factor()));
        instance.run().unwrap()
    });
}
