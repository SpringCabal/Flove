#[derive(Debug, Clone, Default)]
pub struct VictoryManager {
    pub game_over: bool,
    pub won: bool,
}

impl VictoryManager {
    pub fn notify_spire_destroyed(&mut self) {
        if self.game_over {
            return;
        }
        self.game_over = true;
        self.won = false;
    }

    pub fn declare_victory(&mut self) {
        if self.game_over {
            return;
        }
        self.game_over = true;
        self.won = true;
    }
}
