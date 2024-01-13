---
title: "rust 异步编程粗略理解 async/.await"
date: 2021-07-26T22:12:02+08:00
draft: false
categories: [rust] 
tags: [rust]


---

## 简介

[协程](/post/2014-08-21-coroutine/)已经是非常成熟的概念了，很多编程语言(比如JS, Python, DART等等)已提供原生的支持了，并且几乎提供相同的关键字`async/await`。没有提供原生支持的其他编程语言，比如C/C++等，也有类似的库实现（虽然支持并不太完美）。作为比较现代的rust编程语言，在各种编程语言比较雷同的背景之下，也不例外原生支持协程这种异步编程方式。引入协程的目的就是为了用同步的方式编写异步的代码，所以和其他语言一样，在使用上，async/.await的异常简单。比如：

```rust
use futures::executor::block_on;

async fn hello() {
    let content = async_read("a.txt").await;
    println!("{}", content);
    let content = async_read("b.txt").await;
    println!("{}", content);
}

fn main() {
    let future = hello(); 
    block_on(future);
}
```

然而，rust只提供关于异步的的最小集合的相关概念，美名rust依赖于社区，所以runtime就让社区去折腾，去实现。所以现在rust上面使用异步特性，必须引入社区的runtime。相对比较流行的runtime，有3个: [tokio](https://toko.rs), [async-std](https://github.com/async-rs/async-std)和[smol](https://github.com/smol-rs/smol)，最流行的莫属 [tokio](https://toko.rs)。对于各个runtime的优缺点和使用方式不做过多介绍，直接参考相关库文档。

rust异步代码虽然在使用上面比较简单，然而，在原理上面，对比其他语言，复杂不少，毕竟其他语言的runtime是在语言层提供的。

## 原理

对于单个核心cpu而言，同一时间只能有一个进程获取到cpu的使用权，为了使其他进程能够“同时”执行，操作系统把cpu的使用时间分成一个个时间片，以供给其他程序使用，这样操作系统就可以根据策略控制哪个程序可以使用cpu。进程退出cpu使用权，让另外一个进程使用，这称之为`上下文切换`。我们后端经常说`上下文切换`的代价是非常昂贵的，是因为在`上下文切换`需要把程序的运行状态，调用堆栈，cpu寄存器等信息保存和重新加载，这对高并发应用是非常有影响的。这种多任务的方式称之为[抢占式多任务](https://baike.baidu.com/item/%E6%8A%A2%E5%8D%A0%E5%BC%8F%E5%A4%9A%E4%BB%BB%E5%8A%A1/8675779?fr=aladdin)。协程的实现的是另外一种称为协作式多任务的方式，把上下文切换这种昂贵的操作，交给应用程序完成。这样就可以避免频繁的上下文切换导致性能的损失。更重要的是，用同步的方式写异步代码。：~

rust提供[Future](https://doc.rust-lang.org/std/future/trait.Future.html)的特性，`async`块会将块内的代码转换为实现`Future`特性的状态机，这个和js返回`Promise`类似。`Futrue`特性的代码如下:

```rust
pub trait Future {
    type Output;
    pub fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

`poll`中的第二个参数ctx，包括一个重要的内容，[Waker](https://doc.rust-lang.org/nightly/core/task/struct.Waker.html)。如果runtime会对所有的`Futrue`进行`poll`，当`poll`返回Pending时，继续轮询下一个`Futrue`，这样cpu一直处于繁忙或空转状态，所以runtime会创建waker，waker的作用是告诉runtime，`Futrue`可能已经`Ready(T)`了，让runtime对该`Futrue`进行`poll`，这用不至于空耗cpu。

以上面例子为例，编译器转换的代码可能如下：

```rust
enum HelloStateMachine {
    Start(StartState),
    WaitingOnATxt(WaitingOnATxtState),
    WaitingOnBTxt(WaitingOnATxtState),
    End(EndState),
}

impl Future for HelloStateMachine {
    type Output = （）；

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match self { 
                ExampleStateMachine::Start(state) => {…}
                ExampleStateMachine::WaitingATxt(state) => {
                  match state.foo_txt_future.poll(cx) {
                      Poll::Pending => return Poll::Pending,
                      Poll::Ready(content) => {
                          *self = ExampleStateMachine::WaitingBTxt(state);
                         return Poll::Ready(content);
                      }
                  }
                }
                ExampleStateMachine::WaitingBTxt(state) => {…}
                ExampleStateMachine::End(state) => {…}
            }
        }
    }
}
```

当然实际上编译器产生的代码更复杂，但是本质上是产生不同的状态机的代码，以上代码不涉及状态保存。状态保存涉及一个一头雾水的特性`Pin`。

`Pin`是为了解决自引用结构体的问题，因为状态定义基本上都是自引用结构体，比如状态:

```rust
struct WaitingState {
    array: ["hello", "world"],
    element: 0x1001cdd, // element为arry最后一个元素地址
}
```

当`WaitingState`在内存移动到另外一新的内存位置时，element的地址没有更新，那么会造成element的指针无效。最简单的办法是在堆分配elment，在栈中记录内存地址，这样即使无论怎么移动，element都是指向有效的内存地址。但是`mem::replace`或者`mem::swap`等这些不守规则的同学还可以改变内存地址。于是`Pin`就出来了。Pin保证了`Poll::Pending`在状态保存时，状态结构数据总指向正确的地址。

## 小结

虽然`async/.await`在使用上，不需要对其原理熟悉，但是深入了解其内部原理，对写好异步代码有莫大的帮助。由于对rust还不是非常熟悉，上面只是粗略探讨，内容不一定完全正确。
