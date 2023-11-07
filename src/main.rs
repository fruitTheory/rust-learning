use iced::{button, Button, Column, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    increment_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter {
            value: 0,
            increment_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Message> {
        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()))
            .into()
    }
}
