---
title: 'AT&T Stype GAS Assembly'
date: 2016-02-02 14:13:05
aliases: [/2016/02/02/AT-T-Stype-GAS-Assembly/]
categories: [Asm]
tags: [Asm, gas]
---
简单回顾了下IA-32汇编语言，大学教科书学的8086是基于Intel语法的，使用的是nasm，这次整理了一下AT&T格式的，使用的是gas。当然，如果熟悉一种，另外一种语法格式问题应该不大，nasm也可以是跨平台。不过，gnu asm 是GCC toolchain的一部分，自然而然，在Linux/unix平台下，首选它了。

整理的不太完整，也不完全正确，且还不括X8087 FPU部分，和一些高级点的指令。IA-64处理器，可以按同样的方法理解部分指令，同样，也不完全的。

### [IA-32寄存器](/raw/asm/IA32.mmap)

![IA-32寄存器](/raw/asm/IA32.png)

### [GASM 指令](/raw/asm/GAS.mmap)

![IA-32寄存器](/raw/asm/GAS.png)
