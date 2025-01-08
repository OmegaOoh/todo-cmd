pub struct Task {
    title: String,
    done: bool,
}

impl Task {
    pub(crate) fn new(title: String) -> Self {
        Self {title, done: false}
    }

    pub fn to_string(&self) -> String {
        self.title.clone() + " " + if self.done {"done"} else {""}
    }

    fn toggle_done(&mut self) {
        self.done = !self.done;
    }

    fn set_done(&mut self, state: bool) {
        self.done = state;
    }

}