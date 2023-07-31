fn main() {
    let clouser_annotated = |i: i32| -> i32 { i + 1 };
    let clouser_inferred = |i| i + 1;

    let i = 1;

    let example_clouser = |x| x;

    let s = example_clouser(String::from("hello"));

    let n = example_clouser(5.to_string());

    let m = String::new();

    let update_string = move || println!("{}", s);

    exec(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f();
}
