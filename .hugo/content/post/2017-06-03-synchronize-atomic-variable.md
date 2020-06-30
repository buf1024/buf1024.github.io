---
title: "线程同步与原子变量"
date: 2017-06-03T11:19:25+08:00
draft: false
categories: [c] 
tags: [c, gcc, linux]
---

无论是基于多线程还是多进程并发的程序，同步控制总是不可避免的。按照自己的思维和使用习惯，自然会想到的是互斥量(mutext)和自旋锁(spin lock)，当然同步的方式不只这两种。gcc提供了，`__sync_*`系列的内置函数，用于对**基本数据类型**的原子操作。`__sync_*`针对的是c/c++的**基本数据类型**，而互斥量和自旋锁可以用于代码块的同步，当然，也不是说`__sync_*`不能用于代码块的同步控制，只需要一点点的技巧。

在撸`__sync_*`系列函数之前，先看看一丢丢并发程序易忽略的技巧。为了提高效率，我们希望线程的调度级别和系统普通进程的调度级别一样的，为了充分利用cpu的cache特性，我们还希望每次系统调度时都绑定到同一个cpu中执行。前者，已经使用过了：`pthread_attr_setscope(&attr, PTHREAD_SCOPE_SYSTEM)`，后者，叫cpu亲和性（也有叫亲缘性，英文：affinity），用到的api主要有：`sched_setaffinity` 和 `sched_getaffinity`，使用方式也是挺简单的，如：  
```C
    // 线程/进程起来后
    CPU_ZERO(&set);
    CPU_SET(proc_num, &set);

    if (sched_setaffinity(gettid(), sizeof( cpu_set_t ), &set)) {
        // ...
    }
```
我们再看看习惯思维的同步工具，互斥锁（mutext）和自旋锁(spin lock)的区别。互斥锁是sleep-waiting类型的锁，例如，在多核的环境中，A线程如果尝试对B线程进行加互斥锁，如果互斥锁已经被B线程占用，那么A线程就会进入等等状态，那么系统就会对A线程进行上下文切换(conext switch)，让出CPU使用权，上下文切换(conext switch)是比较耗时的操作的。而自旋锁的处理方式则不会进行上下文切换，而是让cpu不断的轮询的获取锁。自旋锁的CPU轮询和互斥锁的上下文切换同样是耗性能的，大多数情况下，有限时间内的CPU轮询比互斥锁的上下文切换轻量太多了。那么什么情况下使用互斥锁（mutext）和自旋锁(spin lock)呢，有人总结说，尽管互斥锁消耗的资源相对多（上下文切换），但互斥锁是适用情况还是比较广，大多数情况下还是选择互斥锁，在需要进行性能优化/或者知道锁代码块运行时间的情况下，才考虑自旋锁。

另外提及一下，线程局部变量。这个在其他语言是有这个概念的，C/C++也是有，但是经常容易忽略。线程局部变量，使用`__thread`修饰，使用时有局限性：  
- `__thread`可以修饰全局变量、函数的静态变量，但是无法修饰函数的局部变量。
- 被`__thread`修饰的变量只能在编译期初始化，且只能通过常量表达式来初始化

我们正式看一下gcc提供的原子操作： 
```C
type __sync_fetch_and_add (type *ptr, type value, ...)
type __sync_fetch_and_sub (type *ptr, type value, ...)
type __sync_fetch_and_or (type *ptr, type value, ...)
type __sync_fetch_and_and (type *ptr, type value, ...)
type __sync_fetch_and_xor (type *ptr, type value, ...)
type __sync_fetch_and_nand (type *ptr, type value, ...)

type __sync_add_and_fetch (type *ptr, type value, ...)
type __sync_sub_and_fetch (type *ptr, type value, ...)
type __sync_or_and_fetch (type *ptr, type value, ...)
type __sync_and_and_fetch (type *ptr, type value, ...)
type __sync_xor_and_fetch (type *ptr, type value, ...)
type __sync_nand_and_fetch (type *ptr, type value, ...)

bool __sync_bool_compare_and_swap (type *ptr, type oldval type newval, ...)
type __sync_val_compare_and_swap (type *ptr, type oldval type newval, ...)

__sync_synchronize (...)
type __sync_lock_test_and_set (type *ptr, type value, ...)
void __sync_lock_release (type *ptr, ...)
```

`__sync_*`系列函数操作的数据都是原始的数据类型，从名字看，很容易猜出它们的用途。最后三个使用的比较少，是内存壁垒的含义，现代CPU都是乱序执行的，有些情况下乱序执行会有问题，所以需要顺序执行，这就是这三个函数的用处。最开始说过，`__sync_*`用于代码块的同步控制，需要一丢丢的技巧。比如用于代码块的封装:
```C
#define trylock(lock)  (*(lock) == 0 && __sync_bool_compare_and_swap(lock, 0, 1))
#define unlock(lock)    *(lock) = 0

// 使用
if(!trylock(&lock)) {
    return;
}
...
unlock(&lock);
```
上面说自旋锁锁被占用时，空耗CPU，使用原子变量，可以进行一定程度的优化，和原来的自旋锁的优化是，自旋一定次数后，主动交出CPU的控制权：
```C
void spinlock(uint_t *lock, uint_t spin)
{
    ngx_uint_t  i, n;
    for ( ;; ) {
        if (*lock == 0 && __sync_bool_compare_and_swap(lock, 0, 1)) {
            return;
        }
        if (ncpu > 1) { // ncpu CPU个数
            for (n = 1; n < spin; n <<= 1) {
                for (i = 0; i < n; i++) {
                    // cpu_pause(); 空转
                }
                if (*lock == 0 && __sync_bool_compare_and_swap(lock, 0, 1)) {
                    return;
                }
            }
        }

        sched_yield(); // 只旋一个周期，没有获取到锁，退出CPU控制权
    }
}
```

正常的应用业务或许不会用到`__sync_*`系列函数，但如果使用了无疑可以进一步提高单个应用的性能。
