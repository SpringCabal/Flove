use alloc::collections::BTreeSet;
use core::cell::{Cell, RefCell};

#[derive(Default)]
pub struct RoundProjectiles {
    round: Cell<u32>,
    shots: RefCell<BTreeSet<i32>>,
}

impl RoundProjectiles {
    fn sync_round(&self, round: u32) {
        if self.round.replace(round) != round {
            self.shots.borrow_mut().clear();
        }
    }

    pub fn created(&self, round: u32, projectile: i32) {
        self.sync_round(round);
        self.shots.borrow_mut().insert(projectile);
    }

    pub fn destroyed(&self, round: u32, projectile: i32) -> bool {
        self.sync_round(round);
        self.shots.borrow_mut().remove(&projectile)
    }
}
