---
title: 微信web通信协议
date: 2017-04-20 21:12:03
aliases: [/2017/04/20/weixin-web-proto/]
categories: [Web]
tags: [WeChat]
---
微信有网页版，[网页版的微信](https://wx.qq.com/)具备手机微信大部分功能。理论上，放在web页面上的所有东西，都是可以被破解的。对于web微信协议的破解，一些没有接触过前端的后端程序员，可能会想到两个方法：1. 研究网页js代码; 2. 大鲨鱼(wireshark)抓包分析（ps：参考之前写过[tcpdump usage](https://luoguochun.cn/2015/07/25/tcpdump-usage/)，[tcp 协议小结](https://luoguochun.cn/2016/09/23/tcp-fuck/), [ssl/tls 握手](https://luoguochun.cn/2016/11/16/https-handshake/)）。对于方法1，现在js的编写方法和以前有很大的不同，web前端通常用模块化方法进行js编写，在发布时，用webpack等工具进行压缩混淆，所以该方法不太可行。对于方法2. 因为现在大多web都采用https进行传输，传输过程中的数据全是加密的，所以这种办法更不可行。对于web前端和与web前端有交互的后端开发人员，都清楚还有这么一个非常常用的方式： 使用浏览器自带的开发者工具即可。理论上我们只要认真分析web微信和服务端的交互过程，分析请求参数和应答结果，即可得出相关的协议。下图为chrome开发者工具的截图：

![chrome微信](/img/weixin/weixinchrome.png)

微信的web协议网上已经有很多人进行分析过，网上进行搜索即可找到，虽然有些字段与现有web版微信有些差别，但大部分协议都是可用的。如，发现不一致导致无法使用，用浏览器开发者工具进行分析一下即可发现差别。结合前人的分析和自己的摸索，web版微信大致的工作流程如下：

![chrome微信](/img/weixin/weixinweb.png)

因为http是无状态的协议，web版微信又不是采用websocke协议，所以在获取到二维码后，需要查询登陆状态，在登陆后，需要定时进行Sync Check获取微信状态，并进行Sync实现获取消息和秘钥同步。

在获取web微信协议后，采用微信web协议进行开发的本质就是模拟桌面版的web浏览器。所以在开发需要注意：

1. 设置http头User-Agent为桌面        -> 服务端会根据此字段判断是否为桌面
2. 设置http头Referer为微信的url      -> 可能会判断来源
3. 设置cookie                       -> 保存服务端的cookie，并上传服务端

用`golang`进行了一些登陆的模拟，代码：[weixinweb](https://github.com/buf1024/weixinweb)，结果如下：

![微信登陆模拟](/img/weixin/weixinsim.png)

这里没有对微信web协议的进行详细分析，详细分析还需要分析各个交互请求参数以及返回结果的含义，只是一时兴趣记录一下web应用的分析过程和分析方法。对于web微信协议，可自行搜索之。

完。
