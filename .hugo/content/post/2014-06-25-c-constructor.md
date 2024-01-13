---
title: 'c构造函数'
date: 2014-06-25 13:00:43
aliases: [/2014/06/25/constructor/]
categories: [C] 
tags: [C,VC] 
---
### 构造函数

任何一们面向对象语言里都会涉及构造函数这一概念，只是实现的方式各有差异。需要这main函数之前执行一段代码是非常容易的事情，只需要声明一对象的全局变量，在构造函数可以为*所欲为*干你想干的事情。然而，对于面向过程的语言比如C，需要实现全局的构造函数就比较奇葩。当然GCC会有很优雅的解决方式，VC则猥琐点。

### 为何需要这个

其实在main函数里面调用一下就可以了，是的，这样是可以，但是对于框架的实现来说，这却是不太好的，客户使用起来不自由。当然也可以说，我是屌丝，我可以装逼点。

### 如何实现

* GCC实现  
  GCC 实现是非常简单的事情。在全局构造函数前叫以下编译器属性即可：  
    __attribute__((constructor))

* vc实现
  vc的实现比较奇葩，VC本身没有类似·__attribute__·这样的属性，你需要将全局函数编译到某个特定的代码段里面。MSDN对于这部分有详细的说明：[CRT Initialization](http://msdn.microsoft.com/en-us/library/bb918180.aspx)  
  简单来说就是将你的全局构造函数的函数指针编译到·\.CRT$XCU·段里面。如何编译到·\.CRT$XCU·段？VC有VC的语法。  
  
    // 声明在段·\.CRT$XCU·里面生成代码
    #pragma section("\.CRT$XCU",read)
    // 声明需要调用的函数
    __declspec(allocate("\.CRT$XCU")) void (\_\_cdecl *a)(void) = func;
    // 调用的函数实现
    void func(void){}

以上实现可以用下面实现：  

    #ifdef _MSC_VER    
    #define __CCALL __cdecl    
    #define __func__ __FUNCTION__    
    #define snprintf _snprintf    
    #pragma warning(disable:4996) // this is very violent    
    #pragma section(".CRT$XCU",read)
    #define __CONSTRCT(a, b)                                             \    
      void __CCALL __##a##__##b##__ ## 520hjm(void);                   \    
      __declspec(allocate(".CRT$XCU"))                                 \    
      void (__CCALL * __ctor__##a##__##b##__ ## 520hjm)(void) =        \    
      __##a##__##b##__ ## 520hjm;    
    #else    
    #define __CCALL    
    #define __CONSTRCT(a, b) __attribute__((constructor))    

### 额外参考

仿照[gtest](https://code.google.com/p/googletest/)的简易单元测试[ctest](https://github.com/buf1024/ctest)就是用的这方面的知识。  
