use ClockKind::{Tick, Tock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockKind {
    Tick,
    Tock,
}

#[derive(Debug, Clone, Copy)]
pub struct Clock {
    pub state: ClockKind,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            state: ClockKind::Tick,
        }
    }
    pub fn next(&mut self) -> Self {
        self.state = match self.state {
            Tick => Tock,
            Tock => Tick,
        };
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::{Clock, ClockKind};

    #[test]
    fn test_clock() {
        let mut clock = Clock::new();
        assert_eq!(clock.state, ClockKind::Tick);

        let new_state = clock.next();

        assert_eq!(clock.state, ClockKind::Tock);
        assert_eq!(new_state.state, ClockKind::Tock);
    }
}
