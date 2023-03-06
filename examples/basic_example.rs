use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use iced_views::Views;

enum Screens {
    First,
    Second,
    Third,
}

struct ApplicationExample {
    views: Views<Screens>,
}

#[derive(Debug, Clone)]
enum Message {
    ShowSecondView,
    ShowThirdView,
    CloseCurrentView,
}

impl Sandbox for ApplicationExample {
    type Message = Message;

    fn new() -> Self {
        Self {
            views: Views::new(Screens::First),
        }
    }

    fn title(&self) -> String {
        "Views example".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ShowSecondView => self.views.push(Screens::Second),
            Message::ShowThirdView => self.views.push(Screens::Third),
            Message::CloseCurrentView => self.views.pop(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = match self.views.current().expect("get current view") {
            Screens::First => {
                column![
                    view_test("First view"),
                    button("Show second view").on_press(Message::ShowSecondView),
                    button("Show third view").on_press(Message::ShowThirdView),
                ]
            }
            Screens::Second => {
                column![
                    view_test("Second view"),
                    button("Show third view").on_press(Message::ShowThirdView),
                    button("Close this view").on_press(Message::CloseCurrentView),
                ]
            }
            Screens::Third => {
                column![
                    view_test("Third view"),
                    button("Close this view").on_press(Message::CloseCurrentView),
                ]
            }
        };

        container(content.spacing(4).align_items(Alignment::Center))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn view_test(label: &str) -> Element<Message> {
    container(text(label)).center_x().center_y().into()
}

fn main() -> iced::Result {
    ApplicationExample::run(Settings::default())
}
