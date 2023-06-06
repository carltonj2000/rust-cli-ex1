use rust_cli_ex1::Person;

fn main() {
    let person: Person = Person::get_person();
    person.show_person();
}
