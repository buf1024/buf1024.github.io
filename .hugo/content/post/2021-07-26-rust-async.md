---
title: "rust 异步编程 async/.await"
date: 2021-07-26T22:12:02+08:00
draft: true
categories: [rust] 
tags: [rust]


---

## 简介

[协程](/post/2014-08-21-coroutine/)已经是非常成熟的概念了，很多编程语言(比如JS, PYTHON, DART等等)已提供原生的支持了，并且几乎提供相同的关键字`async/await`，没有提供原生支持的其他编程语言，比如C/C++等，也有类似的库实现（支持并不完美）。作为比较现代的rust编程语言，也不例外原生支持协程这种异步编程方式。引入协程的目的就是为了用同步的方式编写异步的代码，所以和其他语言一样，async/.await的使用都异常简单。比如：

```rust
use futures::executor::block_on;

async fn hello() {
    println!("hello, world!");
}

fn main() {
    let future = hello(); 
    block_on(future);
}
```

rust想个大懒虫，只提供关于异步的的最小集合，runtime就让社区去折腾去实现，因而runtime就有3个相对比较流行的: [tokio](https://toko.rs), [async-std](https://github.com/async-rs/async-std)和[smol](https://github.com/smol-rs/smol)。对于使用方式不做过多介绍，异步代码的编写方式和其他语言基本类似。

## 原理

`async`块会将块内的代码转换为实现`Future`特性的状态机，这个和js返回`Promise`类似。`Futrue`特性的代码如下:

```rust
pub trait Future {
    type Output;
    pub fn poll(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

