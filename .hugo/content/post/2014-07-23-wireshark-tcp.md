---
title: 'wireshark tcp 协议分析'
date: 2014-07-23 15:28:16
aliases: [/2014/07/23/wireshark-tcp/]
categories: [TCP]
tags: [TCP,Wireshark]
---

注：本文比较简略，且存在错漏。更详细的总结参考[tcp协议小结](https://buf1024.github.io/2016/09/23/tcp-fuck)，后续如若发现有错漏，则同时在[tcp协议小结](https://buf1024.github.io/2016/09/23/tcp-fuck)中进行修正，本文不再做维护。   --2016-09-23

虽然知道wireshark是抓包神器，只会大概大概用一下，还用一下下tcpdump，略懂一点BPF过滤器，也知道一点怎么用wirkshark过滤相关的报文，但是对于详细的字段的含义，如何查看TCP的交互情况还不是非常的了解。现在，简单分析一下。PS：这次抓包的对象是传说中通过公安局多少多少级认证的本公司开发的交易系统，本来看到他的验证码倾斜的很有规律，叫的斑斑点点也不是很密集。就想写个小程序练习一下验证码识别，可是我失望了，在wireshark里面居然没有抓到任何报文，这个东西的验证码居然是客户端生成的，无语。于是，抓下登录过程的报文，看看能否破解，相关的TCP报文：[crack.pcapng](/raw/tcp/wiresharktcp-crack.pcapng)。关于报文分析，有一个很好的E文网站:[packetlife](http://packetlife.net/)。

废话少说，简单在看看TCP的协议头：
![TCP协议头](/img/tcp/tcp-header.png)
这张图片有点过期，保留位是6位，实际的情况是，保留位的后2位已经被使用了。保留位的第5位是Congestion Window Reduced(*CWR*)，第6位是ECN-Echo(*ECN*)。TCP协议的其他部分不说，先看看TCP协议的几个不是很了解标志是什么意思。  

- *CWR*(Congestion Window Reduced)  
 简单来说就是网络不是很畅通了，通知对方减少阻塞窗口，发包速度发慢一点。  

- *ECN*（ECN-Echo）  
 *ECN*两个作用，在TCP三次握手时表明TCP端是否支持ECN；在传输数据时，发送方是无法知道网络是否畅通的，但是经过重重的路由后，路由根据网络的情况可以知道是否阻塞，路由会设置在IP层会设置的相应的标志，即接收端发现了拥塞。*CWR*为发送端缩小拥塞窗口标志，用来通知发送端它已经收到了设置*ECN*标志，应该减慢发包速度。关于*ECN*的详细描述请参考：[*ECN*](http://blog.csdn.net/zhangskd/article/details/7246503)  

- *URG*(Urgent)  
 这就是传说中的带外数据。因为TCP是没有消息边界的，假如有一种情况，你已经发送了一些数据，但是此时，你要发送一些数据优先处理，就可以设置这些标志，同时如果设置了这个标志，紧急指针也会设置为相应的偏移。当接受方收到*URG*数据时，不缓存在接收窗口，直接往上传给上层。具体的使用可以参考[TCP带外数据](http://wenku.baidu.com/view/f04a4dff9e31433239689341.html)。大体来说，就是，调用`send`和`recv`是要加上`MSG_OOB`参数。同时接收方要处理`SIGURG`信号。不过据说这个带外数据在实际上，用得很少。

- *PSH*（Push）  
 简单来说，就是告诉对方，我发这么多数据了，你可以处理了，不用缓冲在接收窗口了，直接交数据给上层吧。如果设置了`SO_NODELAY`选项，可以强制设置这个标志，如果设置了这个标志，数据就不缓冲在发送窗口那里，直接发送。  

TCP报文SYN ACK的计算如下：  

        A -> B SYN J ACK K LEN L  
        B -> A SYN K ACK J+L LEN M  
        A -> B SYN J+L ACK K+M

具体看下wireshark抓到的报文：  

1. TCP3次握手的部分是帧1到帧3。
 ![建立连接](/img/tcp/wiresharktcp-tcp-cap-1.png)  

- 第1帧，发送SYN J:  
  A -> B seq = 0, win = 8192, len = 0, MSS = 1440, WS = 4, SACK_PERM = 1  
  WS(Window Scale), 4表示左移动4位，原来窗口大小是16为，现在是20为，现代扩大了2^4倍，关于WS，这里有比较详细的描述[tcp-windows-and-window-scaling](http://packetlife.net/blog/2010/aug/4/tcp-windows-and-window-scaling/)。这里比较疑惑的就是SACK_PERM这个TCP选项。SACK（Select ACKnowledgement）的目的就是当出现大量的报文丢失时增加恢复时间来用的，类似于累计ACK，就是说N多个ACK合成一个SACK。关于SACK，有两个地方描述的比较详细[SelectiveAcknowledgements](http://kb.pert.geant.net/PERTKB/SelectiveAcknowledgements),[TCP Selective Acknowledgment](http://msdn.microsoft.com/en-us/library/aa917455.aspx)。
- 第2帧，发送SYN K， ACK J+1:  
  B -> A seq = 0, ACK = 1, Win = 14600, Len = 0, MSS = 1448, SACK_PERM = 1 WS = 128  
  这些含义看第1帧，win = 14600, WS = 128，可以看到这台服务器的窗口非常大，WS也很多，网络性能应该不错的（事实也如此）。  
- 第3帧，发送SYN J+1， ACK K+1:
  A -> B seq = 1, ACK = 1, win = 66608, Len = 0  
  这是建立TCP连接的第3次握手，这时win = 66608了，转换为2进制有17位比16位长，因为再第1帧第2帧的交互中已经交互了各种的TCP选项，所以这次的确认不带有TCP选项。  

 当这3次交互完成后，连接真正建立，只要服务端accept后，就可以接收和发送数据了。  

2. TCP数据传输  
 ![普通数据传输](/img/tcp/wiresharktcp-tcp-cap-2.png)  
 截图的是报文的第7帧，这个帧报文在这次抓的报文中相对有代表性点的。这个帧的报文设置了`PSH`标志，而且是TCP分片传输的报文，因为此帧的报文是第6帧报文分片传输的，从`ACK = 125`可以看出。传输数据的报文没有什么特别可以说的：~  

3. TCP终止连接的4次交换的部分是帧19到帧21（可以发现，这里的交互是有问题的）。  
 ![终止连接](/img/tcp/wiresharktcp-tcp-cap-2.png)  

- 第19帧，发送FIN J, ACK K:  
   A -> B seq = 2559, ack = 2361, win = 65812, len = 0  
   客户端发起FIN主动关闭连接和上个报文的ACK（应该是接收完了数据，关闭SOCKET），客户端最后应该会变成TIME_WAIT状态。这是第一次交换。  
- 第20帧，发送FIN K， ACK J+1:  
   B -> A seq = 2361, ack = 2560, Win = 26240, Len = 37  
   这次交换中，除了对客户端的ACK外，同时发送FIN，但同时带有37字节的数据，这37个数据不是我们期待有的。可以猜测一下，可能是服务端里面有37个字节还没有发送，在收到FIN后，把缓存里面的数据全部发送过去。服务端如果忘记的关闭连接，会变成CLOSE_WAIT状态。这里两次的交换合并在一起了。
- 第21帧，发送RST， ACK K+Len:
   A -> B seq = 2560, ACK = 2398, win = 0, Len = 0  
   主动关闭一方收到FIN，回应ACK。但是这里却有一个不是我们期待的`RST`标志。`RST`标志表明往已经关闭连接发送数据，这是个错误。这是第四次交换。  

 这里的客户端与服务端的交换是有问题的，在第20帧，收到FIN时，不应该再发送数据，这样发送的数据很有可能收到的就是`RST`。但是这并不一定是发送数据一方的问题，很有可能是客户端还没有接受完数据就关闭连接了。但可以肯定的是，在客户端或服务端某个地方肯定存在BUG。  

这个就是某交易系统登录的报文，报文涉及5次数据交互（请求-应答）。这有5次交换，第1，2次交换，很可能是交换RSA公钥（猜的，因为报文数据有OpenSSL标志：~）。然而后面的还有3次数据交互，并不是我期待的一次交互。难道还要同步其他密钥之类的？有空问下相关开发人员。如果是单纯破解报文的话，存在比较大的难度，但是如果是`DOS`攻击的话，这应该是非常简单的……  

注：本文比较简略，且存在错漏。更详细的总结参考[tcp协议小结](https://buf1024.github.io/2016/09/23/tcp-fuck)，后续如若发现有错漏，则同时在[tcp协议小结](https://buf1024.github.io/2016/09/23/tcp-fuck)中进行修正，本文不再做维护。   --2016-09-23
