---
title: 'Linux /proc 伪文件系统'
date: 2014-07-28 14:57:18
aliases: [/2014/07/23/process-infORMation-pseduo-file-system/]
categories: [Linux]
tags: [Process, Linux]
---
Linux /proc伪文件系统是内核数据结构的接口，是异常强大的，大部分的文件都是只读的。本文主要是根据man proc来补充说明(**未完待续**)。  

- */proc/[pid]*  
 每一个正在运行的进程都对应有这样一个伪目录，目录名为该进程的pid。每个伪目录下面都有以下伪文件或伪目录:  

- /proc/[pid]/auxv  
  包含ELF解析信息（ELF就是Linux系统二进制可执行文件的文件格式），`auxv` 是 AUXiliary Vector简写。每个条目的格式就是:一个`unsigned long`ID加上一个`unsigned long`数值。这个AUXV是什么意思呢？ 我们可以通过加上`LD_SHOW_AUXV=1 程序名`可以获取具体的数值:  
  
           [heidong@HEIDONGVM 2218]$ LD_SHOW_AUXV=1 /bin/sh         
           AT_SYSINFO_EHDR: 0x7fff173d8000        
           AT_HWCAP:        178bf3ff         
           AT_PAGESZ:       4096         
           AT_CLKTCK:       100         
           AT_PHDR:         0x400040         
           AT_PHENT:        56         
           AT_PHNUM:        9         
           AT_BASE:         0x7fd32ed59000         
           AT_FLAGS:        0x0         
           AT_ENTRY:        0x41aef0         
           AT_UID:          500         
           AT_EUID:         500         
           AT_GID:          500         
           AT_EGID:         500         
           AT_SECURE:       0         
           AT_RANDOM:       0x7fff17230579         
           AT_EXECFN:       /bin/sh         
           AT_PLATFORM:     x86_64         
           sh-4.1$          

  对应的`AT_*`的含义可在`/usr/include/elf.h`找到。  
  相关参考: [What does /proc/PID/auxv mean exactly?](http://www.Linuxquestions.org/questions/Linux-kernel-70/what-does-proc-pid-auxv-mean-exactly-4175421876/) 和 [关于ELF的辅助向量](http://lenky.info/archives/2013/02/2203)。  

- */proc/[pid]/cmdline*  
   包含非僵死进程的命令行启动参数，该文件包含`"\0"`。如:  
  
           [heidong@HEIDONGVM 2238]$ cat cmdline         
           bash[heidong@HEIDONGVM 2238]$          
- */proc/[pid]/coredump_filter*  
   用于控制哪些内存段写入到core文件，16进制显示，是与`mmap`映射类型的位掩码。默认0,1,4,5(如果内核编译时定义CONFIG_CORE_DUMP_DEFAULT_ELF_HEADERS)，所以默认值是33。掩码含义是：  
   bit 0 dump匿名私有映射.  
   bit 1 dump匿名共享映射.  
   bit 2 dump基于文件的私有映射.  
   bit 3 dump基于文件的共享映射.  
   bit 4(Linux 2.6.24) dump elf 头.  
   bit 5(Linux 2.6.28) dump私有页面(private huge pages).
   bit 6(Linux 2.6.28) dump共享页面(shared huge pages).
- /proc/[pid]/cpuset
   // todo  
- /proc/[pid]/cwd  
   进程当前工作目录的软连接。对于多线程的程序，如果主线程退出或挂掉了，子线程还存在，这个连接时无效的。去到当前工作目录：  

           cd /proc/20/cwd; /bin/pwd          
- */proc/[pid]/environ*  
   进程环境变量，每个条目都是以`"\0"`结束。可用这种方法显示:  

           [heidong@HEIDONGVM 2238]$ (cat /proc/2238/environ; echo) | tr '\000' '\n'        
- /proc/[pid]/exe  
   进程可执行文件在软连接。对于多线程的程序，如果主线程退出或挂掉了，子线程还存在，这个连接时无效的。  
- */proc/[pid]/fd/*  
   打开文件文件描述符目录在连接。对于多线程的程序，如果主线程退出或挂掉了，子线程还存在，这个连接时无效的。`/proc/self/fd/N`和`/dev/fd/N`是一样在。补充说明，对于只知道fd怎么找对于的文件呢？答案是很简单的，直接用`readlink`函数，查看对于该目录下文件的连接就可以了。  
- /proc/[pid]/fdinfo/  
   这个目录是和/proc/[pid]/fd/对应的，就是对于没个文件的当前偏移和打开标示。如:  

           [heidong@HEIDONGVM fdinfo]$ cat 165        
           pos: 0        
           flags: 0100002        
           [heidong@HEIDONGVM fdinfo]$        
- /proc/[pid]/limits  
   limits信息。和`getrlimit`获取的信息是一样的。如:  

           [heidong@HEIDONGVM 3500]$ cat limits         
           Limit                     Soft Limit           Hard Limit           Units     
           Max cpu time              unlimited            unlimited            seconds   
           Max file size             unlimited            unlimited            bytes     
           Max data size             unlimited            unlimited            bytes     
           Max stack size            10485760             unlimited            bytes     
           Max core file size        unlimited            unlimited            bytes     
           Max resident set          unlimited            unlimited            bytes     
           Max processes             1024                 14864                processes 
           Max open files            4096                 4096                 files     
           Max locked memory         65536                65536                bytes     
           Max address space         unlimited            unlimited            bytes     
           Max file locks            unlimited            unlimited            locks     
           Max pending signals       14864                14864                signals   
           Max msgqueue size         819200               819200               bytes     
           Max nice priority         0                    0                    
           Max realtime priority     0                    0                    
           Max realtime timeout      unlimited            unlimited            us        
- /proc/[pid]/maps  
   内存映射的地址和权限。如:  

           [heidong@HEIDONGVM 3890]$ cat maps 
           00400000-00410000 r-xp 00000000 fd:00 1072484                            /usr/bin/groff
           0060f000-00610000 rw-p 0000f000 fd:00 1072484                            /usr/bin/groff
           00610000-00614000 rw-p 00000000 00:00 0 
           0080f000-00811000 rw-p 0000f000 fd:00 1072484                            /usr/bin/groff
           01c6f000-01c90000 rw-p 00000000 00:00 0                                  [heap]
- /proc/[pid]/mem  
   进程对应的内存，可以通过`open`, `read`, `lseek`访问。
- /proc/apm  

- /proc/bus  

- /proc/cmdline  

- /proc/config.gz  

- /proc/cpuinfo  

- /proc/devices  

- /proc/diskstats  

- /proc/dma  

- /proc/driver  

- /proc/execdomains  

- /proc/fb  

- /proc/filesystems  

- /proc/fs  

- /proc/ide  

- /proc/interrupts  

- /proc/iomem  

- /proc/ioports  

- /proc/kallsyms  

- /proc/kcore  

- /proc/kmsg  

- /proc/ksyms

- /proc/loadavg  

- /proc/locks  

- /proc/malloc  

- /proc/meminfo  

- /proc/modules  

- /proc/mounts  

- /proc/mtrr  

- /proc/net  

- /proc/partitions  

- /proc/pci  

- /proc/scsi  

- /proc/self  

- /proc/slabinfo  

- /proc/stat  

- /proc/swaps  

- /proc/sys  

- /proc/sysrq-trigger  

- /proc/sysvipc  

- /proc/tty  

- /proc/uptime  

- /proc/version  

- /proc/vmstat  

- /proc/zoneinfo  
