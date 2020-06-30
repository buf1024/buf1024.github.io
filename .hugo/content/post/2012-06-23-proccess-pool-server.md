---
title: 基于libevent进程池服务器
date: 2012-06-23 11:08:14
aliases: [/2012/06/23/proccess-pool-server/]
categories: [linux]
tags: [c, tcp, libevent]
---

本程序仅是为了学习libevent而写的，但可以当作是简单的程序框架扩展为实际的应用。程序本身没有经过广泛的功能测试和压力测试，仅作简单的功能测试，如果在实际应用中产生任何问题与作者无关，也不提供任何技术支持。但欢迎讨论相关技术问题。  

本程序设计思路很简单，一个控制进程，M个业务进程（M可以配置），其中一个业务进程又负责N个TCP连接（N可以配置，因为是在同一个进程内，又没有用于线程，所以从本质来说，还是属于同步传输数据的）。控制进程负责监听连接，当接收到一个TCP连接的时候，按照一定的规则分发到空闲的业务进程中。每个业务进程就负责处理各个分配到的连接，接收到数据后又分发到业务模块，业务模块是可以根据实际的需要扩展的（本程序中则是一个回射服务器的）。  

这个程序且叫一个名字：jmm。下载：[jmm](https://github.com/buf1024/jmm/archive/jmm-1.0.0.zip)  

压缩包里一些特别的文件介绍。  

|文件名|说明|  
|----|:----|  
|Makefile|	Makefile 文件。如果是扩展应用，则修改第6行 <>JMM_MY_OBJ=myserver.o 改为自己的要编译的文件即可。|  
|jmm.conf|	一个简单的配置。|  
|myserver.c|	业务实现模块。这个就是简单的回射而已。|  
|jmm_cmmhdr.h|	对外提供的扩展程序的头文件，包含一些常量的定义。|  
|jmm.h|	对外提供的扩展程序的头文件|  
|clog.h <br>clog.c|	简单的日志处理模块。可独立应用于任何应用程序。|  
|ciniconfig.h <br>ciniconfig.c|	简单的读取INI配置文件的模块。可独立应用于任何应用程序。|  

编译说明：

1. 安装libevent. 下载地址：[libevent.org](www.libevent.org)    
2. 建立文件夹mkdir conf log bin  
3. 最后make可以了。  
4. 如果会Makefile直接看Makefile  

扩展说明：  

        typedef struct jmm_hook 
        { 
        const char* (*prog_name)(); 
        const char* (*prog_version)(); 
        const char* (*prog_desc)();
        
        int (*prog_init)(char*); 
        int (*prog_service)(jmm_prog_in*, jmm_prog_out*); 
        int (*prog_uninit)();
        
        void* (*prog_malloc)(size_t); // sys/types.h 
        void (*prog_free)(void*);
        
        }jmm_hook;
        
        typedef void (*prog_init_hook)(jmm_hook* hook);

除了分配内存的那个外，提供其它所有函数的实现。  

用REG_INIT_HOOK_FUN(myserver_init_hook)注册回调函数。  

参考myserver.c  

烦躁，不写了。  
