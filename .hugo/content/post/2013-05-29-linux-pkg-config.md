---
title: Linux pkg-config
date: 2013-05-29 22:37:14
aliases: [/2013/05/29/Linux-pkg-config/]
categories: [Linux]
tags: [C, Linux]
---

pkg-config主要的作用是提供编译和连接到软件库的信息的。  

如，典型的用法：  

        [heidong@HEIDONGVM bin]$ pkg-config --cflags --libs libevent 
        -I/usr/local/include  -L/usr/local/lib -levent

将显示包含头文件和连接的信息。  

那么pkg-config是如何获取得这些信息的呢？  

pkg-config 是从xxx.pc的文件里面取的软件包的信息，如上面的例子，libevent.pc  

        [heidong@HEIDONGVM bin]$ cat /usr/local/lib/pkgconfig/libevent.pc 
        #libevent pkg-config source file
        
        prefix=/usr/local 
        exec_prefix=${prefix} 
        libdir=${exec_prefix}/lib 
        includedir=${prefix}/include
        
        Name: libevent 
        Description: libevent is an asynchronous notification event loop library 
        Version: 2.0.17-stable 
        Requires: 
        Conflicts: 
        Libs: -L${libdir} -levent 
        Libs.private: -lrt 
        Cflags: -I${includedir}

可见，pkg-config里面主要包含了包含头文件，连接库的信息。在本例子中，Requires就空的，如果非空，cfalgs, libs将添加相应的依赖项。

那么pkg-config是从哪里搜索这些文件的呢？  

首先pkg-config会从自己标准的搜索目录（如/usr/lib/pkgconfig/）和PKG_CONFIG_PATH环境变量目录寻找。
如果寻找不到，会输出这样的信息：  

        [heidong@HEIDONGVM bin]$ pkg-config --libs qt 
        Package qt was not found in the pkg-config search path. 
        Perhaps you should add the directory containing `qt.pc' 
        to the PKG_CONFIG_PATH environment variable 
        No package 'qt' found

可见pkg-config对减轻编程人员的工作负荷有一定的帮助。
