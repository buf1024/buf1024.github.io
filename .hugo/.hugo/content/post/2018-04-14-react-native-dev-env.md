---
title: react-native 开发环境配置 -- 调试部分
date: 2018-04-14 13:44:08
aliases: [/2018/04/14/react-native-dev-env/]
categories: [reactjs]
tags: [react-native]
---
在学习一们新的语言或编程技术的时候，我总偏向于弄清楚它的调试工具和技巧。因为我总觉得我写的程序有大大的bug，而打印日志这种最原始的调试方式总觉得不登大雅之堂，总想找到适当的调试工具进行断点跟踪，这样更容易理解其原理。

按照官方的文档介绍，大体有两种(其他react-devtools，控制台日志之类的过于鸡肋，忽略)，一种是**Remote JS Debug**和[Stetho](http://facebook.github.io/stetho/)。**Remote JS Debug**除了抛出一些不是自己代码的异常外，基本可以满足断点跟踪的问题。[Stetho](http://facebook.github.io/stetho/)这个工具需要修改原生代码(当然是简单修改)，提供更强大的调试功能，可以查看内部数据库。安装文档说明修改代码后是可以在chrome的inspect页面上面发现被调试的app的，但是打开inspect时，却显示的是空白页面，后来了解到这个工具需要翻墙才可以使用，现在免费翻墙工具很多不能使用了，于是作罢。在浏览器上面查看模拟器内部的数据，比`adb pull`文件(如realm数据库)，在电脑看，是有无法比拟的好处，这个后面看有没有其他办法解决。

官方文档关于调试的，还有两个没有说清楚。**一个是如何跟踪http/https的请求应答报文**。**另外一个是redux-devtools怎么使用**。当然文档没说明也无可厚非，两者都是不属于react-native的内容，尽管如此，这两个是日常开发中最常用到的（至少对于我来说）。这两个问题的解决，如果没有接触过手机端开发，解决还是容易有点暴躁。下面记录一下。

第一个，http/https报文。从事后台开发的都可能知道，使用tcpdump/wireshark进行抓包。但是这种方法太过于原始，更重要的是看不出https报文的内容。所以自然让人想到另外一个专门的http抓包工具[Fiddler](https://www.telerik.com/fiddler)，[Fiddler](https://www.telerik.com/fiddler)通过代理的方式进行报文抓取。[Fiddler](https://www.telerik.com/fiddler)在windows下面工作良好，但是在mac下面却需要安装mono框架，mac容量小，作罢。相对应的，mac有一款叫做[Charles](https://www.charlesproxy.com/)的工具，原来和[Fiddler](https://www.telerik.com/fiddler)类似，也是通过代理的方式进行抓包，https需要安装它自带证书。因为[genymotion](https://www.genymotion.com)模拟器没有自带浏览器，所以只能通过电脑下载好证书后，通过`adb push`到手机里面。模拟器安装证书之前，需要设置一个**PIN**密码，证书使用范围选择wifi。安装好证书后，如果一切顺利，就没有必要记录下这个过程了。

- **坑一**，安装证书设置**PIN**后，下次开启模拟器时，屏幕是锁着的，模拟器提示'swipe up to unlock'，如果我没理解错的话，意思应该是上滑解锁。是事实上，**无论你怎么上滑，都无法解锁屏幕**。无意中发现，按菜单键调出紧急呼叫按钮，输入**PIN**才能解锁，这是个很令人抓狂的坑。
- **坑二**，安装证书设置**PIN**后，再打开**Remote JS Debug**，你会发现你是无论怎么样，都是无法连接上debuger的，打开的是‘10.0.3.2’的一个硬编码地址。需要在‘dev settings’(command+m调出)中设置'debug server host & port for device'设置调试服务器。
- **坑三**，设置wifi代理到抓包工具后，在使用`react-native start;react-native run-android`，是无论如何也是启动不了的。正确的顺序应该是先启动app在设置代理。

综上，正确开启http/https报文追踪步骤为：1，安装证书，设置**PIN**；2，启动app；3，设置wifi代理，debug server&host。步骤错误，很容易app就无法启动。另外**Remote JS Debug**时，无法弹出调试页面，提示缓存，需要删除缓存。

第二个，redux-devtools。redux-devtools在web上面用的是很爽的。但在react-native上面，是无法工作的，可以使用[remote-redux-devtools](https://github.com/zalmoxisus/remote-redux-devtools)这个工具进行解决。但官网文档的说明有个坑。在‘Enabling’一章节，说的是设置`process.env.NODE_ENV === 'development'`，就可以生效了。然而使用react-native工具生成的工程，是没有地方设置这个环境变量的（或者有，我不知道）。等效的方法是，`export NODE_ENV='development'`，这样设置后，十次实验可能有一两次能使用，这是很令人烦躁的。正确的做法是文档中说的第二个方法：`devToolsEnhancer({ realtime: true })`，但这个方法需要注意的是在生产中改为false。

工欲善其事，比先利其器。调试工具搞好后，接下来的开发就会顺很多。

 