# **`fn_once`**

This library provides a convenient derive macro for a once function.

[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/fn_once/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/fn_once.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/fn_once?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee
[ci-url]: https://github.com/just-do-halee/fn_once/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/fn_once
[license-url]: https://github.com/just-do-halee/fn_once

```toml
fn_once = "0.1.0"
```

## **`How to use,`**

```rust
use fn_once::once;

#[once]
fn print_once(name: &str) {
    println!("Hello! {}!", name);
}

#[once(or = { number })] // or = { ... }
fn add_one_once(number: u32) -> u32 {
    count + 1
}

#[once(panic)]
fn panic_when_twice() { }

fn main() {
    print_once("World"); // print "Hello! World!"
    print_once("foo"); // no effect

    let mut number = 0;
    number = add_one_once(number); // 0 + 1 -> 1
    number = add_one_once(number); // no effect -> 1
    number = add_one_once(number); // no effect -> 1
    assert_eq!(number, 1);

    panic_when_twice();
    panic_when_twice(); // panic!
}
```
