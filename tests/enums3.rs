// I AM NOT DONE

enum Message {
    ChangeColor((u8, u8, u8)),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    pub fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn echo(&self, s: String) {
        println!("{}", s);
    }

   pub fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Move(point) => {
                self.move_position(point);
            }
            Message::Echo(output) => {
                self.echo(output);
            }
            Message::ChangeColor(color) => {
                self.change_color(color);
            }
            Message::Quit => {
                self.quit();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}