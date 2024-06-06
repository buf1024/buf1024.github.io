---
title: lwan http 服务器
date: 2017-04-28 22:10:32
aliases: [/2017/04/28/lwan-http-server/]
categories: [C]
tags: [Http, C, Linux]
---
### 简介

服务器端程序的设计模式一般比较固定，目前主要有两种主要的解决方式，一种多进程方式(进程池)，另外一种多线程的方式(线程池)(P.S:请忘掉一个连接或一个业务fork一个进程，或起一个线程的方式，系统开销太大)，两种方式都采用了非阻塞io，多路复用(P.S:Linux平台为最高效的`epoll`，新型api，如`signalfd`, `timerfd`, `eventfd`等，也可以进行`epoll`)。

对于多进程的模式，有些实现是，起一个主进程进行监听和管理，再起N个(N=cpu个数)业务进程，当主进程`accept`成功后，根据一定的业务逻辑，将fd发送到业务进程(P.S:发送方式用`socketpair`，参考：[sock.c](https://github.com/buf1024/reinvent-wheel/blob/master/common/sock.c) `unix_domain_send_fd`和`unix_domain_recv_fd`)，业务进程对接收到到fd进行`epoll`。对于多进程，另外一些实现是，起N个(N=cpu个数)业务进程，同时N个进行地位同等，同时进行`epoll`，`listen`，`accept`等(P.S:多进行进行监听将引起群惊效应，但只有一个进程`accept`成功)。而多线程的模式，则和多进程模式类似，有一个主线程进行监听和管理，再起N个(N=cpu个数)业务线程，**并将业务线程的调度级别设置为系统级别**，这点非常重要，各个业务线程程进行进行`epoll`。  

多进程和多线程并无本质区别，多线程的优点个人看来，数据都在整个进程空间，线程内数据都可以共享，需要留意共享数据的互斥访问，缺点就是调试起来稍微麻烦，而多进程的优点在于个人看来，调试相对多线程方便，缺点就是不方便运维。(P.S:旧同事入职一家导航的公司，他们的网络框架就是多线程模式，而公司的，多数是多进程模式。旧同事提起，所以现在再回头看看[lwan](https://github.com/lpereira/lwan), 当时只是看，没记录下来，现在重新记录一下)

[lwan](https://github.com/lpereira/lwan)是个非常轻量级的http服务器(P.S:检验你是否掌握了C语言的标准，是否能完整的实现一个http服务器)，代码量只有2W多行，比起成熟稳定的[nginx](http://nginx.org/)代码量少太多了，当然功能也少很多。[lwan](https://github.com/lpereira/lwan)在16年5月份左右一段时间在C语言项目排行榜star增加数排前列。个人觉得，[lwan](https://github.com/lpereira/lwan)，它采用多线程模式，遵循了高并发服务设计的最基本模式，自己设计一个比较有特色的[trie](http://baike.baidu.com/link?url=3ojcT0gOp_oV_uDCAr0IYmAl6AziaFrQNodhpkS1DPpB7euef3LideO12NpZYhJWhuHYIw-2JqSlUUw_pvP9pa)树的实现，可扩展的模块编写，同时创新的使用了协程（协程，自己在很久之前也关注过，参考: [coroutine协程](https://buf1024.github.io/2014/08/21/coroutine/)），这是最大的特点。当然，到目前为止，它还不稳定，从自己关注提出的第一个issue, 还有很多BUG，代码有一些不好的风格（如相对比较多的`goto`语句，一言不合就`abort`）。尽管如此，它还是一个非常值得学习的web服务器。下面简单粗略回顾下[lwan](https://github.com/lpereira/lwan)，详细了解的话，参考代码，自己也fork了一份，加了部分注释[lwan](https://github.com/buf1024/lwan)。(P.S:自己叫别人去学习，没理由自己忘记了)

### 大致工作流程

[lwan](https://github.com/bu24/lwan)的工作模式是非常典型的多线程模型，粗略画一下它的工作流程：
![lwan工作流程](/img/lwan/workflow.png)
[lwan](https://github.com/bu24/lwan)总共包括，一个低级别的job线程，主要于资源清理，一个主线程，主要用于接收连接和分发连接，以及N(N可配置，一般等于CPU个数)个调度级别为进程级别的业务处理线程。[lwan](https://github.com/bu24/lwan)进行线程初始化是，使用到了`pthread_barrier_*`这种数据结构，`pthread_barrier_*`其实只做且只能做一件事，就是充当栏杆（barrier意为栏杆)。形象的说就是把先后到达的多个线程挡在同一栏杆前，直到所有线程到齐，然后撤下栏杆同时放行。1）`init`函数负责指定要等待的线程个数；2） `wait`函数由每个线程主动调用，它告诉栏杆“我到起跑线前了”。`wait`执行末尾栏杆会检查是否所有人都到栏杆前了，如果是，栏杆就消失所有线程继续执行下一句代码；如果不是，则所有已到`wait`的线程停在该函数不动，剩下没执行到wait()的线程继续执行；3）`destroy`函数释放init申请的资源。

job线程在每一次循环调用`sleep`类的函数，这类函数有个致命的缺点，那就是不能响应信号，这就是导致[lwan](https://github.com/bu24/lwan)退出非常慢的原因。使用`pthread_cond_wait`可以避免该问题，已经修改，并进行了pull request：[quick quit optimize](https://github.com/lpereira/lwan/pull/199)。业务处理线程里面全部使用协程进行处理。

### 数据结构

一般程序，使用比较多的两个数据结构一个是`list`链表，另外一个是`hashtable`。`list`没有太多可讲的，而`hashtable`的实现，在这里可能也不是最优的。`hashtable`比较高效，而且经过大量检验的是redis的实现方式，推荐看这个，公司内部使用的库也是修改这个而来。网上对[dict.c](https://github.com/antirez/redis/blob/4.0/src/dict.c)的分析很多，如：[Redis 源码分析（1）：字典和哈希表（dict.c 和 dict.h）](http://huangz.iteye.com/blog/1455808)。这里不累赘了，大体来说：hash 的典型实现方法，定义一个假如长度为len的数组，每个数组里面为leaf的节点，leaf节点为一链表结构。插入和查询就是，先计算key的hash，hash值取模len，得出数据所在的位置，添加到leaf节点后（插入），或者在从leaf链表里面遍历（查询，由于hash冲突），得到最终的结果。redis的`dict`包括两个这个样的`hashtable`，首先插入数据从0号`hashtable`插入，插入时创建，当0号`hashtable`增长到一定的程度或比例，那么就会创建1号`hashtable`，并将0号`hashtable`迁移到这个`hashtable`，如果正在发生迁移，那么新插入的数据就会插入1号`hashtable`，0号到1号`hashtable`的迁移是分摊到`dict`的每个操作之中的，并非采用集中式的迁移。当0号`hashtable`全部迁移到1号`hashtable`时，那么将0号`hashtable`删除，并将1号`hashtable`变为0号`hashtable`，1号`hashtable`变为NULL。然后在重复这样一个过程。

[trie](http://baike.baidu.com/link?url=zKneAm7OsFA-2VblcQ21Grh7ZYBSnSkaycwjc9q1Z5IbMNU0tdaqi71hgVjx7ZrO81aKwVYdnOAqOGl3VbU3U_)树是这里的另外一个重要的数据结构，很多web框架都严重依赖这种数据结构，个人觉得，这里实现的`trie`还是比较高效的：

```C
// trie数据结构定义
struct lwan_trie_node {
    struct lwan_trie_node *next[8];
    struct lwan_trie_leaf *leaf;
    int ref_count;
};

struct lwan_trie_leaf {
    char *key;
    void *data;
    struct lwan_trie_leaf *next;
};

struct lwan_trie {
    struct lwan_trie_node *root;
    void (*free_node)(void *data);
};

//查找或插入时，就已经确认位置了    /* Traverse the trie, allocating nodes if necessary */
    for (knode = &trie->root; *key; knode = &node->next[(int)(*key++ & 7)])
        GET_NODE();

```

比如，5，58,12,123,1234,12345在内存的存储情况（简化，假设`struct lwan_trie_node *next[4]`）：
![trie内存存储](/img/lwan/trie.png)

### 协程coro

业务处理线程里面全部使用协程进行处理的。协程应该属于[lwan](https://github.com/bu24/lwan)的核心了。协程可以参考[coroutine协程](https://buf1024.github.io/2014/08/21/coroutine/)，里面描述这里的有很大区别，但本质上都是都对`context`不同程度封装，或用汇编，或直接用系统函数。这里的coro定义了一个`struct coro_defer_array defer;`，用于`connection`超时或协程被清理时，内部资源的清理。具体协程不再详述。

协程本身概念并不难复杂，如果熟悉这种编程模式，那么非阻塞编程模式将可以用顺序编程的方式来思考，对程序员的思考的角度和业务逻辑的理解将有很大的提升(非阻塞的方式时，当业务流程需要进行阻塞时，比如需要连接第三方系统请求数据，那么业务当前的状态可能需要保存下来，当第三方系统应答时，再恢复状态，进行下一步处理，这样，业务流程就变成一段一段的了)。但是协程并没有大规模的使用，[风云](http://blog.codingnow.com/)也曾介绍过，在推广协成概念时，遇到了很多问题(具体出处已忘记)。  

使用协程必须适当评估程序`STACK`的大小，[lwan](https://github.com/lpereira/lwan)里面是固定定义为`#define PTHREAD_STACK_MIN 16384`，协程分配的`STACK`为`((3 * (PTHREAD_STACK_MIN)) / 2)`或`#define CORO_STACK_MIN         (5 * (PTHREAD_STACK_MIN))`，这样那么如果在协程切换函数里面定义一个比较大的临时变量，如`char tmp[1024000]`那么程序必挂了，因为栈溢出了。所以使用协程需要考虑`STACK`或考虑动态分配内存，动态分配内存还需要考虑释放问题(参考 lwan-coro.c `coro_defer`)。另外一个问题是，程序coredump时，用gdb进行调试，发现用`bt`进行调用堆栈查看时，有部分是乱的，不方便问题定位。我想这些有可能是协程没有被大规模推广的原因之一吧？

### 模块处理

[lwan](https://github.com/lpereira/lwan)通过模块进行自身的扩展，自己本身也实现了一些模块。`lwan-mod`开头的文件都是它的模块。个人编写模块，只有实现相关的函数，直接编译成动态库即可。

```C
// 模块需实现函数，模块名可在配置文件进行配置，返回const struct lwan_module *结构体指针
const struct lwan_module *lwan_module_模块名(void);
/*lwan 模块是函数名为 lwan_module_模块名的函数，内置模块使用-rdynamic编译进可执行程序内*/
struct lwan_module {
    void *(*init)(const char *prefix, void *args);
    void *(*init_from_hash)(const char *prefix, const struct hash *hash);
    void (*shutdown)(void *data); 
    bool (*parse_conf)(void *data, struct config *config);
    // http 请求处理函数
    enum lwan_http_status (*handle)(struct lwan_request *request, struct lwan_response *response, void *data);
    enum lwan_handler_flags flags; // 处理标志
};
```

自己实现一个helloworld模块可参考：[helloworld](https://github.com/buf1024/lwan/blob/master/common/lwan-mod-helloworld.c)

### 配置文件

[lwan](https://github.com/lpereira/lwan)提供的配置文件，并没有覆盖全部的配置，下面从代码中提取的配置文件，供参考下(见国不少开源项目采用这种配置格式，但貌似没有具体定义这是那种格式，所有自己叫这种格式为`tson`，并写了一个简单解析的代码，参考[tson](https://github.com/buf1024/reinvent-wheel/blob/master/common/tson.c))：

```
#true on yes    以及非0数值，被解析为true
#false off no   以及非0数值，被解析为false

# Timeout in seconds to keep a connection alive.
keep_alive_timeout = 15

# Set to true to not print any debugging messages. (Only effective in
# release builds.)
quiet = false

# Set SO_REUSEPORT=1 in the master socket.
reuse_port = true

# Disable HAProxy's PROXY protocol by default. Only enable if needed.
proxy_protocol = false

allow_cors=false

# Value of "Expires" header. Default is 1 month and 1 week.
expires = 1M 1w

#error_template = error.html

# post 数据大小，最多为128m
max_post_data_size=40960000

allow_temp_files=true

# Number of I/O threads. Default (0) is number of online CPUs.
threads = 0

#需要root用户，或者有root权限的用户执行， 该section必须在所以listener之前
#straitjacket {
#  # 切换运行改程序的用户
#  user = heidong
#  # 程序运行的根目录
#  chroot = /home/heidong
#}

# section解析  ip:port
listener *:8080 {
    # 配置三种方式 1. prefix 路径 1. &handler 路径 3. module 路径。 只有配置为module的，方有回调函数读里面配置项目
    #prefix / {
    #        #handler和module只能是一个, module 为函数名为 lwan_module_{module_name}的模块
    #        module = module_name
    #        #handler 函数原型 lwan_http_status_t (*handler)(lwan_request_t *request, lwan_response_t *response, void *data);
    #        handler  = handler_symbol
    #        authorization basic {     #目前只支持basic
    #            realm=value #默认Lwan
    #            passowrd_file=file #默认htpasswd
    #        }
    #}
    # 配置handler
    #&handler / {
    #}
    serve_files / {
            path = ./

            # When requesting for file.ext, look for a smaller/newer file.ext.gz,
            # and serve that instead if `Accept-Encoding: gzip` is in the
            # request headers.
            serve precompressed files = true
            

            # 配置该section后，必须授权才可访问该url
            authorization basic {     #目前只支持basic
                realm=测试测试 #默认Lwan
                password_file=/home/heidong/privt/proj/lwan/htpasswd.conf #默认htpasswd，数据格式为：用户名=密码
            }
    }
}


```

### 其他

这里只是简单过一些lwan最基本的东西，没有详细的过里面的其他方面，仔细分析的话还有很多东西可以学，这里不再罗列了，暂时先这样。
