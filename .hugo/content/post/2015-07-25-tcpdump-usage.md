---
title: tcpdump usage
date: 2015-07-25 14:26:04
aliases: [/2015/07/25/tcpdump-usage/]
categories: [tcp]
tags: [tcp, tcpdump]
---

#### Tcpdump使用方法  
（1）**第一种是关于类型的关键字主要包括`host`，`net`，`port`**  
例如： `host 210.27.48.2`，指明 `210.27.48.2`是一台主机，`net 202.0.0.0` 指明`202.0.0.0`是一个网络地址，`port 23` 指明端口号是`23`。如果没有指定类型，缺省的类型是`host`.
 
（2）第二种是确定传输方向的关键字主要包括`src` , `dst` ,`dst or src`, `dst and src`  
这些关键字指明了传输的方向，例如：`src 210.27.48.2`，指明ip包中源地址是`210.27.48.2` , `dst net 202.0.0.0` 指明目的网络地址是`202.0.0.0`。如果没有指明方向关键字，则缺省是`src or dst`关键字。

（3）第三种是协议的关键字，主要包括`fddi`，`ip`，`arp`，`rarp`，`tcp`，`udp`等类型  
Fddi指明是在FDDI(分布式光纤数据接口网络)上的特定的网络协议，实际上它是"ether"的别名，fddi和ether具有类似的源地址和目的地址，所以可以将fddi协议包当作ether的包进行处理和分析。其他的几个关键字就是指明了监听的包的协议内容。如果没有指定任何协议，则tcpdump将会监听所有协议的信息包。
 
除了这三种类型的关键字之外，其他重要的关键字如下：`gateway`, `broadcast`,`less`,`greater`,还有三种逻辑运算，取非运算是`not` ，`!`，与运算是`and`，`&&`;或运算 是`or`，`││`；这些关键字可以组合起来构成强大的组合条件来满足人们的需要，下面举几个例子来说明。普通情况下，直接启动tcpdump将监视第一个网络界面上所有流过的数据包。

    A. tcpdump –i eth0 –c 10  
    使用-i参数指定tcpdump监听的网络界面，这在计算机具有多个网络界面时非常有用，
    使用-c参数指定要监听的数据包数量，
    使用-w参数指定将监听到的数据包写入文件中保存
     
    B.想要截获主机172.16.14.107和主机172.16.14.27或172.16.14.99的通信，使用命令：（在命令行中使用括号时，一定要用’\’
    tcpdump host 172.16.14.107 and \ (172.16.14.27or172.16.14.99 \)
    
    C.如果想要获取主机172.16.14.107除了和主机172.16.14.27之外所有主机通信的ip包，使用命令：
    tcpdump ip host 172.16.14.107 and ! 172.16.14.27
    
    D.如果想要获取主机172.16.14.107接收或发出的telnet包，使用如下命令：
    tcpdump tcp port 23 host 172.16.14.107
    
    E.对本机的udp 123 端口进行监视 （123 为ntp的服务端口）
    tcpdump udp port 123

    F.系统将只对名为hostname的主机的通信数据包进行监视。主机名可以是本地主机，
    也可以是网络上的任何一台计算机。下面的命令可以读取主机hostname发送的所有数据： 
    tcpdump -i eth0 src host hostname
    
    G.下面的命令可以监视所有送到主机hostname的数据包： 
    tcpdump -i eth0 dst host hostname
    #src表示源，即发送
    #dst表示目的地，即接收
    
    H.我们还可以监视通过指定网关的数据包： 
    tcpdump -i eth0 gateway Gatewayname
    
    I.如果你还想监视编址到指定端口的TCP或UDP数据包，那么执行以下命令： 
    tcpdump -i eth0 host hostname and port 80
    
    J.如果想要获取主机172.16.14.107接收或发出的telnet包，使用如下命令：
    tcpdump tcp port 23 host 172.16.14.107
    
    K. 如果我们只需要列出送到80端口的数据包，用dst port 80；如果我们只希望看到返回80端口的数据包，用src port 80。 
    tcpdump –i eth0 host hostname and dst port 80  #目的端口是80 或者
    tcpdump –i eth0 host hostname and src port 80  #源端口是80, 80端口一般是提供http的服务的主机
  
tcpdump输出格式:  
总的的输出格式为：系统时间 来源主机.端口 > 目标主机.端口 数据包参数
  
如果要用wireshark分析数据，须加上`-s –w`选项，否则wireshark不能识别文件格式：  
`tcpdump -i eth0 -c 100 -s 0 -w /home/data.pcap`



实际上就是`BPF`语法，此语法同时适用于WIRESHARK获取报文的语法。


