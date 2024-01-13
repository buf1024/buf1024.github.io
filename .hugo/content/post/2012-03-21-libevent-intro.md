---
title: libevent 概述
date: 2012-03-21 21:05:10
categories: [C++]
tags: [C++, libevent]
---
声明，本文章东搬西扯抄来过的，并非原创，写给自己参考的。

关于libevent的描述，有两个百科的连接可以参考：

 [libevent百度百科](http://baike.baidu.com/view/1590523.htm)

 [libevent维基百科](http://zh.wikipedia.org/wiki/Libevent)

在维基百科时，有几个有用的连接可以参考：

 [libevent 2.0参考书籍（英文）](http://www.wangafu.net/~nickm/libevent-book/)

还有另一个竞争力的事件库：

 [libev(另一个有竞争力的事件库)](http://software.schmorp.de/pkg/libev.html)

libevent是一个事件触发的网络库，适用于windows、Linux、bsd等多种平台，内部使用select、epoll、kqueue等系统调用管理事件机制。libevent支持用户使用三种类型的事件，分别是网络IO、定时器、信号三种。Libevent提供了DNS，HTTP Server，RPC等组件。

# 相关组件

| 组件               | 说明                                                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| evutil             | 不同平台网络实现的通用功能函数。                                                                                        |
| event 和event_base | libevent的核心。提供与平台无关的事件驱动的非阻塞IO后端的API。它可以让你知道socket何时可读可写，超时处理和系统信号探测。 |
| bufferevent        | 提供event_base更方便的封装函数。提供缓冲支持。bufferevent同样提供多种IO后端。                                           |
| evbuffer           | bufferevent的缓冲机制底层实现。并提供更方便和高效的缓冲访问。                                                           |
| evhttp             | 简单的HTTP客户端服务端实现。                                                                                            |
| evdns              | 简单的DNS客户端服务端实现。                                                                                             |
| evrpc              | 简单的RPC客户端服务端实现。                                                                                             |

# 相关库

| 库               | 说明                                                                                               |
| ------------------ | ---------------------------------------------------------------------------------------------------- |
| libevent_core    | 核心事件和缓冲功能。包含所有的event_base, evbuffer, bufferevent, 和工具函数。                      |
| libevent_extra   | 协议相关的功能，包含HTTP, DNS, 和RPC。                                                             |
| libevent         | 包含libevent_core和libevent_extra。由于历史的原因而存在。新应用程序不应该连接这个库。              |
| libevent_pthread | 依赖事pthread线程库，提供线程和锁实现。当你使用pthread多线程时，才连接这个库。（只在某些平台存在） |
| libevent_openssl | 依赖于OpenSSL库的加密通信支持。当你使用加密通信时，才连接这个库。（只在某些平台存在）              |

# 相关头文件

        所有的头文件安装在event2目录下。头文件大类别如下表：

| 类别         | 说明                                                                                      |
| -------------- | ------------------------------------------------------------------------------------------- |
| API头文件    | 这个是libevent的对外头文件，文件没有任何后辍。                                            |
| 兼容头文件   | 兼容头文件包含抛弃的函数定义，除非是为了移值旧的程序，否则不要使用。文件以_compat.h结尾。 |
| 结构体头文件 | libevent结构体定义的头文件。文件以_struct.h结尾。                                         |

        除了上述提到的头文件外，还有一些为了兼容旧程序而使用的头文件。它们不放在event2目录。放在这个目录的上一层。这些文件其实是新文件的包装而已。对应该关系如下表。

| 旧头文件 | 被替换的头文件                         |
| ---------- | ---------------------------------------- |
| event.h  | event2/event*.h, event2/buffer*.h, event2/bufferedevent*.h, event2/tag*.h |
| evdns.h  | event2/dns*.h                          |
| evhttp.h | event2/http*.h                         |
| rvrpc    | event2/rpc*.h                          |
| evutil   | event2/util*.h                         |

# 其它

        本文大多剽窃自[libevent-book](http://www.wangafu.net/~nickm/libevent-book/)。具体请详细参考之。另外很多懒人不太习惯Linux的编程调试环境，所以根据libevent提供的Makefile建了个VC2008的解决方案，直接解压出来丢到libevent目录，分别编译各个库。另，只为了调试搞DEBUG版本的，RELEASE的动都没动，sameple工程包括所有的例子了，直接编译是不通的。所以如果你要分别测试的话，要搞掉其它的。最简单的方法就是#if 0 #endif之类的。[下载VC2008解决方案](http://download.imlgc.com/lib/libevent/vcbuild.zip)。
