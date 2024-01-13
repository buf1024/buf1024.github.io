---
title: "Rust sqlx SQLBuilder"
date: 2024-01-13T16:00:10+08:00
draft: false
categories: [Rust] 
tags: [Rust, Database]
---

对于个人来说，一直不喜欢`ORM`​框架。明明就是熟悉`sql`​的，为何还要封装一层？抛开针对各个数据库厂商难以作`sql`​ 优化之外，封装之后的写法也不见得优雅多少，而且每个`ORM`​框架写法有差异，还要针对各个`ORM`​框架学习一种新的语法和规则，不是蛋疼么？不过，裸操`sql`​也难免太过暴力，不断的重写模板代码，也是挺操蛋的。所以类似于`MyBatis`​之类的半自动框架，或者说这类`sql builder`​挺收欢迎的。对于`rust`​生态来说，[sqlx](https://github.com/launchbadge/sqlx)是比较收欢迎的框架，很多`ORM`​也是基于它来实现的。

​`sqlx`​是纯`rust`​编写的，异步的可选不同的`runtime`​，而它最大的特点就编译期间可以进行`sql`​检查，如果下面，字段名写错了，直接就被`rls`​，检测出来。

​![sqlx](/img/rust/sqlx-error.png)​

### SQLx CLI

​`sqlx`​自带一个client，`sqlx-cli`​用于简单的数据库管理和迁移。具体的使用方式参考自带的文档：[https://github.com/launchbadge/sqlx/tree/main/sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

不过如果，使用该方式会在数据库产生额外的表，该表用于迁移管理：

```rust
mysql> desc _sqlx_migrations;
+----------------+------------+------+-----+-------------------+-------------------+
| Field          | Type       | Null | Key | Default           | Extra             |
+----------------+------------+------+-----+-------------------+-------------------+
| version        | bigint     | NO   | PRI | NULL              |                   |
| description    | text       | NO   |     | NULL              |                   |
| installed_on   | timestamp  | NO   |     | CURRENT_TIMESTAMP | DEFAULT_GENERATED |
| success        | tinyint(1) | NO   |     | NULL              |                   |
| checksum       | blob       | NO   |     | NULL              |                   |
| execution_time | bigint     | NO   |     | NULL              |                   |
+----------------+------------+------+-----+-------------------+-------------------+
```

按照文档说明：`sqlx migrate add <name>`​添加`-r`​选项可以在生成的文件后面增加`down/up`​后缀名，用于`revert`​回退，实际上的操作并不会，可以手工添加。不过`sqlx migrate run`​是按时间顺直接执行的，直接新建一个，也是可以实现类似回退的效果。

这种开发的方式是否适合国内的情况呢？感觉这个还是有待商榷的。不一定是所有的模块都是用rust编写，让人难以接受的时候增加一张`_sqlx_migrations`​表，对数据库强入侵。另外一个就是`migrations`​文件可以记录到数据库设计的变迁历史，可是国内的大多数情况之下，运维那一侧更加希望的是由统一的一个`sql`​，让他们执行，而不是由程序一个`migrate run`​。这样对数库权限控制基本就是忽视的了。

因此是否使用这个`sqlx-cli`​就要根据实际情况觉定。

### Query(CURD)

查询的有两种，一种是`unprepared`​，另外一种是`prepared`​的。两种使用方式：

​`// low-level, Executor trait
conn.execute("BEGIN").await?; // unprepared, simple query
conn.execute(sqlx::query("DELETE FROM table")).await?; // prepared, cached query`​

两者之间的区别在在于一个是裸字符串，另外一个是`sqlx`​函数包裹。我们大多情况之下都会使用`prepared`​的，因为性能较高。

对于`prepared`​的查询，也有两种使用方式，一种是使用宏，另外一种是使用api。如：

```rust
// api调用  
let rows: Vec<_> = sqlx::query_as::<_, User>("select name, age from user where ?=?")
        .bind(1)
        .bind(1)
        .fetch_all(&mut *tx)
        .await?;
// 宏
    let last_id = sqlx::query!("insert into user(name, age) values(?, ?)", "世界和平", 120)
        .execute(&mut *tx)
        .await?
        .last_insert_id();
```

两者是有明显的区别。使用宏的方式，是需要配置`.env`​指定数据库的url，在**编译期间实时运行**。不过使用宏，也有一个问题：如果你使用的是`Any`​数据库驱动，想根据连接字符串来区别不同的数据库厂商，那么宏的方式就会直接报错。这个问题在2021年就提出了：[https://github.com/launchbadge/sqlx/issues/964](https://github.com/launchbadge/sqlx/issues/964)，并且也有了解决的方案，可是现在两年多过去了，一直没有按照解决的方案实现，可能这个需求比较小众。

使用api的方式也是有一个问题，英文不同的数据库厂商返回的数据是不一定一样的，比如有些数据库，他是没有返回`last_insert_id`​的。所以，如果是要写此类代码，还是需要注意差异。

将查询结果转换为`struct`​基本就实现了半自动框架功能了，操作的方式就是实现`sqlx::FromRow`​这个`trait`​，以及用宏的方式提供了，如：

```rust
#[derive(sqlx::FromRow, Debug)]
struct User {
    name: String,
    age: i32,
}
```

**事务**操作方式也是挺简单：

```rust
let mut tx = pool.begin().await?;
let last_id = sqlx::query!("insert into user(name, age) values(?, ?)", "世界和平", 120)
        .execute(&mut *tx)
        .await?
        .last_insert_id();
tx.commit().await?;
// 或
tx.rollback().await?;
  
```

注意`.execute(&mut *tx)`​的调用，`Transaction`​对象实现了`deref`​，所以可以传递给`execute`​。

### 小结

以`MyBatis`​作为对比，`sqlx`​基本具备`MyBatis`​的基本功能。在此基础上并且还提供了额外的，如编译期间检测，`sqlx-cli`​数据库管理等功能。虽然这些额外功能在正常的开发之中不一定能用的上，不过这也不失为他作为一个强大的`sql builder`​。

‍
