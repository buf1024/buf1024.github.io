---
title: "nom -- 乐高式富有语义的parser"
date: 2021-09-12T21:12:02+08:00
draft: false
categories: [rust] 
tags: [rust-lib]
---

## 简介

写过parser的人，不管是简单的自定义协议，或者复杂的协议，一般都是采用自上往下的解释方式，从第1个字节，一路开黑，到最后字节。遇到`;`用一个判断，遇到`:`用一个`match`等等，`switch`相应的`case`，所谓遇神拜神，遇鬼杀鬼，遇佛却不知所措。这样的问题是，加上错误处理，`if else`可能会过于复杂而凌乱，时间久了，难以维护。稍微高端点的，可能会写出几个复杂一点的正则表达式，不过最后也很有可能，最终忘记当初写这正则的含义。高端玩家估计就用`lex/yacc flex/bison`，的确好用又好维护，除了增加一下描述文件，增加一些与开发语言无关的东西。不过杀鸡焉用牛刀，这么庞大的工具，有必要割本来就小的小JJ？！

[nom](https://github.com/Geal/nom)提供一种比较清奇的思路，估计作者深受乐高的熏陶，才有这种创造。我们知道，乐高玩具，提供的是很限的几个基础形状模块，通过一个个模块的组合起来，就可以实现各种物件的创作，栩栩如生。[nom](https://github.com/Geal/nom)的思路并不是教你如何像`lex/yacc`构造特定语法的模板文件，也不期待你自顶而下解释，而是像乐高一样，提供很多基础的基础形状，如`tag, take_while1, is_a`等，并提供组合一起的方法，如`alt, tupple, preceded`等。用各种方法可以组合，形成各式各样的parser，而又不损失任何性能。

[nom](https://github.com/Geal/nom)函数的设计高度一致，几乎所有函数的调用，都返回带相同签名的函数:

```rust
// 一般函数返回
impl Fn(Input) -> IResult<Input, Output, Error>

// IResult定义
pub type IResult<I, O, E = error::Error<I>> = Result<(I, O), Err<E>>;

// ParseError<I> 为 () 实现了，所以一般可以用()，方便使用ParseError
impl<I> ParseError<I> for ()
```

[nom](https://github.com/Geal/nom)的基础函数构造块，分两个版本，一个是`complete`和`stream`版本，两个直观上最大的区别是报错方式，`stream`版本报错为`Err::Incomplete(n)`，`complete`直接报`Err::Error/Err::Failure`。除此之后，使用上并没有明显的差异。

[nom](https://github.com/Geal/nom)的parser和combinator应有尽有，没有的也可以组合出来。除了[docs.rs](https://docs.rs)的文档，作者还贴心的列出来(因为rust生成的文档不方便一起看)，这里不累赘，参考作者介绍[choosing_a_combinator](https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md)。

## 示例

[tokio](https://tokio.rs)的官方教程[tutorial](https://tokio.rs/tokio/tutorial)是学习rust很好的一个开始的地方，把之前`C/C++`已有的概念用rust实现了一遍，既亲切又熟悉。[tokio](https://tokio.rs)的官方教程[tutorial](https://tokio.rs/tokio/tutorial)是实现简单redis功能，代码仓库位于: [mini-redis](https://github.com/tokio-rs/mini-redis)。 [mini-redis](https://github.com/tokio-rs/mini-redis)对于redis协议的解析是通过一个字节解析的，幸好简单，所以原实现并不凌乱。现修改为用[nom](https://github.com/Geal/nom)解析方式，代码仓库位于: [https://github.com/buf1024/buf1024.github.io/tree/master/.demo/rust-lib/mini-redis](https://github.com/buf1024/buf1024.github.io/tree/master/.demo/rust-lib/mini-redis)，协议解析的文件：[frame.rs](https://github.com/buf1024/blog-demo/blob/master/rust-lib/mini-redis/src/frame.rs)

```rust
/// A frame in the Redis protocol.
/// 协议格式
#[derive(Clone, Debug)]
pub enum Frame {
    // +xxx\r\n 简单的string
    Simple(String),
    // -xxx\r\n 错误的string
    Error(String),
    // :ddd\r\n u64数值
    Integer(u64),
    // $dd\r\nbbbbb\r\n 有内容的chunk
    Bulk(Bytes),
    // $-1\r\n 空Chunk
    Null,
    // *dd\r\nxxx\r\n array dd 我数量
    Array(Vec<Frame>),
}
```

mini-redis简单实现了redis的5个基本协议，实现并不复杂，拼拼凑凑就成了，魔改的[nom](https://github.com/Geal/nom)的版本如下:

```rust
fn parse_simple(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = delimited(tag("+"), take_until1("\r\n"), tag("\r\n"))(src)?;
        Ok((input, (Frame::Simple(String::from(output)), src.len() - input.len())))
    }

    fn parse_error(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = delimited(tag("-"), take_until1("\r\n"), tag("\r\n"))(src)?;
        Ok((input, (Frame::Error(String::from(output)), src.len() - input.len())))
    }

    fn parse_decimal(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = map_res(
            delimited(tag(":"), take_until1("\r\n"), tag("\r\n")),
            |s: &str| u64::from_str_radix(s, 10),
        )(src)?;
        Ok((input, (Frame::Integer(output), src.len() - input.len())))
    }

    fn parse_bulk(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) = alt((
            map(tag("$-1\r\n"), |_| Frame::Null),
            map(terminated(length_value(
                map_res(
                    delimited(tag("$"), take_until1("\r\n"), tag("\r\n")),
                    |s: &str| u64::from_str_radix(s, 10)),
                rest),
                           tag("\r\n"),
            ), |out| {
                let data = Bytes::copy_from_slice(out.as_bytes());
                Frame::Bulk(data)
            }))
        )(src)?;
        Ok((input, (output, src.len() - input.len())))
    }

    fn parse_array(src: &str) -> IResult<&str, (Frame, usize)>
    {
        let (input, output) =
            map(length_count(
                map_res(
                    delimited(tag("*"), take_until1("\r\n"), tag("\r\n")),
                    |s: &str| {
                        println!("{}", s);
                        u64::from_str_radix(s, 10)
                    }),
                Frame::parse,
            ), |out| {
                let data = out.iter().map(|item| item.0.clone()).collect();
                Frame::Array(data)
            },
            )(src)?;
        Ok((input, (output, src.len() - input.len())))
    }

    pub fn parse(src: &str) -> IResult<&str, (Frame, usize)>
    {
        alt((Frame::parse_simple, Frame::parse_error, Frame::parse_decimal, Frame::parse_bulk, Frame::parse_array))(src)
    }
```

可以看到[nom](https://github.com/Geal/nom)的版本有很简洁的方式，甚至一行代码即可实现，不需要原代码那样，一个字节一个字节的判断，来操控和移动内存位置。就类似乐高积木一样，一个一个的叠起来，简单又简洁。

测试：

```shell
^_^@~/rust-lib/mini-redis]$ RUST_LOG=debug cargo run --bin mini-redis-server
   Compiling mini-redis v0.4.0 (~/rust-lib/mini-redis)
    Finished dev [unoptimized + debuginfo] target(s) in 7.44s
     Running `target/debug/mini-redis-server`
Sep 13 00:10:16.517  INFO mini_redis::server: accepting inbound connections
Sep 13 00:10:40.261 DEBUG mini_redis::server: accept address from 127.0.0.1:55931
5
Sep 13 00:10:40.264 DEBUG run: mini_redis::connection: nom frame: Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world"), Bulk(b"px"), Integer(3600)]), n: 50
Sep 13 00:10:40.264 DEBUG run: mini_redis::server: cmd=Set(Set { key: "hello", value: b"world", expire: Some(3.6s) })
Sep 13 00:10:40.264 DEBUG run:apply: mini_redis::cmd::set: response=Simple("OK")
^CSep 13 00:10:52.278 DEBUG my_exit: mini_redis_server: press once more to exit
^CSep 13 00:10:52.934 DEBUG my_exit: mini_redis_server: now exit
Sep 13 00:10:52.934  INFO mini_redis::server: shutting down

^_^@~/rust-lib/mini-redis]$ RUST_LOG=debug cargo run --bin mini-redis-cli set hello world 3600 
   Compiling mini-redis v0.4.0 (~/rust-lib/mini-redis)
    Finished dev [unoptimized + debuginfo] target(s) in 1.61s
     Running `target/debug/mini-redis-cli set hello world 3600`
Sep 13 00:10:40.262 DEBUG set_expires{key="hello" value=b"world" expiration=3.6s}: mini_redis::client: request=Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world"), Bulk(b"px"), Integer(3600)])
Sep 13 00:10:40.265 DEBUG set_expires{key="hello" value=b"world" expiration=3.6s}: mini_redis::connection: nom frame: Simple("OK"), n: 5
Sep 13 00:10:40.265 DEBUG set_expires{key="hello" value=b"world" expiration=3.6s}: mini_redis::client: response=Some(Simple("OK"))
OK

```

## 小结

rust的[nom](https://github.com/Geal/nom)提供一种类似于乐高积木的方式去构造解析器，通过组合各种基本的parser，完全可以构造出足够复杂的解析器，同时不会因为组合各种parser，而失去任何性能。同时，得益于其命名方式和统一的函数返回形式，基于[nom](https://github.com/Geal/nom)的解析器，语义上比那些`get_line`之类的，清晰太多。所以在解析任务上，除了有现成而又成熟的解析库，完全可以考虑采用的，而非`get_line，get_u8`那样一个字节的或者正则那样一个模式的，开黑下去……

