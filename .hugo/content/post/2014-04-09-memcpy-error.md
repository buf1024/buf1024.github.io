---
title: memcpy造成其他变量值改变
date: 2014-04-09 19:12:17
categories: [misc]
tags: [C]
---

以前C/C++遇到过很多很多的问题，有时候费了很久很久时间，很大很大的努力才解决。但是都没有记录下来，现在想想以前的遇到的问题，只知道这样的事情发生过，至于是什么问题，一点印象都没有了。所以遇到奇怪的问题，还是记录下来靠谱点。

背景：

使用OTL访问ORACLE数据库，在POWER CPU 64位LINUX上面程序看起来运行没有什么异常情况。移植到X86构架的CPU 64位LINUX上面，却发现某条SQL无法执行。初步分析后（分析错误），发觉执行某条SQL后，某变量值莫名其妙变了。发邮件问OTL作者，作者看得也不是很仔细，一时也发现不了什么问题。

最后继续看几次代码，发现还是自己程序的问题：

具体如下：


```c
// wrong:
memcpy(info.usr_name, u_info.usr_name, sizeof(info.usr_name));
memcpy(info.usr_no, u_info.usr_num, sizeof(info.usr_name)); 
memcpy(info.bank_acct, rcd.bank_acct, sizeof(info.bank_acct));
```

```
```

```c
correct:
memcpy(info.usr_name, u_info.usr_name, sizeof(info.usr_name));
memcpy(info.usr_no, u_info.usr_num, sizeof(info.usr_no));
memcpy(info.bank_acct, rcd.bank_acct, sizeof(info.bank_acct));
```


info 和rcd为两个结构体，局部临时变量。由于中间一个memcpy拷贝长度错了，造成其他变量值改变，具体改变情况和CPU构架大小端相关，在POWER CPU上面不出问题纯属侥幸。