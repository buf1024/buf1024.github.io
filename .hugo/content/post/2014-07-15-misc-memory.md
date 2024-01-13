---
title: '杂乱记忆'
date: 2014-07-15 11:10:13
aliases: [/2014/07/15/Misc-memory/]
tags: [C,Python,Linux]
---

这里记录的是一些常见的错误或技巧或一些注意事项等，不定时更新:  

- [hexo](http://hexo.io) 安装（含插件）  
 hexo 是相对比较好的一个静态BLOG生成器，虽然我在[树莓派](http://www.raspberrypi.org)上面编译一个nodejs耗时6个多小时，每次hexo -g耗时接近15分钟，但这不影响使用。安装：  
  
        npm install -g hexo    
        npm install hexo-generator-feed    
        npm install hexo-generator-sitemap    

- Python 后台操作git  
  程序切换的git目录后:  

        git_cmd = ["git", "add", "./*"]
        ret = subprocess.call(git_cmd)
        
        git_cmd = ["git", "commit", "-m", "'auto commit'", "-a"]
        ret = subprocess.call(git_cmd)
        
        git_cmd = ["git", "push"]
        ret = subprocess.call(git_cmd)

 注意，因为提交了后台，使用的不是git clone仓库的那个用户，需要在为具体的git目录添加git信息  

        git config user.email "xxx@xx.xx"
        git config user.name  "xxx"
        git config push.default simple

 同时需要在运行这个用户的主目录下添加.ssh信息。否则提交不成功  

- 路由器登陆  
  TP-link路由器的登陆使用的是http basic auth。具体是每一次请求，都必须在http头输入http认证信息。http认证方式可参考：[HTTP认证方式](http://blog.csdn.net/hotnet522/article/details/5824716)。Python 代码:  

        req = urllib2.Request(url)
        req.add_header("Authorization", "Basic aGVsbG86d29ybGQ=")
        rsp = urllib2.urlopen(req)
        rsp.close()

- [mysch](https://github.com/buf1024/mysch) 程序需全路径  
  mysch是调度监控程序，目前没有流量控制，只有时间控制，调用的程序名需要全路径。  

- [openerdns](https://code.google.com/p/openerdns/)和[Opendns](http://www.opendns.com)  

        OpenerDNS可实现最简单的翻墙，有时不可用。
        地址: 42.120.21.30  
        OpenDNS不能翻墙。
        地址：  
        208.67.222.222  
        208.67.220.220  
        208.67.222.220  
        208.67.220.222  
