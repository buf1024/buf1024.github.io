---
title: "config-rs -- 多种数据配置格式"
date: 2021-08-02T10:12:02+08:00
draft: false
categories: [rust] 
tags: [rust-lib]
---
## 简介

读取配置文件是一个比较常见的需求，不过不同的人可能会选择不同的配置文件格式，有的人选择toml，有的人选择ini，又或者是从环境变量钟获取。所以，对应读取特定的文件格式，应用程序一般用选用特定文件格式的库。[config-rs](https://github.com/mehcode/config-rs)提供一种比较全面(支持主流的各种文件格式)而使用简单的方式。

## 示例

假设有以下配置文件：

```toml
# 文件
#examples/config/default.toml
#examples/config/dev.toml
#examples/config/pro.toml
#分别对应default配置，开发和生产配置
# 配置内容都是以下，键一样，值不一样，甚至default缺失一些值
[database]
url = "postgres://postgres@localhost-def-ini"

[secret]
key = "def-key-ini"
token = "def-token-ini"
```

```rust
use config::{Config, Environment, File};
use std::env;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Secret {
    key: String,
    token: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Settings {
    debug: bool,
    database: Database,
    secret: Secret,
}


fn main() {
    let run_mode = env::var("RUN_MODE").unwrap_or("dev".into());

   // add_source后面的，会覆盖前面的
    let s = Config::builder()
     // 默认配置
        .add_source(File::with_name("examples/config/default"))
     // pro/dev 配置会覆盖默认配置
        .add_source(File::with_name(&fORMat!("example/config/{}", run_mode)).required(false))
     // 注意环境变量一APP_开头的，如果APP_DEBUG，大小写无关
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    println!("debug: {:?}", s.get_bool("debug"));
    println!("database: {:?}", s.get::<String>("database.url"));
    let settings: Settings = s.try_deserialize().unwrap();

    println!("settings: {:?}", settings);
}

```

输出:

```rust
$ APP_debug=0 cargo run --color=always --package rust-me --example config
debug: Ok(false)
database: Ok("postgres://postgres@localhost-def-ini")
settings: Settings { debug: false, database: Database { url: "postgres://postgres@localhost-def-ini" }, secret: Secret { key: "def-key-ini", token: "def-token-ini", version: 120 } }

```

可见，使用太简单了。
