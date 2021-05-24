pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> where T: Draw {
    pub fn run(&self) {
        for components in self.components.iter() {
            components.draw();
        }
    }
}

pub struct Buttom {
    pub width: u32,
    pub heigth: u32,
    pub label: String,
}

impl Draw for Buttom {
    fn draw(&self) {
        // code that actually draw a buttom
    }
}