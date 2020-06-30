---
title: sys/queue.h
date: 2012-05-02 21:08:14
aliases: [/2012/02/02/sys-queue-h/]
categories: [linux]
tags: [c, linux]
---

### 概述
sys/queue.h是LINUX/UNIX系统下面的一个标准头文件，用一系列的数据结构定义了一队列。包括singly-lined list, list, simple queue(Singly-linked Tail queue), tail queue, circle queue五种。  

引用此头文件对这五种数据结构的描述：  

>A singly-linked list is headed by a single forward pointer. The 
elements are singly linked for minimum space and pointer manipulation 
overhead at the expense of O(n) removal for arbitrary elements. New 
elements can be added to the list after an existing element or at the 
head of the list.  Elements being removed from the head of the list 
should use the explicit macro for this purpose for optimum 
efficiency. A singly-linked list may only be traversed in the forward 
direction.  Singly-linked lists are ideal for applications with large 
datasets and few or no removals or for implementing a LIFO queue. 
>
A list is headed by a single forward pointer (or an array of forward 
pointers for a hash table header). The elements are doubly linked 
so that an arbitrary element can be removed without a need to 
traverse the list. New elements can be added to the list before 
or after an existing element or at the head of the list. A list 
may only be traversed in the forward direction. 
>
A simple queue is headed by a pair of pointers, one the head of the 
list and the other to the tail of the list. The elements are singly 
linked to save space, so elements can only be removed from the 
head of the list. New elements can be added to the list after 
an existing element, at the head of the list, or at the end of the 
list. A simple queue may only be traversed in the forward direction. 
>
A tail queue is headed by a pair of pointers, one to the head of the 
list and the other to the tail of the list. The elements are doubly 
linked so that an arbitrary element can be removed without a need to 
traverse the list. New elements can be added to the list before or 
after an existing element, at the head of the list, or at the end of 
the list. A tail queue may be traversed in either direction. 
>
A circle queue is headed by a pair of pointers, one to the head of the 
list and the other to the tail of the list. The elements are doubly 
linked so that an arbitrary element can be removed without a need to 
traverse the list. New elements can be added to the list before or after 
an existing element, at the head of the list, or at the end of the list. 
A circle queue may be traversed in either direction, but has a more 
complex end of list detection.

简单来说，即是单链表，双链表，单链队列，双向队列（尾队列）和双向循环队列。  

虽然这是LINUX/UNIX里面的文件，但此文件本身没有用到LINUX/UNIX的系统特性，因而可以跨平台使用。  

下面对各数据结构简单描述之。  

### 单链表(singly-linked list)
singly-linked list就是一单链表。  

singly-linked list相关的定义：  

|宏定义|	说明|  
|-----|:----|  
|SLIST_HEAD(name, type)|	定义表头结点。 <br>name: 表头结点名。<br>type: 结点类型。|  
|SLIST_HEAD_INITIALIZER(head)|	初始化头结点。 <br>head: 表头结点。|  
|SLIST_ENTRY(type)|	定义链表的链域。<br>type: 结点类型。|  

singly-linked list函数：  

|宏定义|	说明|  
|-----|:----|  
|SLIST_INIT(head)|初始化头结点。 <br>head: 表头结点。|  
|SLIST_INSERT_AFTER(slistelm, elm, field)|将结点elm插入到结点slistelm后面。 <br>slistelm：链表中某结点。 <br>elm:要插入的结点。 <br>field:链表中链域的名称。|  
|SLIST_INSERT_HEAD(head, elm, field)|将结点elm插入到头结点head后面。 <br>head: 表头结点。<br>elm:要插入的结点。<br>field:链表中链域的名称。|  
|SLIST_REMOVE_HEAD(head, field)	|	移除将表头结点下面一个结点。 <br>head: 表头结点。 <br>field:链表中链域的名称。|  
|SLIST_REMOVE(head, elm, type, field)|移除将elm结点，elm结点一定要是链表中一结点。 <br>head: 表头结点。 <br>elm:某结点。 <br>type: 结点类型。 <br>field:链表中链域的名称。|  
|SLIST_FOREACH(var, head, field)|遍历链表，相当于for循环。 <br>var: 结点类型的变量名称。 <br>head: 表头结点。 <br>field:链表中链域的名称。|  

singly-linked list 访问方法：  

|宏定义|	说明|  
|-----|:----|  
|SLIST_EMPTY(head)|	判断链表是否为空。 <br>head: 表头结点。|  
|SLIST_FIRST(head)|	访问链表里的第一个元素。 <br>head: 表头结点。|  
|SLIST_NEXT(elm, field)|访问elm结点后一个元素。 <br>elm:某结点。 <br>field:链表中链域的名称。|  

简单例子：  

        struct SListItem
        {
            int data;
            SLIST_ENTRY(SListItem) entry;
        };
        /*
         struct SListItem
         {
            int data;
            struct {
                struct SListItem* sle_next;
            } entry;
         }
         */
        void slist_demo()
        {
            struct SListItem* item = NULL;
            SLIST_HEAD(SListHead, SListItem) shead;
            /*
             struct SListHead {
                 struct SListItem* slh_first;
             } shead;
             */
            SLIST_INIT(&shead);
        
            item = (struct SListItem*)malloc(sizeof(struct SListItem));
            item->data = 1;
        
            SLIST_INSERT_HEAD(&shead, item, entry);
            /*
             item->entry.sle_next = (&shead)->slh_first;
             (&shead)->slh_first = item;
             */
        
            item = (struct SListItem*)malloc(sizeof(struct SListItem));
            item->data = 2;
        
            SLIST_INSERT_HEAD(&shead, item, entry);
            /*
             item->entry.sle_next = (&shead)->slh_first;
             (&shead)->slh_first = item;
             */
        
            SLIST_FOREACH(item, &shead, entry){
                printf("%d ", item->data);
            }
            /*
             for(item = (&shead)->slh_first; item; item = item->entry.sle_next){
                ...
             }
             */
            printf("\n");
        
            while(!SLIST_EMPTY(&shead)){
                item = SLIST_FIRST(&shead);
                printf("remove %d\n", item->data);
                SLIST_REMOVE(&shead, item, SListItem, entry);
                free(item);
            }
            /*
             while(!((&shead)->slh_first == NULL)){
                 item = (&shead)->slh_first;
                 ...
                 (&shead)->slh_first = (&shead)->slh_first->entry.sle_next;
                 ...
             }
             */
        }
        /*结果
        2 1
        remove 2
        remove 1
        */
        
### 双向链表(list)  

list就是双向链表，不过链域有点古怪，指向前一个结点是指针的指针。  
list 相关定义  

|宏定义|	说明|  
|-----|:----|  
|LIST_HEAD(name, type)|定义表头结点。 <br>name: 表头结点名。 <br>type: 结点类型。|  
|LIST_HEAD_INITIALIZER(head)|	初始化头结点。 <br>head: 表头结点。|  
|LIST_ENTRY(type)|定义链表的链域。 <br>type: 结点类型。| 

list函数  

|宏定义|	说明|  
|-----|:----|  
|LIST_INIT(head)|初始化头结点。 <br>head: 表头结点|  
|LIST_INSERT_AFTER(listelm, elm, field)|将结点elm插入到结点listelm后面。 <br>listelm：链表中某结点。 <br>elm:要插入的结点。 <br>field:链表中链域的名称。|  
|LIST_INSERT_BEFORE(listelm, elm, field)|将结点elm插入到结点listelm前面。 <br>listelm：链表中某结点。 <br>elm:要插入的结点。 <br>field:链表中链域的名称。| 
|LIST_INSERT_HEAD(head, elm, field)|将结点elm插入到头结点head后面。 <br>head: 表头结点。 <br>elm:要插入的结点。 <br>field:链表中链域的名称。|  
|LIST_REMOVE(elm, field)|移除将elm结点。 <br>elm:某结点。 <br>field:链表中链域的名称。|  
|LIST_FOREACH(var, head, field)|遍历链表，相当于for循环。 <br>var: 结点类型的变量名称。 <br>head: 表头结点。 <br>field:链表中链域的名称。| 

 list访问方法  

|宏定义|	说明|  
|-----|:----|  
|LIST_EMPTY(head)|判断链表是否为空。 <br>head: 表头结点。|  
|LIST_FIRST(head)|访问链表里的第一个元素。 <br>head: 表头结点。|  
|LIST_NEXT(elm, field)|访问elm结点后一个元素。 <br>elm:某结点。 <br>field:链表中链域的名称。| 

***注意***，因为list是双向链表，但在访问方法里没有写出访问前一个元素的宏。因而可以这样写一个，参数含义和LIST_NEXT一样：  

       #define LIST_PRE(elm, field) \
       (((elm)->field.le_pre) != &elm ? *((elm)->field.le_pre) : NULL)  

简单例子:  

       struct ListItem
       {
           int data;
           LIST_ENTRY(ListItem) entry;
       };
       /*
       struct ListItem
       {
           int data;
           struct{
               struct ListItem* le_next;
               struct ListItem** le_prev;
           } entry;
       };
       */
       void list_demo()
       {
           struct ListItem* item = NULL;
           
           LIST_HEAD(ListHead, ListItem) lhead;
           /*
           struct ListHead {
               struct ListItem* lh_first;
           } lhead;
           */
           LIST_INIT(&lhead);
           /*
           do{
                (&lhead)->lh_first = NULL;
           }while(0);
           */
       
           item = (struct ListItem*)malloc(sizeof(struct ListItem));
           item->data = 1;
       
           LIST_INSERT_HEAD(&lhead, item, entry);
           
           item = (struct ListItem*)malloc(sizeof(struct ListItem));
           item->data = 2;
           
           LIST_INSERT_HEAD(&lhead, item, entry);
           /*
           do{
               if(((item)->entry.le_next = (&lhead)->lh_first) != NULL)
                   (&lhead)->lh_first->entry.le_pre = &(elm)->entry.le_next;
               (&lhead)->lh_first = (item);
               (item)->entry.le_prev = &(&lhead)->lh_first;
           }while(0);
           */
           LIST_FOREACH(item, &lhead, entry){
               printf("%d ", item->data);
           }
           /*
           for ((item) = ((&lhead)->lh_first);
               (item);
               (item) = ((item)->entry.le_next)){
               ...
           }    
           */
           printf("\n");
       
           while(!LIST_EMPTY(&lhead)){
               item = LIST_FIRST(&lhead);
               printf("remove %d\n", item->data);
               LIST_REMOVE(item, entry);
               free(item);
           }
           /*
           while(!((&lhead)->lh_first == NULL)){
               item = ((&lhead)->lh_first);
               ...
               do{
                 if ((item)->entry.le_next != NULL)                \
                   (item)->entry.le_next->entry.le_prev =             \
                       (item)->entry.le_prev;                \
                 *(item)->entry.le_prev = (item)->entry.le_next;            \
               } while (0);
               ...
           }
           */
       }
       /*
       结果
       2 1
       remove 2
       remove 1
       */

### 简单队列(simple queue)

简单来说，就是表对有两个链域，分别指向头和尾。  
simple queue 定义（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|SIMPLEQ_HEAD(name, type)||  
|SIMPLEQ_HEAD_INITIALIZER(head)||  
|SIMPLEQ_ENTRY(type)|<br>| 

simple queue函数（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|SIMPLEQ_INIT(head)||  
|SIMPLEQ_INSERT_HEAD(head, elm, field)||  
|SIMPLEQ_INSERT_TAIL(head, elm, field)|| 
|SIMPLEQ_INSERT_AFTER(head, listelm, elm, field)||  
|SIMPLEQ_REMOVE_HEAD(head, field)||  
|SIMPLEQ_REMOVE(head, elm, type, field)|| 
|SIMPLEQ_FOREACH(var, head, field)| <br>| 

simple queue方法（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|SIMPLEQ_EMPTY(head)||  
|SIMPLEQ_FIRST(head)||  
|SIMPLEQ_NEXT(elm, field)| <br>| 

简单例子：  
用法与list用法类似，不再重复。  

### 单链尾队列(singled-linked tail queue)  

这个和Simple queue是一样的，参考simple queue  
singled-linked tail queue定义(具体说明不再写，可以参考list的，或者就直接展开宏)  

|宏定义|	说明|  
|-----|:----|  
|STAILQ_HEAD(name, type)||  
|STAILQ_HEAD_INITIALIZER(head)||  
|STAILQ_ENTRY(type)| <br>| 

tail queue 函数（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|STAILQ_INIT(head)||  
|STAILQ_INSERT_HEAD(head, elm, field)||  
|STAILQ_INSERT_TAIL(head, elm, field)	|| 
|STAILQ_INSERT_AFTER(head, listelm, elm, field)	||  
|STAILQ_REMOVE_HEAD(head, field)	||  
|STAILQ_REMOVE(head, elm, type, field)	|| 
|STAILQ_FOREACH(var, head, field)	| <br>| 

tail queue方法（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|STAILQ_EMPTY(head)	||  
|STAILQ_FIRST(head)||  
|STAILQ_NEXT(elm, field)| <br>|

简单例子：  
用法与list用法类似，不再重复。  

### 循环队列(circle queue)  

循环队列。  
circle queue定义（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|CIRCLEQ_HEAD(name, type)	||  
|CIRCLEQ_HEAD_INITIALIZER(head)	||  
|CIRCLEQ_ENTRY(type)| <br>|

circle queue函数（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|CIRCLEQ_INIT(head)||  
|CIRCLEQ_INSERT_AFTER(head, listelm, elm, field) ||  
|CIRCLEQ_INSERT_BEFORE(head, listelm, elm, field)||
|CIRCLEQ_INSERT_HEAD(head, elm, field) ||  
|CIRCLEQ_INSERT_TAIL(head, elm, field)|| 
|CIRCLEQ_REMOVE(head, elm, field) || 
|CIRCLEQ_REPLACE(head, elm, elm2, field)| <br>|

circle queue访问方法（具体说明不再写，可以参考list的，或者就直接展开宏）  

|宏定义|	说明|  
|-----|:----|  
|CIRCLEQ_FIRST(head)||  
|CIRCLEQ_LAST(head)||  
|CIRCLEQ_END(head)||
|CIRCLEQ_NEXT(elm, field)	||  
|CIRCLEQ_PREV(elm, field)||  
|CIRCLEQ_EMPTY(head)||
|CIRCLEQ_FOREACH(var, head, field)	||  
|CIRCLEQ_FOREACH_REVERSE(var, head, field)| <br>|  


简单例子：  
用法与list用法类似，不再重复。 

## 小结
&emsp虽然这是linux/unix实现的经过长时间考验的成熟的数据结构，但是如果不是很熟悉的话，第一次用起来还是感觉挺不习惯的。但是好在各个数据结构的定义和方法都非常类似，接口比较统一，如果用多的了，熟悉了，感觉就不错了。  

