struct Config {
    // Define your configuration fields
    tabwidth: usize,
}

impl Config {
    fn new() -> Self {
        let tabwidth: usize = 4;
        Config{
            tabwidth,
        }
        // Initialize your config
    }

    // Other methods
}

struct StructA<'a> {
    config: &'a Config,
}

impl<'a> StructA<'a> {
    fn new(config: &'a Config) -> Self {
        StructA { config }
    }

    fn some_function(&self) {
        // Use self.config fields here
        println!("{}", self.config.tabwidth);
    }
}

struct StructB<'a> {
    config: &'a Config,
}

impl<'a> StructB<'a> {
    fn new(config: &'a Config) -> Self {
        StructB { config }
    }

    fn some_other_function(&self) {
        // Use self.config fields here
        println!("{} from B", self.config.tabwidth);
    }
}

fn main() {
    println!("Hello");
    let config = Config::new();
    let struct_a = StructA::new(&config);
    struct_a.some_function();

    let struct_b = StructB::new(&config);
    struct_b.some_other_function();
}