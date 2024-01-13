---
title: coroutine协程
date: 2014-08-21 16:46:46
aliases: [/2014/08/21/coroutine/]
categories: [C]
tags: [Coroutine]
---

如果你接触过lua这种小巧的脚本语言，你就会经常接触到一个叫做`协程`的神奇概念。大多数脚本语言都有对`协程`不同程度的支持。但是大多编译语言，如C/C++，根本就不知道这样的东西存在。当然也很多人研究如何在编译语言实现`协程`的实现，轮子一个又一个的被发明。[酷壳](http://coolshell.cn/) 这篇文章[《一个“蝇量级” C 语言协程库》](http://coolshell.cn/articles/10975.html)说的很详细，但对于文中介绍的协程库[protothread](http://dunkels.com/adam/pt/)，很难看的懂。[云风大哥](http://blog.codingnow.com/)在搜索无满意结果后也重新发明轮子，实现自己版本的一个协程库[《C 的 coroutine 库》](http://blog.codingnow.com/2012/07/c_coroutine.html), 云风版本的`coroutine`是常规的根据`getcontext/swapcontext`的非常传统的方法，没有用到其他什么奇淫技巧。同时他接口几乎和lua`协程`的接口一样，比较容易看的懂，但是貌似和CPU构架相关，下面详细说。这篇文章就是主要写看云风的代码的，代码注释放到github上面[coroutine](https://github.com/buf1024/coroutine)，当然因为代码本身很精练和简单，注释也就非常少的。关于`协程`，可参考百度百科：[`协程`](http://baike.baidu.com/view/2665148.htm?fr=aladdin)。  

开始之前，看看unix/Linux下面的`context系列函数`（[云风大哥](http://blog.codingnow.com/)说，windows下面可以用`纤程`实现，以前看《windows核心编程》的时候，了解过这个概念，但是实际上编程运用的只限于进程/线程，没有用到过`纤程`这高级货，现在接触`windows api`也相对少，所以这个就不详细展开）。`context系列函数`大约就以下函数：

        #include <ucontext.h>        

        int getcontext(ucontext_t *ucp);        
        int setcontext(const ucontext_t *ucp);        

        void makecontext(ucontext_t *ucp, void (*func)(), int argC, ...);        
        int swapcontext(ucontext_t *oucp, ucontext_t *ucp);        

根据Linux手册页，`getcontext`返回的是当前执行进程的上下文信息，包括信号掩码，执行栈，寄存器等信息。比较值得注意的是`makecontext`，该函数除了传递`ucp`外，还传递一个通用的函数指针`func`和一个`argc`，传递给`func`的函数原型并不一定是`void(*)()`类型的，参数的个数由`argc`确定。当调用`makecontext`成功后，如果对这个上下文进行切换(`swapcontext`)或用这个上下文设置(`setcontext`)，内核就会调用该`func`函数。`协程`的实现就是用到这个重要的原理。Linux手册页有个详细的例子介绍`makecontext`怎么用的，请参考之。  

我们来看[云风大哥](http://blog.codingnow.com/)的协程库:  

        // 协程调度大小        
        struct schedule {        
         char stack[STACK_SIZE]; // 当前运行的协程的栈
         ucontext_t main; // 下个协程切换的上下文状态
         int nco; // 当前协程
         int cap; // 容量
         int running; // 当前运行的协程
         struct coroutine **co; // 协程数组
        };

        struct coroutine {
         coroutine_func func; // 调用函数
         void *ud;      // 用户数据
         ucontext_t ctx; // 保存的协程上下文状态
         struct schedule * sch; // 保存struct schedule指针
         ptrdiff_t cap; // 上下文切换时保存的栈的容量
         ptrdiff_t size; // 上下文切换时保存的栈的大小 size <= cap
         int status; // 协程状态
         char *stack; // 保存的协程栈大小
        };  

定义的结构体够简洁的，除了`struct coroutine`里面的`cap`和`size`，感觉作用不大。上下文信息是在`coroutine_resume`里面设置的，这里将栈设置为自己定义的区域，大小也限定了，还设置了下个上下文切换的地址（在`swapcontext`里为他赋初始值）：  

        getcontext(&C->ctx);
  C->ctx.uc_stack.ss_sp = S->stack; // 设置栈
  C->ctx.uc_stack.ss_size = STACK_SIZE; // 设置栈大小
  C->ctx.uc_link = &S->main; // 下个切换的上下文状态，由swapcontext设置其值
  S->running = id;
  C->status = COROUTINE_RUNNING;
  uintptr_t ptr = (uintptr_t)S;
  makecontext(&C->ctx, (void (*)(void)) mainfunC, 2, (uint32_t)ptr, (uint32_t)(ptr>>32));
  swapcontext(&S->main, &C->ctx);

[云风大哥](http://blog.codingnow.com/)的协程库有点不好的地方就是这个协程库或许是与CPU的构架相关的，做成与CPU无关应该不难。看保存上下文信息的代码：  

        static void
        _save_stack(struct coroutine *C, char *top) {
         char dummy = 0;
          //这里做了一个特定的假设(事实上大部分CPU也是如此)
         assert(top - &dummy <= STACK_SIZE); // stack大小
         if (C->cap < top - &dummy) {
          free(C->stack); // 首次C->stack为NULL,free(NULL)是OK的
          C->cap = top-&dummy;
          C->stack = malloc(C->cap);
         }
         C->size = top - &dummy;
         memcpy(C->stack, &dummy, C->size);
        }

传递的参数`top`指向的是栈顶，栈顶与新定义的变量相减`top - &dummy`得到栈的大小。这里做了一个特定的假设，栈由高地址向低地址方向增长(这是非常典型的程序布局方式，虽然我不知道那种CPU不是用这种布局，但是觉得肯定有有低地址向高地址增长的)，这种程序布局的CPU一般是X86构架的，所以这个库时与CPU结构相关的。  

总的来说，这个协程库写的非常精简。[protothread](http://dunkels.com/adam/pt/)这种高级货，暂时不看了。粗略的介绍完毕，如有错误，请批评指正。  

---------
2016-04-28 补充：  
"""根据Linux手册页，`getcontext`返回的是当前执行进程的上下文信息，包括信号掩码，执行栈，寄存器等信息。比较值得注意的是`makecontext`，该函数除了传递`ucp`外，还传递一个通用的函数指针`func`和一个`argc`，传递给`func`的函数原型并不一定是`void(*)()`类型的，参数的个数由`argc`确定。当调用`makecontext`成功后，如果对这个上下文进行切换(`swapcontext`)或用这个上下文设置(`setcontext`)，内核就会调用该`func`函数。`协程`的实现就是用到这个重要的原理。Linux手册页有个详细的例子介绍`makecontext`怎么用的，请参考之。"""  
上面一段说明说的比较笼统，再简单补充不使用这个库的例子：  
 main -> co_resume -> func -> co_yield -> main  
 调用co_resume 后，执行上下文切换，调用协程函数func，在协程函数func里面，调用co_yield, 执行上下文切换，返回main，下次co_resume时，func从co_yield后面继续执行(因为之前保存了上下文的切换信息)。

[例子](https://github.com/buf1024/coroutine)输出：  

        main        
        main fun: 0        
        test_func: 0        
        main fun: 1        
        test_func: 1        
        main fun: 2        
        test_func: 2        
        main fun: 3        
        test_func: 3        
        ^Csignal int        
