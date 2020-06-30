---
title: '隐藏make编译的编译输出'
date: 2014-07-04 16:45:00
aliases: [/2014/07/04/hide-compile-output/]
categories: [c]
tags: [makefile, linux, redis]
---
编译程序make时，会看到很多编译输出。对于开发来说，这是非常直观的，但是如果分发到客户（需要编译）或者只需要关注错误时，这种输出就感觉烦人。Redis是这么做的：  

    REDIS_CC=$(QUIET_CC)$(CC) $(FINAL_CFLAGS)
    REDIS_LD=$(QUIET_LINK)$(CC) $(FINAL_LDFLAGS)
    REDIS_INSTALL=$(QUIET_INSTALL)$(INSTALL)
    
    CCCOLOR="\033[34m"
    LINKCOLOR="\033[34;1m"
    SRCCOLOR="\033[33m"
    BINCOLOR="\033[37;1m"
    MAKECOLOR="\033[32;1m"
    ENDCOLOR="\033[0m"
    
    ifndef V
    QUIET_CC = @printf '    %b %b\n' $(CCCOLOR)CC$(ENDCOLOR) $(SRCCOLOR)$@$(ENDCOLOR) 1>&2;
    QUIET_LINK = @printf '    %b %b\n' $(LINKCOLOR)LINK$(ENDCOLOR) $(BINCOLOR)$@$(ENDCOLOR) 1>&2;
    QUIET_INSTALL = @printf '    %b %b\n' $(LINKCOLOR)INSTALL$(ENDCOLOR) $(BINCOLOR)$@$(ENDCOLOR) 1>&2;
    endif

在编译前加`QUIET`选项，正常的编译会忽略掉，出错或警告的信息会完整的输出来。学到东西了！！  


