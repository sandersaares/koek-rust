use std::fmt::Display;

use koek_classify::{Classified, DataClass, DisplayBehavior};

pub const NOT_SECRET_DATA: DataClass = DataClass {
    name: "Not Secret",
    display_behavior: DisplayBehavior::Clear,
};

pub const SECRET_DATA: DataClass = DataClass {
    name: "Secret",
    display_behavior: DisplayBehavior::DefaultRedact,
};

#[test]
fn direct_printing_classified_values_works() {
    let secret_string = SECRET_DATA.classify("foofoo");
    let not_secret_string = NOT_SECRET_DATA.classify("barbar");

    // At time of use, no difference in how different classes are to be displayed.
    println!("The secret value is {}", &secret_string);
    println!("The non-secret value is {}", &not_secret_string);

    // You can still get the internal value and manually use it.
    println!(
        "The value inside the secret value is {}",
        &secret_string.value
    );
    println!(
        "The value inside the non-secret  value is {}",
        &not_secret_string.value
    );
}

#[test]
fn indirect_printing_classified_values_works() {
    let secret_string = SECRET_DATA.classify("foofoo");
    let not_secret_string = NOT_SECRET_DATA.classify("barbar");

    // The same function can accept classified data purely by traits without caring which class it is.
    indirect_print(&secret_string);
    indirect_print(&not_secret_string);
}

fn indirect_print(value: &impl Display) {
    println!("The value is {}", &value);
}

#[test]
fn counting_classes_works() {
    let secret_string = SECRET_DATA.classify("foofoo");
    let not_secret_string = NOT_SECRET_DATA.classify("barbar");

    // You can make a method that counts how many of each data class (or a specific data class) it sees.
    count_class(&secret_string);
    count_class(&not_secret_string);
}

fn count_class<TValue>(value: &Classified<TValue>) {
    let class_name = value.class.name;

    println!("The class name is {}", class_name);
}
