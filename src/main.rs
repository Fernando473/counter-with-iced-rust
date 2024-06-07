mod counter;
mod message;

use iced::{Element, Sandbox, Settings, Theme};
use iced::widget::{column, button, text, text_input, container};
use iced::alignment;
use iced::Theme::{Dracula};
use crate::counter::Counter;
use crate::message::Message;

fn main() -> iced::Result {
    Counter::run(Settings::default())
}


impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0, value_to_mutter: "1".parse().unwrap() }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += self.value_to_mutter.parse::<i32>().unwrap();
            }
            Message::DecrementPressed => {
                self.value -= self.value_to_mutter.parse::<i32>().unwrap();
            }
            Message::InputChanged(input) => {
                self.value_to_mutter = input
            }
        }
    }


    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50).vertical_alignment(alignment::Vertical::Center).horizontal_alignment(alignment::Horizontal::Center),
            button("Decrement").on_press(Message::DecrementPressed),
            container("").padding(20),
            container("").padding(20),
            text_input("Quantity", &self.value_to_mutter).on_input(Message::InputChanged),

        ]
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Dracula
    }
}



