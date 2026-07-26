fn main() {
    struct Button { label: String, on_click: Box<dyn Fn(&str)> }
    impl Button {
        fn new(label: &str, on_click: impl Fn(&str) + 'static) -> Self {
            Button { label: label.to_string(), on_click: Box::new(on_click) }
        }
        fn click(&self) { (self.on_click)(&self.label); }
    }
    let b = Button::new("Submit", |label| println!("Button '{label}' clicked!"));
    b.click();
}
