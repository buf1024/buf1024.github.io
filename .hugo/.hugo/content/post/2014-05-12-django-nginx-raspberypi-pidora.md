---
title: 'django + nginx + raspberypi + pidaro'
date: 2014-05-12 22:12:47
aliases: [/2014/05/12/django-nginx-raspberypi-pidora/]
categories: [django]
tags: [js, css, django, python]
---

对于一个从事后台开发的人来说，搞了个网站，我自己都觉得有点意外。不是很懂html，不是很懂css，不是很懂js，不是很懂web开发模式/框架，不是很懂httpd/nginx……，web的东西样样都知道一点，但样样都不知道很多，web开发对于我来说很陌生。很早前，买了个树莓派，就一直丢在那里，24小时开着机，但也是仅仅开机。想着一直让它这么浪费电不是办法，于是就现在折腾些web的东西了。于是便在工作空闲的时候，花了两个多星期，从CSS开始学起，有了这个很一般，或许很多BUG的网站。已经部署在树莓派和新浪云APP上。因为树莓派实在太卡，所以数据库用的是sqlite3，但新浪APP ENGINE，只支持MYSQL，所以在SAE上面，数据库是MYSQL。树莓派访问地址：http://buf1024.xicp.net:3721/ SAE访问地址http://bigfalse.sinaapp.com/ 。已经将源代码放到github上面，github上面是sae版本，github页面：https://github.com/buf1024/bigfalse。    

特点/优点/缺点：  
   1. 登陆，管理等都是用http，及其容易盗取管理员密码。  
   2. 全文搜索没有实现  
   3. RSS没有实现，没有分享没有实现  
   4. 没有实现用windows live发布文章  
   5. SAE版本，没实现评论回复邮件通知，树莓派有实现，不过开的话，太卡，关闭了。  
   6. 首页，管理页面，更多页面采用瀑布流模式  
   7. 界面还算清新  
…………  

还有一大堆缺点。呵呵。

