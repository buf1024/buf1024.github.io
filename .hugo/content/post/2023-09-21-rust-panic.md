---
title: "一行代码让 rustc panic"
date: 2023-09-21T18:00:10+08:00
draft: false
categories: [rust] 
tags: [rust]
---

今天遇到一个非常特殊的问题，源自于以下一段代码：

```rust
// 敏感信息去掉
format!("https://xxxget?\
         a=(ab%3D%22code%22)&X_=xadf0",
         code = &code[2..]);
```

这是一段拼接url的代码，以上代码是错误的写法，`%3D%22code%22`​这里应该是`b%3D%22{code}%22`​，这样才能传递参数进行格式化，由于自己的粗心写错了。

​`rls`​没有检测出来，本来传说中的非常强大的编译应该检测出来，只不过这个简单的问题，却导致编译器`panic`​了!并生成了`rustc-ice`​文件，并让我提交`issue`​。

```rust
thread 'rustc' panicked at compiler/rustc_builtin_macros/src/format_foreign.rs:272:55:
invalid format num `"22600000"`
stack backtrace:
   0:        0x1063bb42f - std::backtrace::Backtrace::create::h276e52d394804866
   1:        0x1063bb375 - std::backtrace::Backtrace::force_capture::hf01754881169428b
   2:        0x112daf7e1 - std[1347dba3eeb2185a]::panicking::update_hook::<alloc[1cfb2b8206dd332]::boxed::Box<rustc_driver_impl[38518a20a95be9f6]::install_ice_hook::{closure#0}>>::{closure#0}
   3:        0x1063d6596 - std::panicking::rust_panic_with_hook::hc06cecb314d2e534
   4:        0x1063d632d - std::panicking::begin_panic_handler::{{closure}}::hb0f4c27b60ad4000
   5:        0x1063d3009 - std::sys_common::backtrace::__rust_end_short_backtrace::h62f21dd221c17a86
   6:        0x1063d605d - _rust_begin_unwind
   7:        0x10645e395 - core::panicking::panic_fmt::h74b5171bcf212936
   8:        0x11290024c - <rustc_builtin_macros[c397c70d617caee1]::format_foreign::printf::Substitutions as core[a5946ab14a20e1f4]::iter::traits::iterator::Iterator>::next
   9:        0x11290e77c - rustc_builtin_macros[c397c70d617caee1]::format::make_format_args
  10:        0x1129117cb - rustc_builtin_macros[c397c70d617caee1]::format::expand_format_args_impl
  11:        0x112f18c3b - <rustc_expand[33739c2ad659c2f6]::expand::MacroExpander>::fully_expand_fragment
  12:        0x112f17760 - <rustc_expand[33739c2ad659c2f6]::expand::MacroExpander>::expand_crate
  13:        0x1135a2ccd - <rustc_session[ae81351961406b05]::session::Session>::time::<rustc_ast[95b967775af3a4bd]::ast::Crate, rustc_interface[9ea207dfe3565402]::passes::configure_and_expand::{closure#1}>
  14:        0x11359cf1a - rustc_interface[9ea207dfe3565402]::passes::resolver_for_lowering
  15:        0x11411385d - rustc_query_impl[91ec14b0b4344d37]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[91ec14b0b4344d37]::query_impl::resolver_for_lowering::dynamic_query::{closure#2}::{closure#0}, rustc_middle[ac208c32f4486d9f]::query::erase::Erased<[u8; 8usize]>>
  16:        0x1141578a9 - <rustc_query_impl[91ec14b0b4344d37]::query_impl::resolver_for_lowering::dynamic_query::{closure#2} as core[a5946ab14a20e1f4]::ops::function::FnOnce<(rustc_middle[ac208c32f4486d9f]::ty::context::TyCtxt, ())>>::call_once
  17:        0x11406730f - rustc_query_system[f5b2597d247dc7ad]::query::plumbing::try_execute_query::<rustc_query_impl[91ec14b0b4344d37]::DynamicConfig<rustc_query_system[f5b2597d247dc7ad]::query::caches::SingleCache<rustc_middle[ac208c32f4486d9f]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[91ec14b0b4344d37]::plumbing::QueryCtxt, true>
  18:        0x114173ee3 - rustc_query_impl[91ec14b0b4344d37]::query_impl::resolver_for_lowering::get_query_incr::__rust_end_short_backtrace
  19:        0x112dc37e5 - <rustc_middle[ac208c32f4486d9f]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[38518a20a95be9f6]::run_compiler::{closure#1}::{closure#2}::{closure#4}, &rustc_data_structures[6dcb2c1aaf93e55a]::steal::Steal<(rustc_middle[ac208c32f4486d9f]::ty::ResolverAstLowering, alloc[1cfb2b8206dd332]::rc::Rc<rustc_ast[95b967775af3a4bd]::ast::Crate>)>>
  20:        0x112d79485 - rustc_span[78dc5708a39e8e6c]::set_source_map::<core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>, rustc_interface[9ea207dfe3565402]::interface::run_compiler<core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>, rustc_driver_impl[38518a20a95be9f6]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  21:        0x112dae7a3 - std[1347dba3eeb2185a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9ea207dfe3565402]::util::run_in_thread_with_globals<rustc_interface[9ea207dfe3565402]::interface::run_compiler<core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>, rustc_driver_impl[38518a20a95be9f6]::run_compiler::{closure#1}>::{closure#0}, core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>>
  22:        0x112dafb7e - <<std[1347dba3eeb2185a]::thread::Builder>::spawn_unchecked_<rustc_interface[9ea207dfe3565402]::util::run_in_thread_with_globals<rustc_interface[9ea207dfe3565402]::interface::run_compiler<core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>, rustc_driver_impl[38518a20a95be9f6]::run_compiler::{closure#1}>::{closure#0}, core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a5946ab14a20e1f4]::result::Result<(), rustc_span[78dc5708a39e8e6c]::ErrorGuaranteed>>::{closure#1} as core[a5946ab14a20e1f4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:        0x1063e0439 - std::sys::unix::thread::Thread::new::thread_start::hab308b3b916baab6
  24:     0x7ff8169484e1 - __pthread_start


rustc version: 1.74.0-nightly (3223b0b5e 2023-09-20)
platform: x86_64-apple-darwin

query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
```

从这个`panic`​信息，想定位到代码哪一行出问题，是无法下手的，只有编译器的调用堆栈。既然提示说是编译器的BUG，并让我提交issue，那么或者更新编译器可能可以解决。不过更新之后，问题依旧，查了一下rust仓库的issue，并没有类似的问题。

本着不服输的姿态，我要定位到到底是哪一行代码导出编译器`panic`​了，于是便像之前写C++那样，一行一行代码的删除，直到最后定位到文中开头的那一行。原来就是自己的粗心导致的，稍微做一下修改便可以让编译器正常通过。

然而，无论如何，编译器都不能挂的。难道强大的rust编译器就那么脆弱，一个简单的`format`​宏就能让编译器挂了？于是，便把那一行单独的拿出来试，这个时候，奇怪的事情发生了，它居然能正确之处问题的所在，它又展示了它的强大之处：

```rust
error: named argument never used
 --> a.rs:7:18
  |
7 |             code="abc");
  |                  ^^^^^ named argument never used
  |
note: format specifiers use curly braces, and the conversion specifier `D` is unknown or unsupported
 --> a.rs:5:20
  |
5 |             filter=(SE%3D%22123%22)&\
  |                    ^^^
note: format specifiers use curly braces, and the conversion specifier `%` is unknown or unsupported
 --> a.rs:5:23
  |
5 |             filter=(SE%3D%22123%22)&\
  |                       ^^^^^^^
  = note: printf formatting is not supported; see the documentation for `std::fmt`

error: aborting due to previous error

```

尝试了很久，在单独的文件，或cargo工程之中，并不能让该问题复现。不过，在原来代码的项目中，它又挂了一次又一次。

所以，这个编译器的`panic`​，是哪特殊的上下文下面，才能复现，至于哪个特殊的上下文，这估计不是我的能力范围。也因此，不能提供一个稳定的复现bug的场景，也就没有提交issue，代码比较大。

李某琦先生，我实实在在的在这个简单的问题上面，耗费了一个下午，这并非我不努力，而是我努力了也没有进展。
