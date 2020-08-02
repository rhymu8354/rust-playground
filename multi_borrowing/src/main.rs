struct Context {
    name: String,
}

impl Context {
    fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    fn add_prefix_by_ref(
        &mut self,
        prefix: &str
    ) {
        self.name = prefix.to_owned() + &self.name;
    }

    fn add_prefix_by_value(
        &mut self,
        prefix: String
    ) {
        self.name = prefix + &self.name;
    }

    fn report(&self) {
        println!("The name is {}.", self.name);
    }
}

fn main() {
    let mut context = Context::new(String::from("Foo"));
    context.report();
    context.add_prefix_by_ref(
        "Bar"
    );
    context.report();
    context.add_prefix_by_value(
        context.name.clone()
    );
    context.report();
    context.add_prefix_by_ref(
        &context.name
    );
    context.report();
}
