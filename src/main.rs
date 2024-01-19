use learning_proc_macro::Reflective;

#[derive(Reflective)]
struct Foo {
    a: i32,
    b: bool,
    c: String,
}

fn main() {
    let foo = Foo {
        a: 4,
        b: false,
        c: "foo".to_string(),
    };

    println!("The name of struct: {}", foo.name());
    println!("Fields of struct: {:?}", foo.field_names());
}
