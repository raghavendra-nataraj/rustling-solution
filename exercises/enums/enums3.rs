// enums3.rs
// Address all the TODOs to make the tests pass!

enum Message {
    // TODO: implement the message variant types based on their usage below
    Quit,
    ChangeColor(i32,i32,i32),
    Echo(String),
    Move{ x:i32, y:i32},
}

struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message
        {
            Message::ChangeColor(x,y,z) => { println!("Change Color")},
            Message::Echo(st) => {println!("Echo")},
            Message::Move{x,y} => {println!("Move")},
            Message::Quit => {println!("Quit")},
            _ => {println!("defulat")}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State{
            quit: true,
            position: Point{ x: 10, y: 15 },
            color: (255, 0, 255)
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move{x: 10, y: 15 });
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

}
