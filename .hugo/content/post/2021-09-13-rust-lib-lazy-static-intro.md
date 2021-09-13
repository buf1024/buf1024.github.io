---
title: "lazy_static -- 静态变量延迟初始化"
date: 2021-09-13T01:12:02+08:00
draft: false
categories: [rust] 
tags: [rust-lib]
---

全局静态变量初化，在他编程语言里面，是一个很简单又自然的事，然而在rust里面并非容易。如果像下面定义全局静态变量，那么会报错：

```rust
static ref VEC_VAL: Vec<String> = vec!["1, 2, 3".into()];
```

静态变量不允许动态分配内存:
```shell
error[E0010]: allocations are not allowed in statics
 --> bin/lazy_static.rs:5:29
  |
5 | static VEC_VAL: Vec<String> = vec!["1, 2, 3".into()];
  |                             ^^^^^^^^^^^^^^^ allocation not allowed in statics
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
```

[lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static/)提供一种初始化静态全局变量的方式，通过[lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static/)宏，全局静态变量被延迟到第一次调用时才初始化，并只初始化一次。[lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static/)宏，匹配的是`static ref`，所以定义的静态变量都是不可变引用。如:

```rust
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INT_VAL: i32 = 10;
    static ref VEC_VAL: Vec<String> = vec!["1, 2, 3".into()];

    static ref MAP_VAL: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert("Hello".into(), "world".into());
        map.insert("F*k".into(), "me".into());
        map
    };
}
fn main() {
    println!("{}, {:?}", *INT_VAL, *VEC_VAL);
    MAP_VAL.iter().for_each(|(k, v)| {
        println!("key={}, value={}", k, v);
    })
}
```

输出:

```shell
10, ["1, 2, 3"]
key=Hello, value=world
key=F*k, value=me
```

