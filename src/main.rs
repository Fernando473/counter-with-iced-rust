use iced::{Element, Sandbox, Settings};
use iced::widget::{button, column, text, Column};
use iced_core::alignment;

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    // The counter value
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;
    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // fn view(&self) -> Element<Message> {
    //     // We use a column: a simple vertical layout
    //     column![
    //         // The increment button. We tell it to produce an
    //         // `IncrementPressed` message when pressed
    //         button("+").on_press(Message::IncrementPressed),
    //
    //         // We show the value of the counter here
    //         text(self.value).size(50),
    //
    //         // The decrement button. We tell it to produce a
    //         // `DecrementPressed` message when pressed
    //         button("-").on_press(Message::DecrementPressed),
    //     ]
    //         .
    // }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50).vertical_alignment(alignment::Vertical::Center).horizontal_alignment(alignment::Horizontal::Center),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
            .padding(20)
            .into()
    }
}



