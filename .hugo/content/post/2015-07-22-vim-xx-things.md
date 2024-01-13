---
title: VIM XX THINGS
date: 2015-07-22 15:25:00
aliases: [/2015/07/22/vim-xx-things/]
categories: [Vim]
tags: [Vim]
---
### 关于中文乱码  

跟字符编码方式有关的选项:  
*encoding*, *fileencoding*, *fileencodings*, *termencoding*。更详细的信息可看vim 帮助文档: `:help encoding-names`。简略含义如下：  

1. *encoding*： Vim 内部使用的字符编码方式，包括 Vim 的 buffer (缓冲区)、菜单文本、消息文本等。  
2. *fileencoding*： Vim 中当前编辑的文件的字符编码方式  
3. *fileencodings*： Vim 启动时会按照它所列出的字符编码方式逐一探测即将打开的文件的字符编码方式，并且将 fileencoding 设置为最终探测到的字符编码方式。
4. termencoding: Vim 所工作的终端 (或者 Windows 的 Console 窗口) 的字符编码方式。  
所以在一般情况下，`~/.vimrc`里面设置如下配置，则可解决大多数中文乱码问题:

    let &termencoding=&encoding
    set fileencodings=ucs-bom,utf-8,gbk,cp936,gb18030,big5,euc-jp,euc-kr,latin1

### 关于map映射

对于`map`, `unmap`, `mapclear`而言，可以有多种组合，不带前缀的`map`在普通模式和可视模式生效，命令组合，可能有这么几种前缀:  

1. `nore` 表示非递归(可能是no recursive的缩写)  
2. `n` 表示在普通模式下生效  
3. `v` 表示在可视模式下生效  
4. `i` 表示在插入模式下生效  
5. `c` 表示在命令行模式下生效  

所以`map`有以下命令，分别作用于不同的模式:

    :map :noremap :unmap :mapclear
    :nmap :nnoremap :nunmap :nmapclear
    :vmap :vnoremap :vunmap :vmapclear
    :imap :inoremap :iunmap :imapclear
    :cmap :cnoremap :cunmap :cmapclear
