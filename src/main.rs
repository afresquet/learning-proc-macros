use learning_proc_macro::{MetaData, Reflective};

fn main() {
    foo();
    bar();
}

#[derive(Reflective)]
struct Foo {
    a: i32,
    b: bool,
    c: String,
}

fn foo() {
    let foo = Foo {
        a: 4,
        b: false,
        c: "foo".to_string(),
    };

    println!("The name of struct: {}", foo.name());
    println!("Fields of struct: {:?}", foo.field_names());
}

#[derive(MetaData)]
#[metadata(author = "Alvaro", serial_version = 4)]
struct Bar {
    #[metadata(author = "Alvaro")]
    a: i32,
    #[metadata(author = "Alvaro")]
    b: bool,
}

fn bar() {
    let bar = Bar { a: 1, b: true };

    println!("Struct Author: {}", bar.author());
    println!("Struct Serial Version: {}", bar.serial_version());
    println!("Struct Fields Authors: {:?}", bar.fields_authors());
}
