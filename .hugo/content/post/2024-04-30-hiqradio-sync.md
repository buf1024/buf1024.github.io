---
title: "网络收音机增加同步功能"
date: 2024-04-30T16:00:10+08:00
draft: false
categories: [Rust, Flutter] 
tags: [Rust, Flutter, Radio]
---


之前写了个跨n端的网络收音机，不过一直缺少的一个功能，就是跨多端播放记录和收藏夹的同步。虽然可以通过导入导出的方式勉强可以实现某种程度的同步，但是这也太低端和繁琐了。

所以花几天时间来完善这个功能。

‍

因为服务端的性能非常差，只有256mb的内存，可以猜测一下用什么语言实现的。

‍

![mac](/img/hiqradio/hiqradio-mac.png)

‍
![android](/img/hiqradio/hiqradio-android.png)

![web](/img/hiqradio/hiqradio-web.png)

‍

web: <https://toyent.com/hiqradio/> <https://buf1024.github.io/hiqradio/> 响应式的。

android/mac(apple芯片): <https://pan.quark.cn/s/c0df18a02331#/list/share>

至于其他平台，手头没有机器，也就这样了，喜欢自己去编译：<https://github.com/buf1024/hiqradio。不过可以明确一点，全是屎山，想到什么写什么。>

服务端代码嘛，就不公开了，这个语言学习曲线有点陡。

‍

一张有意思截图，改动预计时间范围远超出自己的预期。

‍

![log](/img/hiqradio/hiqradio-log.png)
