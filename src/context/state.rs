#[derive(Debug, Clone)]
pub struct State<T: Clone> {
    value: T
}

impl<T> State<T> where T: Clone {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }

    pub fn get(&self) -> T {
        return self.value.clone();
    }

    pub fn set(&mut self, value: T) where T: Clone {
        self.value = value;
    }
}