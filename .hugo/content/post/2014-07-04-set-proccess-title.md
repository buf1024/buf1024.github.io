---
title: '设置进程的名称'
date: 2014-07-04 10:27:47
aliases: [/2014/07/04/set-proccess-title/]
categories: [C]
tags: [C,Linux,redis]
---
### 如何开搞

我们平时ps进程时，显示的进程标题是我们输入的程序名以及一些参数。这种显示方式并非非常直观，我们需要一种方式可以设置程序的标题，一眼就知道该进程时什么鸟。如redis或nginx:  

    root      8527     1  0 10:30 ?        00:00:00 nginx: master process nginx    
    nginx     8528  8527  0 10:30 ?        00:00:00 nginx: worker process    

实现的原理非常简单，我们只需要修改argv[0],说指向的那段内存空间的内容即可。但是为了使程序的正常逻辑不受影响，事先要把原理的内容保存起来，并通过原来的方式可以访问。  
argv[0]所指向的的空间是和·enviorn·所指向的内存空间是连续的，在程序启动的时候，系统已经设置好这些空间了。如图所示：  

![](http://privt-share.qiniudn.com/@/redis/setproctitle.jpg)  

那么，1. 我们只需要重新分配空间（不一定是连续的）来存储这些内容，将地址分别复制给`argv`或`environ`数组，这样原来程序的流程不受影响。2. 我们可以在原`agv`最开始的地方和`environ`最结尾的地方，随便设置你显示的内容即可，也就是图中`./exe`开始的地方和`Env5=val5`最后一个0结尾复的地方。只需要注意到，设置的内容必须与0结尾，而且不能超过该内存空间。  

### redis如何实现

redis的实现在[setproctitle](https://github.com/antirez/redis/blob/2.8.12/src/setproctitle.c)里面。只是实现有点疑惑：  

1. argc个数少于argv数组数？  

     for (i = 0; i < argc || (i >= argc && argv[i]); i++) {
      if (!argv[i] || argv[i] < end)
       continue;

      end = argv[i] + strlen(argv[i]) + 1;
     }

 按照我们一直以来的常识，是通过argc从argv里面取出数据来的，而他这里`i < argc || (i >= argc && argv[i])`这个条件明显就是argc可能会少于argv个数。什么情况下会出现这种情况呢？  
2. SPT.nul貌似没有用  

     if (nul < SPT.nul) {
      *SPT.nul = '.';
     } else if (nul == SPT.nul && &nul[1] < SPT.end) {
      *SPT.nul = ' ';
      *++nul = '\0';
     }
 这是`setproctitle`里面的一个奇怪的逻辑，更本没想到他是干嘛用的。而且貌似SPT.nul在set proc title里面没有什么作用。  

### 简单吐槽

虽然redis非常出名，但是redis的代码貌似不是很规范……或者是我们一直接触的就不规范，哈……  
