---
title: "vim一些小技巧"
date: 2019-03-01T18:02:12+08:00
draft: false
categories: [vim] 
tags: [vim]
---

### 命令行编辑

```c
q:打开命令窗口

CTL+R 加上寄存器名称，复制寄存器内容
```

### 关于函数跳转：

   注意VIM，编辑源代码时，先生成tag文件，和设置好include路径。

[+ctrl+i 跳转到函数、变量和#define

 [+ctrl+d 跳转到#define处

ctrl+i 向前跳到前几次光标位置

ctrl+o 向后跳到后几次光标位置

gf 跳转到对应到头文件后，跳回ctrl+6

### 关于map映射

对于map, unmap, mapclear而言，可以有多种组合，不带前缀的map在普通模式和可视模式生效，命令组合，可能有这么几种前缀:

1. nore 表示非递归(可能是no recursive的缩写)
2. n 表示在普通模式下生效
3. v 表示在可视模式下生效
4. i 表示在插入模式下生效
5. c 表示在命令行模式下生效

所以map有以下命令，分别作用于不同的模式:

```c
:map :noremap :unmap :mapclear
:nmap :nnoremap :nunmap :nmapclear
:vmap :vnoremap :vunmap :vmapclear
:imap :inoremap :iunmap :imapclear
:cmap :cnoremap :cunmap :cmapclear
```

### 其他杂项

1. gu或者gU

nvim 打开系统剪切板

1. 安装以下任一软件

* xclip
* xsel (newer alternative to xclip)
* pbcopy/pbpaste (only for Mac OS X)

2. .vimrc添加

 set clipboard+=unnamedplus