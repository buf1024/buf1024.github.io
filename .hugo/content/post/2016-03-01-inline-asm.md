---
title: '内联ASM'
date: 2016-03-01 14:13:05
categories: [asm]
tags: [Asm, gas]
---
一、最基本格式:

    `asm("assembly code")`

    注意：

    1. 汇编代码必须用双引号括起来

    2. 如果汇编代码有多行，下一行必须有制表符开始

    例子：

```c
    asm ( “movl $1, %eax\n\t”

              “movl $0, %ebx\n\t”

              “int $0x80”);
```


    使用全局变量，基本格式只能使用全局变量，无法使用局部变量

    例子：

```c
    #include

    int a = 10, b = 20;

    int rst = 0;

    int main()

    {

        asm("movl a, %eax\n\t"

                "movl b, %ebx\n\t"

                "imull %ebx, %eax\n\t"

                "movl %eax, rst\n\t");

        printf(": %d\n", rst);

        return 0;

    }
```

    ==

    编译器会对内联的ASM进行优化，为了避免编译器优化asm

    `asm volatile ("assembly code")`

    或

    `asm ("assembly code")`

    或

    `asm volatile ("assembly code")`

二、扩展格式

    `asm (“assembly code” : output locations : input operands : changed registers);`

    Assembly code   : 格式同基本格式

    Output locations: 输出位置，内存/寄存器，多个数据用逗号分隔

    Input operands  :  输入位置，内存/寄存器，多个数据用逗号分隔

   Changed registers: 变更的寄存器（注意，默认输入/输出已经包括的寄存器不能出现在此）

   如果没有输出/输入/变更寄存器，可以空着，但冒号还是要有的。

   另外，避免编译器优化的方法和上面做法一样。

   使用扩展格式的asm，引用寄存器时，需要%%，如：

```c
  asm (“imull %%edx, %%ecx\n\t”

           “movl %%ecx, %%eax”

              : “=a”(result)

              : “d”(data1), “c”(data2));
```

Output locations/ Input operands 格式：

   “constraint”(variable)

   constraint       含义

   a    使用 %eax, %ax, %al

   b    使用 %ebx, %bx, %bl

   c     使用 %ecx, %cx, %cl

   d    使用 %edx, %dx, %dl

   S     使用 %esi, %si

   D    使用 %edi, %di

    r     使用所有通用寄存器

   q     使用%eax, %ebx, %ecx, %edx

   A     使用 %eax, %edx 表示64位值

    f     使用浮点寄存器

    t     使用first（top）浮点寄存器

    u    使用second浮点寄存器

   m    使用变量内存地址

    o    使用偏移内存地址

    V    只使用直接内存地址

    i     使用立即数(整数)值

    n    使用已知立即数(整数)值

    g    使用内存或寄存器

     Output locations还可以加修饰符：

     修饰符  含义

     +     操作数可读可写

     =     操作数只可写

     %    操作数可以切换到下个操作数

     &    内联函数结束前，操作数可以删除和复用

     如：

     `asm (“assembly code” : “=a”(result) : “d”(data1), “c”(data2));`

     含义：

     将data1移到%edx，data2移到%ecx，结果移到%eax，然后移到result。

Output locations/ Input operands 中的寄存器，可以使用占位符引用，从$0开始，如：

```c
    asm (“assembly code”

                : “=r”(result)

                : “r”(data1), “r”(data2));
```

     $0, 表示包含result的寄存器，$1表示包含data1的寄存器，$2表示包含data2的寄存器。

     另外，在后面的变量中，也可以应用前面变量。如：

```c
    asm (“assembly code”

                : “=r”(data2)

                : “r”(data1), “0”(data2));
```

    Output locations/ Input operands 中的寄存器，也可以赋一个变量，格式：

    %[name]”constraint”(variable)

    例子：

```c
    asm (“imull %[value1], %[value2]”

              : [value2] “=r”(data2)

              : [value1] “r”(data1), “0”(data2));
```

    扩展asm格式里面可以包含跳转指令，需要加后缀指明跳转的方向，f表示向前跳转，b表示向后跳转。如

```c
   asm(“cmp %1, %2\n\t”

            “jge 0f\n\t”

            “movl %1, %0\n\t”

            “jmp 1f\n “

            “0:\n\t”

            “movl %2, %0\n “

            “1:”

                :”=r”(result)

                :”r”(a), “r”(b));
```