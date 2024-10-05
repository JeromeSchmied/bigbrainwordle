use ratatui::crossterm::{
    self,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Correct,
    WrongPos,
    Wrong,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Ch {
    ch: char,
    state: Option<State>,
}
impl Default for Ch {
    fn default() -> Self {
        Self {
            ch: ' ',
            state: None,
        }
    }
}
impl From<char> for Ch {
    fn from(ch: char) -> Self {
        Self { ch, state: None }
    }
}
impl Ch {
    fn state(self, state: State) -> Self {
        Self {
            ch: self.ch,
            state: Some(state),
        }
    }

    fn print(self) {
        let color = self.state.map_or(Color::White, |s| s.into());
        crossterm::execute!(
            std::io::stdout(),
            // left wrapper
            SetForegroundColor(color),
            Print("▐"),
            //
            SetForegroundColor(Color::Black),
            SetBackgroundColor(color),
            Print(self.ch.to_ascii_uppercase()),
            SetBackgroundColor(Color::Reset),
            // right wrapper
            SetForegroundColor(color),
            Print("▌"),
            // seperator
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
            Print(' '),
        )
        .unwrap();
    }
}

// WARN: don't use, it won't be coloured
// impl Display for Ch {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let color = self.state.map_or(Color::Reset, |s| s.into());
//         write!(
//             f,
//             "{l_wrapper}{}{r_wrapper}",
//             self.ch.to_string().bg(color),
//             l_wrapper = "▐".fg(color),
//             r_wrapper = "▌".fg(color)
//         )?;
//         Ok(())
//     }
// }

impl From<State> for Color {
    fn from(state: State) -> Self {
        match state {
            State::Correct => Color::Green,
            State::WrongPos => Color::Yellow,
            State::Wrong => Color::DarkGrey,
        }
    }
}

fn main() {
    let words = [
        [
            Ch::from('t').state(State::Wrong),
            Ch::from('r').state(State::WrongPos),
            Ch::from('a').state(State::Wrong),
            Ch::from('c').state(State::Wrong),
            Ch::from('e').state(State::WrongPos),
        ],
        [
            Ch::from('w').state(State::Wrong),
            Ch::from('e').state(State::WrongPos),
            Ch::from('i').state(State::WrongPos),
            Ch::from('r').state(State::WrongPos),
            Ch::from('d').state(State::Wrong),
        ],
        [
            Ch::from('p').state(State::Wrong),
            Ch::from('r').state(State::WrongPos),
            Ch::from('i').state(State::WrongPos),
            Ch::from('z').state(State::Wrong),
            Ch::from('e').state(State::WrongPos),
        ],
        [
            Ch::from('r').state(State::Wrong),
            Ch::from('i').state(State::Correct),
            Ch::from('v').state(State::Wrong),
            Ch::from('e').state(State::Correct),
            Ch::from('r').state(State::Correct),
        ],
        [
            Ch::from('m').state(State::Correct),
            Ch::from('i').state(State::Correct),
            Ch::from('n').state(State::Correct),
            Ch::from('e').state(State::Correct),
            Ch::from('r').state(State::Correct),
        ],
        [
            Ch::default(),
            Ch::default(),
            Ch::default(),
            Ch::default(),
            Ch::default(),
        ],
    ];

    for word in words {
        for ch in word {
            ch.print();
        }
        println!("\n")
    }
}
