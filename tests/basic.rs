use fn_once::once;

#[once(or = { count })]
fn bar(count: i8) -> i8 {
    println!("count({}) + 1", count);
    count + 1
}

#[once]
fn bar2(name: &str) {
    println!("Hello! {}", name);
}

#[test]
fn test() {
    let mut count = 0;
    count = bar(count); // + 1 = 1
    count = bar(count); // 1
    count = bar(count); // 1
    bar2("world");
    bar2("forld");
    assert_eq!(count, 1);
}
