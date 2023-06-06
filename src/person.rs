pub mod person {
    pub use clap::Parser;

    #[derive(Parser)]
    pub struct Person {
        /// Enter name here
        #[arg(short, long, default_value_t = String::from("Carlton"))]
        pub name: String,

        /// Enter age here
        #[arg(long, short, default_value_t = 57)]
        pub age: usize,
    }

    impl Person {
        pub fn get_person() -> Person {
            Person::parse()
        }
        pub fn show_person(&self) {
            println!("{} is {} years old", self.name, self.age);
        }
    }
}
