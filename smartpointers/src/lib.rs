pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    pub fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}