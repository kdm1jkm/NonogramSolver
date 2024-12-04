use std::{cell::RefCell, rc::Rc};

use super::solver_display::{SolverDisplay, SolverState};

pub trait DisplayHandler {
    fn get_display(&self) -> Option<Rc<RefCell<Box<dyn SolverDisplay>>>>;

    fn update_progress(&self, progress: (usize, usize)) {
        if let Some(display) = self.get_display() {
            display.borrow_mut().update_progress(progress);
        }
    }

    fn change_state(&self, state: SolverState) {
        if let Some(display) = self.get_display() {
            display.borrow_mut().change_state(state);
        }
    }
}
