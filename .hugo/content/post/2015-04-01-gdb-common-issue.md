---
title: gdb一丢丢常见问题
date: 2015-04-01 17:12:10
categories: [gdb]
tags: [gdb]
---

一、gdb 无法自动加载当前文件 .gdbinit文件

提示：

warning: File "/home/heidong/privt/proj/osv6/.gdbinit" auto-loading has been declined by your `auto-load safe-path' set to "$debugdir:$datadir/auto-load".

To enable execution of this file add

    add-auto-load-safe-path /home/heidong/privt/proj/osv6/.gdbinit

line to your configuration file "/home/heidong/.gdbinit".

To completely disable this security protection add

    set auto-load safe-path /

line to your configuration file "/home/heidong/.gdbinit".

For more information about this security protection see the

"Auto-loading safe path" section in the GDB manual.  E.g., run from the shell:

    info "(gdb)Auto-loading safe path"

解决：

1. You can add the line:

set auto-load safe-path /

To your home directory’s .gdbinit file. This will override ALL security and GDB will ALWAYS open a .gdbinit file if it finds one. If you are developing on your own machine, or a secure machine (however you would like to define that), then this is probably the quickest and easiest solution.

2. If you want to be a little more careful, you can add the directories individually to your home directory’s .gdbinit file:

set auto-load safe-path /home/faye/workspace/todo/Debug

Or you can say that ALL sub-paths under your home directory are OK:

set auto-load safe-path /home/faye

3. Finally, you can pass a path in on start-up:

gdb -iex "set auto-load safe-path /path/to/.gdbinit/file"

If you want to see all the trusted paths, just type:

show auto-load safe-path

二、printf打印不全

set print element 0

1. 默认设置下，在调试多进程程序时GDB只会调试主进程。但是GDB（>V7.0）支持多进程的分别以及同时调试，换句话说，GDB可以同时调试多个程序。只需要设置follow-fork-mode(默认值：parent)和detach-on-fork（默认值：on）即可。

    follow-fork-mode  detach-on-fork   说明

parent                   on               只调试主进程（GDB默认）

child                     on               只调试子进程

parent                   off              同时调试两个进程，gdb跟主进程，子进程block在fork位置

child                     off              同时调试两个进程，gdb跟子进程，主进程block在fork位置

   设置方法：set follow-fork-mode [parent|child]   set detach-on-fork [on|off]

   查询正在调试的进程：info inferiors

   切换调试的进程： inferior

   添加新的调试进程： add-inferior [-copies n] [-exec executable] ,可以用file executable来分配给inferior可执行文件。

   其他：remove-inferiors infno， detach inferior

线程调试命令

(gdb)info threads

显示当前可调试的所有线程，每个线程会有一个GDB为其分配的ID，后面操作线程的时候会用到这个ID。

前面有*的是当前调试的线程。

(gdb)thread ID

切换当前调试的线程为指定ID的线程。

(gdb)thread apply ID1 ID2 command

让一个或者多个线程执行GDB命令command。

(gdb)thread apply all command

让所有被调试线程执行GDB命令command。

(gdb)set scheduler-locking off|on|step

估计是实际使用过多线程调试的人都可以发现，在使用step或者continue命令调试当前被调试线程的时候，其他线程也是同时执行的，怎么只让被调试程序执行呢？通过这个命令就可以实现这个需求。

off 不锁定任何线程，也就是所有线程都执行，这是默认值。

on 只有当前被调试程序会执行。

step 在单步的时候，除了next过一个函数的情况(熟悉情况的人可能知道，这其实是一个设置断点然后continue的行为)以外，只有当前线程会执行。

//显示线程堆栈信息

(gdb) bt

察看所有的调用栈

(gdb) f 3

调用框层次

(gdb) i locals

显示所有当前调用栈的所有变量。