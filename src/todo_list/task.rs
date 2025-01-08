#[derive(Clone)]
pub struct Task {
    title: String,
    done: bool,
}

impl Task {
    pub(crate) fn new(title: String, done: bool) -> Self {
        Self {title, done}
    }

    pub fn to_string(&self) -> String {
        self.title.clone()
    }

    pub fn get_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, state: bool) {
        self.done = state;
    }
}