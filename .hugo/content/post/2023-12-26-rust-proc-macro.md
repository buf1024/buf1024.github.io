---
title: "rust 过程宏"
date: 2023-12-26T16:00:10+08:00
draft: false
categories: [rust] 
tags: [rust]
---
### 简介

[Rust](https://www.rust-lang.org/zh-CN/) 编程语言里面有两种宏系统，一种是声明宏（[Declarative Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming)），另一种为过程宏（[Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes)）。声明宏和过程宏是两种基本上完全不一样的宏系统，编写的方式也完全不一致，使用方式除了函数式外也不一致。关于声明宏学习，[Rust 宏小册](https://zjp-cn.github.io/tlborm/introduction.html) 里面有比较详细的说明，这里不再啰嗦。而对于过程宏，网上是可以搜索到的资料则相对较少，系统介绍学习的资料就更加少了。

过程宏所做的事情则是从输入中获**取到标记流**，**处理这些标记流或者生成新的标记流**，然后将处理后的标记流返回给编译器作下一步的处理。需要注意的是，过程宏操作的是[Rust](https://www.rust-lang.org/zh-CN/) `AST`（抽象语法树），所以即使是在宏里面，也必须是合法[Rust](https://www.rust-lang.org/zh-CN/)的语法结构。这也就意味着，解析过程宏的过程中，`var`表示的是一个合法的标识符，而`6var`则是非法的。

这篇文章是，对过程宏进行一些不完全的探讨和学习。

### 三种过程宏形式

过程宏必须是一个独立的库（很多开源项目喜欢用`xxx_derive`的名称命名），这个库只导出过程宏的函数，而这个宏是被编译器调用的。`Cargo.toml`里面必须有以下内容表明是一个过程宏：

```rust
[lib]
proc-macro = true
```

[`proc_macro`](https://doc.rust-lang.org/stable/proc_macro/index.html) 是 [Rust](https://www.rust-lang.org/zh-CN/) 编译器提供的编写过程宏所需的类型和工具，过程宏有以下三种表示形式：

#### `derive` 式

1. 函数带有`#[proc_macro_derive(Name)]` 属性或者` #[proc_macro_derive(Name, attributes(attr))]`属性
2. 函数签名为 `pub fn xxxx (proc_macro::TokenStream) -> proc_macro::TokenStream`

函数的名称叫什么并不重要，使用时是使用`proc_macro_derive`里面的名称，如下例子

```rust
#[proc_macro_derive(Getters, attributes(getter))]
pub fn getters(input: TokenStream) -> TokenStream {
  //...
}
```

使用

```rust
#[derive(Getters)]
struct Test {
  #[getter(name=get_name)]
  name: String
}
```

#### 函数式

1. 函数带有 `#[proc_macro] `属性
2. 函数签名为 `pub fn xxx (proc_macro::TokenStream) -> proc_macro::TokenStream`

函数的名称就是使用时名称，如下例子：

```rust
#[proc_macro]
pub fn lazy_static(input: TokenStream) -> TokenStream {
  //...
}
```

使用方式和声明宏调用一摸一样

```rust
lazy_static!{
 //...
}
```

#### 属性式

1. 函数带有` #[proc_macro_attribute] `属性
2. 函数签名为 `pub fn xxx(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream`

函数的名称就是使用时名称，如下例子：

```rust
#[proc_macro_attribute]
pub fn retry(attr: TokenStream, input: TokenStream) -> TokenStream {
  //...
}

```

使用方式和`Python`装饰器的使用方式类似

```rust
#[retry(times=5, timeout=60s)]
pub fn fetch_data(url: String) -> Result<MyData> {
  //...
}
```

一般来说，`derive` 式是对原有功能的扩展，**原有的声明是保留下的**，更多是在原有基础上**增加功能**，比如增加`impl`函数，增加泛型约束等等。**函数式**则更多的是用于自定义语法的解析，如果声明宏描述语法困难一般可以考虑用函数式来替代。而属性式则是完全对原有功能的改写了，属于替代性的。特别需要注意的是，**过程并不是卫生性**的，这点是和声明宏不一样。也就是说，过程宏它是会污染当前模块。所以，在过程宏里面定义或使用类型时，必须要用全路径的形式，而自定生成的函数变量等等命名也要特殊考虑，以防止污染了当前模块。

### `quote`, `syn`, `proc_macro2`以及`trybuild`和[`cargo-expand`](https://github.com/dtolnay/cargo-expand)

编写过程宏，编译器只提供[`proc_macro`](https://doc.rust-lang.org/stable/proc_macro/index.html)这个`crate`，不过它所提供的功能非常有限，单独使用这个库的话，编写比较啰嗦和麻烦。因此大神[dtolnay](https://github.com/dtolnay)提供了``, ``和``这三个库来简化过程宏的编写，有了这三个`crate`，编写过程宏就如同编写普通的代码一样，除了调试困难一点外，基本没什么差别。这三个库基本成功编写过程宏事实上的标准。

同时为了使编写过程更加温柔点而不至于暴躁，[dtolnay](https://github.com/dtolnay)提供了`trybuild`这个库用于编写宏的单元测试，[`cargo-expand`](https://github.com/dtolnay/cargo-expand)用于过程宏的展开。

#### `proc_macro2`

[`proc_macro`](https://doc.rust-lang.org/stable/proc_macro/index.html)所提供的`TokenStream`和`quote`和`syn`所处理的`TokenStream`是不兼容的，所以另外增加一个 `proc_macro2`，用于和[`proc_macro`](https://doc.rust-lang.org/stable/proc_macro/index.html)的`TokenStream`互相转换。实际上这个库的使用，只是在最后返回值里面调用一下`into`而已。

#### `quote`

`quote`是将编写的代码转换为[Rust](https://www.rust-lang.org/zh-CN/) token的方式，提供一种称之为`quasi-quoting`的方式，将代码视为数据，并可以进行插值。比较常用的是这两个宏：`parse_quote!`，`quote!`，以及`format_ident!`。

#### `syn`

这是编写过程宏最重要的一个，大部分时间都是和这个库进行打交道，它表示了一个完整的[Rust](https://www.rust-lang.org/zh-CN/) 语法，如果看语言的[Reference](https://doc.rust-lang.org/nightly/reference/)感觉到抽象，最好来这里看代码，它以非常具体的编码实现告诉你这个[Reference](https://doc.rust-lang.org/nightly/reference/)是怎么表示的。

#### `trybuild`

平常的单元测试是编译好的代码，然后再运行测试用例。然而过程宏的的测试，处需要测试是否编译通过，也需要编译出错的结果是否正确。这是需要这个`trybuild`库了。

#### [`cargo-expand`](https://github.com/dtolnay/cargo-expand)

[`cargo-expand`](https://github.com/dtolnay/cargo-expand)用于宏代码的展开，需要注意的是，需要正常编译通过的代码才可以进行展开。


使用这`quote`, `syn`, `proc_macro2`三个库来编写过程宏后，框架代码基本一致，一般有如下三个步骤，如下方式：

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // 将输入的标记解析成语法树
    let input = parse_macro_input!(input as DeriveInput);
  
	// 使用quote！进行插值处理
    let expanded = quote! {
        // ...
    };

    // 将proc_macro2的TokenStream转换为proc_macro的TokenStream
    TokenStream::from(expanded)
}
```

以下，编写三种形式是示例，来看下着三种形式的过程过程宏怎么编写以及怎么使用。示例代码仓库：[https://github.com/buf1024/my_macro_demo](https://github.com/buf1024/my_macro_demo)

### 示例：`derive` 式

假设我们需要为结构体生成一系列的`getter`函数，当然`getter`的名字是可以自定义的也可以根据默认的字段名称生成，也可以设置`getter`的可见性，同时根据是否注释生成对应的`desc`函数。不需要考虑这样的功能在实际工作中是否有意义，这里的重点是学校过程宏的编写过程。

首先编写宏就是为了使用它，所以第一步，要了解是怎么使用这个宏：

```rust
// 首先可以是这样简单使用
#[derive(Getters)]
struct MyStruct {
    data: String,
}

// 又或者想变更一下它的名称
#[derive(Getters)]
struct MyStruct {
    #[getter(name=get_fuck_data)]
    data: String,
}

// 又或者是这样
#[derive(Getters)]
struct MyStruct {
	#[getter(vis=pub(crate))]
    #[getter(name=get_fuck_data)]
    data: String,
}

// 以及可能有注释
#[derive(Getters)]
struct MyStruct {
    /// 这是一个data的属性
	#[getter(vis=pub(crate))]
    #[getter(name=get_fuck_data)]
    data: String,
}

// 设置机构体可能复杂, 带上了生命周期参数和泛型
#[derive(Getters)]
struct MyStruct<'a, T: Sync+Send+Constraint> {
	#[getter(vis=pub(crate))]
    #[getter(name=get_fuck_data)]
    data: &'a str,
    constraint: T
}
```

确定了过程宏的使用方式后，我就可以可以定义我们的导出函数了:

```rust
#[proc_macro_derive(Getters, attributes(getter))]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let token_stream = expand_getters(input);
    token_stream
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
```

我们将输入`token`流解析为`DeriveInput`，是因为`DeriveInput`实现了`Parse` trait。定义如下：

```rust
pub trait Parse: Sized {
    fn parse(input: ParseStream) -> Result<Self>;
}

pub struct DeriveInput {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub ident: Ident,
        pub generics: Generics,
        pub data: Data,
    }

pub enum Data {
        Struct(DataStruct),
        Enum(DataEnum),
        Union(DataUnion),
    }
```

从`DeriveInput`所实现的`Parse`和`DeriveInput`数据结构可以看出，`derive` 式过程宏只支持`Struct`，`Enum`和`Union`三种数据结构。

写过程宏的一个重要的工作就是获取所修饰的数据结构的基本信息，而对于`derive` 式过程宏来说，这些数据放到`attrs`这个属性里面，用`Attribute`这个结构来表示，`Meta`则是存储这样数据的。

```rust
pub struct Attribute {
        pub pound_token: Token![#],
        pub style: AttrStyle,
        pub bracket_token: token::Bracket,
        pub meta: Meta,
    }

pub enum Meta {
        Path(Path),

        /// A structured list within an attribute, like `derive(Copy, Clone)`.
        List(MetaList),

        /// A name-value pair within an attribute, like `feature = "nightly"`.
        NameValue(MetaNameValue),
    }
```

Meta是什么鬼？按照`syn`的文档：

```rust
    /// text
    /// #[derive(Copy, Clone)]
    ///   ~~~~~~Path
    ///   ^^^^^^^^^^^^^^^^^^^Meta::List
    ///
    /// #[path = "sys/windows.rs"]
    ///   ~~~~Path
    ///   ^^^^^^^^^^^^^^^^^^^^^^^Meta::NameValue
    ///
    /// #[test]
    ///   ^^^^Meta::Path
    /// 
```

需要注意的是，注释文档是解析为`#[doc = r" Single line doc comments"]`的。所以，文档注释的获取：

```rust
let doc_str: String = f
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("doc"))
                .try_fold(String::new(), |acc, attr| {
                    let mnv = match &attr.meta {
                        syn::Meta::NameValue(mnv) => mnv,
                        _ => return Err(syn::Error::new_spanned(attr, "expect name value!")),
                    };
                    let doc_str = match &mnv.value {
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }) => lit.value(),

                        _ => return Err(syn::Error::new_spanned(attr, "expect string literal!")),
                    };
                    Ok(format!("{}\n{}", acc, doc_str))
                })?;
```

前面提到`DeriveInput`实现了`Parse`trait进行解析，而我们要对`Attribute`里面的内容进行解析，则是需要实现该trait：

```rust
impl Parse for GetterMeta {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::name) {
            let _: kw::name = input.parse()?;
            let _: Token![=] = input.parse()?;

            let name: Ident = if input.peek(LitStr) {
                let sl: LitStr = input.parse()?;
                let value = sl.value();

                format_ident!("{}", value.trim())
            } else {
                input.parse()?
            };

            Ok(Self {
                name: Some(name),
                vis: None,
            })
        } else if lookahead.peek(kw::vis) {
            let _: kw::vis = input.parse()?;
            let _: Token![=] = input.parse()?;

            let vis = input.parse()?;

            Ok(Self {
                name: None,
                vis: Some(vis),
            })
        } else {
            Err(lookahead.error())
        }
    }
}
```

写法可以完全参考`DeriveInput`的写法，调用`Atrribute`的，`parse_args`或`parse_args_with`，则可以调用到`Parse`trait。

获取到基本的数据后，自然就生成代码，这里两个重要的宏: `quote!`和`parse_quote!`。最后生成的代码用`#[automatically_derived]`进行装饰，说明这是自动生成的代码。

```rust
let (impl_generic, type_generic, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generic #st_name #type_generic #where_clause {
            pub fn hello(&self) {
                println!("hello!");
            }
            #getters
        }
    })
```

过程宏写好之后，我们就要写单元测试了，当然也可以先写单元测试，再写过程宏，这称为测试用例驱动的开发模式。不过，作为示例，我们不写很详细的测试用例，我们只写两个测试用例，一个是成功的，一个是失败的，展示怎么使用即可。失败的用例，我们需要提供一个和测试文件一致，以`.stderr`结尾的输出，如果编译器的输出一致则测试通过，这个输出可以先编译出来，让编译器生成，然后自己对比是不是自己想要的结果。

过程宏使用后，是否符合自己的需要，需要宏展开来观察，使用是`cargo exand`命令：

```rust
#![allow(dead_code)]

use my_derive::Getters;

#[derive(Getters)]
struct MyStructRef<'a> {
    /// 你好呀
    #[getter(vis=pub(crate))]
    #[getter(name = "get_fuck_data")]
    data: &'a str,
}

fn main() {}



//展开代码

#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use my_derive::Getters;
struct MyStructRef<'a> {
    /// 你好呀
    #[getter(vis = pub(crate))]
    #[getter(name = "get_fuck_data")]
    data: &'a str,
}
#[automatically_derived]
impl<'a> MyStructRef<'a> {
    pub fn hello(&self) {
        {
            ::std::io::_print(format_args!("hello!\n"));
        };
    }
    pub(crate) fn get_fuck_data(&self) -> &'a str {
        &self.data
    }
    pub fn get_fuck_data_desc(&self) -> &'static str {
        "你好呀"
    }
}
fn main() {}
```

### 示例：属性式

假设我们有这样一个需求，可以对原有函数进行增加一些其他功能，比如`retry`，可以设置调用超时时间，超时后或者出错后，可以进行重新调用，类似于Python装饰器，可以考虑用属性式过程宏表示。同样，因为是测试例子，所以，没有超时和重试功能，只做怎么获取属性宏的数据和生成代码。

首先确定调用的形式：

```rust
#[retry(times=5, timeout=60)]
fn remote_request(a: i32, b: i32) -> i32 {
    println!("@remote_request!");
    a + b
}
```

再确定其展开形式：

```rust
// cargo exapnd 生成的
fn remote_request(a: i32, b: i32) -> i32 {
    fn __new_remote_request(a: i32, b: i32) -> i32 {
        {
            ::std::io::_print(format_args!("@remote_request!\n"));
        };
        a + b
    }
    for _ in 0..5 {
        __new_remote_request(a, b);
    }
    for _ in 0..60 {
        __new_remote_request(a, b);
    }
    __new_remote_request(a, b)
}
```

属性式的签名是这个样子的：

```rust
#[proc_macro_attribute]
pub fn retry(attr: TokenStream, input: TokenStream) -> TokenStream
```

属性和输入流已经作为参数传递给我们了，而我需所要做的是需要将属性解析出来和对原函数的重写。

```rust
    let item_fn = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(attr as Args);
```

和`derive`式的一样，通过实现`Parse`trait来解析：

```rust
impl Parse for RetryAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::times) {
            let _: kw::times = input.parse()?;
            let _: Token![=] = input.parse()?;

            let times: LitInt = if input.peek(LitInt) {
                input.parse()?
            } else {
                return Err(lookahead.error());
            };

            Ok(Self {
                times: Some(times),
                timeout: None,
            })
        } else if lookahead.peek(kw::timeout) {
            let _: kw::timeout = input.parse()?;
            let _: Token![=] = input.parse()?;

            let timeout: LitInt = if input.peek(LitInt) {
                input.parse()?
            } else {
                return Err(lookahead.error());
            };

            Ok(Self {
                times: None,
                timeout: Some(timeout),
            })
        } else {
            Err(lookahead.error())
        }
    }
}
```

### 示例：函数式

假设我们要计算二元二次方程组的值，我们计划是这样使用的：

```rust
let (x, y) = formula!(1 * x + 1 * y = 2, 2 * x + 1 * y = 9);
```

这个宏直接就在编译期间就计算出x，y的值（当然是存在有解的情况下），无解就`panic`。为了使问题简单，我们假设x，y前面都是有系数的。过程宏的操作过程就是解析出里面的表达式：

```rust
pub(crate) struct FormulaArgs {
    formula: Vec<Formula>,
}

impl Parse for FormulaArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let attrs = Punctuated::<Formula, Token![,]>::parse_terminated(input)?;
        let formula: Vec<_> = attrs.into_iter().collect();
        if formula.len() != 2 {
            panic!("require two formula")
        }
        Ok(FormulaArgs { formula })
    }
}

#[derive(Default)]
pub(crate) struct Formula {
    x: i32,
    y: i32,
    rs: i32,
}

impl Parse for Formula {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        let x = if lookahead.peek(LitInt) {
            let x_lit: LitInt = input.parse()?;
            let x: i32 = x_lit.to_string().parse().unwrap();
            let _: Token![*] = input.parse()?;
            let _: kw::x = input.parse()?;

            x
        } else {
            return Err(lookahead.error());
        };

        let r1: Result<Token![+]> = input.parse();
        let r2: Result<Token![-]> = input.parse();
        let factor = match (r1, r2) {
            (Ok(_), Err(_)) => 1,
            (Err(_), Ok(_)) => -1,
            (_, _) => return Err(lookahead.error()),
        };

        // let factor = if lookahead.peek(Token![+]) {
        //     let _: Token![+] = input.parse()?;
        //     1
        // } else if lookahead.peek(Token![-]) {
        //     let _: Token![-] = input.parse()?;
        //     -1
        // } else {
        //     return Err(lookahead.error());
        // };

        let y = if lookahead.peek(LitInt) {
            let y_lit: LitInt = input.parse()?;
            let y: i32 = y_lit.to_string().parse().unwrap();
            let _: Token![*] = input.parse()?;
            let _: kw::y = input.parse()?;
            y * factor
        } else {
            return Err(lookahead.error());
        };

        let _: Token![=] = input.parse()?;

        let rs_lit: LitInt = input.parse()?;
        let rs: i32 = rs_lit.to_string().parse().unwrap();

        if x == 0 && y == 0 && rs != 0 {
            return Err(syn::Error::new_spanned(rs_lit, "invalid equal"));
        }

        Ok(Self { x, y, rs })
    }
}
```

当解析出所需要的数据后，直接就可以进行插值生成自己所需要的数据。

### 总结

过程宏的编写过程有两个步骤。

首先是解析出过程宏所需要的信息，这个步骤一般是通过实现`syn`所提供的`Parse`trait实现的。由于syn表示的也是合法的语法结构，所以并不是所以的写法都是支持的。有时候，解析的时候出出现一些莫名奇妙解析不了的问题，比如解析二元方程时：

```rust
       // let factor = if lookahead.peek(Token![+]) {
        //     let _: Token![+] = input.parse()?;
        //     1
        // } else if lookahead.peek(Token![-]) {
        //     let _: Token![-] = input.parse()?;
        //     -1
        // } else {
        //     return Err(lookahead.error());
        // };
```

这段代码总是无法解析成功，原因未解，或者使用姿势不对，又或者是`syn`可能潜在有`bug`。不过都有变通的方法实现。

其次解析出自己所要的数据后，就可以根据具体的情况进行插值处理，只有是使用到`quote`这个库，而大多数情况之下只使用到两个宏: `quote!`和`parse_quote!`。当然也不是说没有坑。比如说解析出函数的`block`时，再重组时，要加上大扩号。再比如，解析出行数调用列表时，解析成一个元组表达式，而插值时，可以放个函数名称在前面，就变成了函数调用。而这些都是变成过程宏编写的惯例吧，习惯就好。

错误处理是给宏的使用者看的，友好的错误提示很容易就让调用者知道哪里错了。而使用错误处理是比较简单的，直接掉用`syn::Error`生成一个`Span`即可，`Span`也是可以`combine`的里面有个`token`的参数。不过如果不知道`token`的情况之下怎么处理呢？目前自己的做法是`panic`，这并非是一种明智的方式。或者有更加灵活的处理方式。

使用`cargo expand`是可以查看到宏展开的内容，不过`cargo expand`的问题是过程宏没有问题时，才可以正常的展开，出现编译问题不展开，这也就造成了调试过程宏的困难，目前也没有什么好的办法去解决。

总体来说，过程宏的编写并不是非常困难，`syn`表示了一个完整的[Rust](https://www.rust-lang.org/zh-CN/)语法，查看里面语法的表示，比看语言[Reference](https://doc.rust-lang.org/nightly/reference/items.html)强太多了，而这对于更深入了解声明宏的[片段分类符](https://doc.rust-lang.org/nightly/reference/macros-by-example.html#metavariables)更有帮助！[proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop) 是大神[dtolnay](https://github.com/dtolnay)设计的过程宏系统，全部做出来，估计写过程宏没有什么问题了。

再贴一下示例代码仓库：[https://github.com/buf1024/my_macro_demo](https://github.com/buf1024/my_macro_demo)
