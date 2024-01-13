---
title: bt种子文件文件结构
date: 2014-09-17 19:12:17
aliases: [/2014/09/17/bt-file-structure/]
categories: [Misc]
tags: [BT]
---
估计80%以上接触互联网的人都知道bt是什么东西，任何一个用bt下载的人都知道这样一个概念，种子。bt种子就是记录了p2p对等网络中tracker, nodes, files等信息，也就是说，这个种子告诉你，你要下载什么，到哪里下载。bt种子文件有自己的文件格式，下面简单看看bt种子文件的结构。  
在开始之前，我们先介绍bencode这种编码格式，因为bt种子文件，包括以后的DHT网络中，都是用这种编码的。网上有很多介绍，这里简单再重复一遍。bencode有4种数据类型:string, integer, list 和 dictionary。  
  
1. string  
  字符是以这种方式编码的: <字符串长度>:<字符串>。  
  如hell: 4:hell
2. integer  
  整数是一这种方式编码的: i<整数>e。  
  如1999: i1999e
3. list  
 列表是一这种方式编码的: l[数据1][数据2][数据3][...]e。  
 如列表[hello, world, 101]： l5:hello5:worldi101ee  
4. dictionary  
  字典是一这种方式编码的: d[key1][value1][key2][value2][...]e，其中key必须是string而且按照字母顺序排序。  
  如字典{aa:100, bb:bb, cc:200}： d2:aai100e2:bb2:bb2:cci200ee  

很多语言都有bencode的实现，[Python](http://Python.org)版本可在pypi里面找到: [bencode](https://pypi.Python.org/pypi/bencode/1.0)。

bt种子文件是使用bencode编码的，整个文件就dictionary，包含以下键。  

1. info, dictinary, 必选, 表示该bt种子文件的文件信息。  

  文件信息包括文件的公共部分  
  
  piece length, integer, 必选, 每一数据块的长度
  pieces, string, 必选, 所有数据块的SHA1校验值  
  publisher, string, 可选, 发布者  
  publisher.utf-8, string, 可选, 发布者的UTF-8编码  
  publisher-url, string, 可选, 发布者的URL  
  publisher-url.utf-8, string, 可选, 发布者的URL的UTF-8编码  
  
  如果bt种子包含的是单个文件，包含以下内容  
  
  name, string, 必选, 推荐的文件名称
  name.utf-8, string, 可选, 推荐的文件名称的UTF-8编码  
  length, int, 必选， 文件的长度单位是字节  
  
  如果是多文件，则包含以下部分:  
  
  name, string, 必选, 推荐的文件夹名称
  name.utf-8, string, 可选, 推荐的文件名称的UTF-8编码  
  files, list, 必选, 文件列表，每个文件列表下面是包括每一个文件的信息，文件信息是个字典。  
  
  文件字典  
  
  length, int, 必选， 文件的长度单位是字节  
  path, string, 必选， 文件名称，包含文件夹在内
  path.utf-8, string, 必选， 文件名称UTF-8表示，包含文件夹在内
  filehash，string, 可选， 文件hash。  
  ed2k, string, 可选, ed2k信息。  
  
2. announce, string, 必选, tracker 服务器的地址  

3. announce-list, list, 可选, 可选的tracker服务器地址  

4. creation date， interger， 必选, 文件创建时间  

5. comment， string, 可选, bt文件注释  

6. created by， string， 可选， 文件创建者。  

上面列举的可能不是很完整的，但是大体上主要的字段没有重大的错误。
