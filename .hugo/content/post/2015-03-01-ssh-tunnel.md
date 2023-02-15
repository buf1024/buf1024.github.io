---
title: ssh 隧道访问局域网机器
date: 2015-03-01 18:12:15
categories: [ssh]
tags: [ssh]
---
1 SSH 免登陆密码

1.1 生产 RSA密钥对

    `$ssh-keygen -t rsa`

    ~/.ssh目录将生成id_rsa（私钥）和id_rsa.pub（公钥）

1.2 将公钥复制到需要自动登陆的机器上面，追加到文件.ssh/authorized_keys

     `$cat id_rsa.pub >> .ssh/authorized_keys`

1.3 将`​ .ssh/authorized_keys`文件的权限修改为**644**（至关重要）

sshd_config中设置GatewayPorts为yes

2. SSH 隧道访问局域网机器

2.1 局域网机器建立一条方向隧道

    `$nohup ssh -t -t -R 3033:127.0.0.1:22 user@外网IP`

    意思是，ssh登陆上外网，同时在外网机器上打开3033端口监听，这样在外网访问本机3033端口便转发的局域网机器的22端口

2.2 通过外网机器访问局域网机器

     `$ssh user@127.0.0.1 -p 3033`

2.3 设置外网机器SSH心跳，如果不设置ssh发心跳，该隧道很容易就超时。

    `$ sudo vim /etc/ssh/sshd_config`


```c
   # 添加
    ClientAliveInterval 30
    ClientAliveCountMax 8
```

---

以下参考资料


SSH: Port Forwarding

1.正向隧道-隧道监听本地port,为普通活动提供安全连接

ssh -qTfnN -L port:host:hostport -l user remote_ip

2.反向隧道----隧道监听远程port，突破防火墙提供服务

ssh -qTfnN -R port:host:hostport -l user remote_ip

3.socks代理

SSH -qTfnN -D port remotehost（用证书验证就直接主机名，没用的还要加上用户名密码）

-q Quiet mode. 安静模式，忽略一切对话和错误提示。

-T Disable pseudo-tty allocation. 不占用 shell 了。

-f Requests ssh to go to background just before command execution. 后台运行，并推荐加上 -n 参数。

-n Redirects stdin from /dev/null (actually, prevents reading from stdin). -f 推荐的，不加这条参数应该也行。

-N Do not execute a remote command. 不执行远程命令，专为端口转发度身打造。

2.

ssh实现转发, 只要用到以下两条命令:

```c
# ssh -CfNg -L 6300:127.0.0.1:1521 oracle@172.16.1.164

# ssh -CfNg -R 1521:127.0.0.1:6300 oracle@172.16.1.164
```

不论是做跳板, 还是加密隧道, 还是加密其他的网络连接也都是这两条命令. 视具体情况而定, 有时只要用到其中一条, 有时两条都要用到.

命令解释:

1. -CfNg

C表示压缩数据传输

f表示后台用户验证,这个选项很有用,没有shell的不可登陆账号也能使用.

N表示不执行脚本或命令

g表示允许远程主机连接转发端口

2. -L 本地转发

```c
# ssh -CfNg -L 6300:127.0.0.1:1521 oracle@172.16.1.164
```

本机(运行这条命令的主机)打开6300端口, 通过加密隧道映射到远程主机172.16.1.164的1521端口(使用远程主机oracle用户). 在本机上用netstat -an|grep 6300可看到. 简单说,本机的6300端口就是远程主机172.16.1.164的1521端口.

1. -R 远程转发

```c
# ssh -CfNg -R 1521:127.0.0.1:6300 oracle@172.16.1.164
```

作用同上, 只是在远程主机172.16.1.164上打开1521端口, 来映射本机的6300端口.

4. 实用例子

有A,B,C 3台服务器, A,C有公网IP, B是某IDC的服务器无公网IP. A通过B连接C的80端口(A<=>B<=>C), 那么在B上执行如下命令即可:

$ ssh -CfNg -L 6300:127.0.0.1:80 userc@C

$ ssh -CfNg -R 80:127.0.0.1:6300 usera@A

服务器A和服务器C之间, 利用跳板服务器B建立了加密隧道. 在A上连接127.0.0.1:80, 就等同C上的80端口. 需要注意的是, 服务器B上的6300端口的数据没有加密, 可被监听, 例:

tcpdump -s 0-i lo port 6300

3.

[http://bianbian.org/technology/264.html](http://bianbian.org/technology/264.html)

里有一些好用的网址，介绍ssh及其代理穿透防火墙的

[http://www.inet.no/dante/](http://www.inet.no/dante/)

Dante -- Proxy communication solution

ssh 是有端口转发功能的。（转）

[http://doc.linuxpk.com/80817.html](http://doc.linuxpk.com/80817.html)

ssh的三个强大的端口转发命令：

QUOTE:

ssh -C -f -N -g -L listen_port:DST_Host:DST_port [user@Tunnel_Host](mailto:user@Tunnel_Host)

ssh -C -f -N -g -R listen_port:DST_Host:DST_port [user@Tunnel_Host](mailto:user@Tunnel_Host)

ssh -C -f -N -g -D listen_port [user@Tunnel_Host](mailto:user@Tunnel_Host)

-f Fork into background after authentication.

后台认证用户/密码，通常和-N连用，不用登录到远程主机。

-p port Connect to this port. Server must be on the same port.

被登录的ssd服务器的sshd服务端口。

-L port:host:hostport

将本地机(客户机)的某个端口转发到远端指定机器的指定端口. 工作原理是这样的, 本地机器上分配了一个 socket 侦听 port 端口, 一旦这个端口上有了连接, 该连接就经过安全通道转发出去, 同时远程主机和 host 的 hostport 端口建立连接. 可以在配置文件中指定端口的转发. 只有 root 才能转发特权端口. IPv6 地址用另一种格式说明: port/host/hostport

-R port:host:hostport

将远程主机(服务器)的某个端口转发到本地端指定机器的指定端口. 工作原理是这样的, 远程主机上分配了一个 socket 侦听 port 端口, 一旦这个端口上有了连接, 该连接就经过安全通道转向出去, 同时本地主机和 host 的 hostport 端口建立连接. 可以在配置文件中指定端口的转发. 只有用 root 登录远程主机才能转发特权端口. IPv6 地址用另一种格式说明: port/host/hostport

-D port

指定一个本地机器 “动态的'’ 应用程序端口转发. 工作原理是这样的, 本地机器上分配了一个 socket 侦听 port 端口, 一旦这个端口上有了连接, 该连接就经过安全通道转发出去, 根据应用程序的协议可以判断出远程主机将和哪里连接. 目前支持 SOCKS4 协议, 将充当 SOCKS4 服务器. 只有 root 才能转发特权端口. 可以在配置文件中指定动态端口的转发.

-C Enable compression.

压缩数据传输。

-N Do not execute a shell or command.

不执行脚本或命令，通常与-f连用。

-g Allow remote hosts to connect to forwarded ports.

在-L/-R/-D参数中，允许远程主机连接到建立的转发的端口，如果不加这个参数，只允许本地主机建立连接

Linux命令行下SSH端口转发设定笔记(转)

原文:[http://be-evil.org/post-167.html](http://be-evil.org/post-167.html)

在Windows下面我们可以很方便的使用putty等ssh工具来实现将服务器上的端口映射到本机端口来安全管理服务器上的软件或者服务 那么我们换到在Liunx下我们应该怎么做呢？

ssh -L 本地端口:服务器地址:服务器端口 用户名@服务器地址 -N

参数详解:

-L 端口映射参数 本地端口 - 这个任意即可，只要本机没有其他的程序占用这个端口就行

服务器地址 - 你需要映射的服务器地址（名称/ip）

服务器端口 - 远程的服务器端口

-N - 不使用Shell窗口，纯做转发的时候用，如果你在映射完成后继续在服务器上输入命令，去掉这个参数即可

例子A:我们想远程管理服务器上的MySQL,那么使用下面命令

ssh -L 3306:127.0.0.1:3306 [user@emlog-vps](mailto:user@emlog-vps) -N

运行这个命令之后，ssh将会自动将服务器的3306映射到本机的3306端口，我们就可以使用任意MySQL客户端连接 localhost:3306即可访问到服务器上的MySQL了。

例子B:一次同时映射多个端口

ssh -L 8888:[www.host.com:80-L](http://www.host.com:80-L) 110:mail.host.com:110 \ 25:mail.host.com:25 [user@host](mailto:user@host) -N

这个命令将自动把服务器的80，110，25端口映射到本机的8888，110和25端口 以上命令在ubuntu 9.10 上测试通过...

关于ssh端口转发的深入实例[转]

[http://blog.myhnet.cn/2009/01/08/deepin-instances-of-ssh-port-forwarding/](http://blog.myhnet.cn/2009/01/08/deepin-instances-of-ssh-port-forwarding/)

首先，认识下这三个非常强大的命令：

ssh -C -f -N -g -L listen_port:DST_Host:DST_port [user@Tunnel_Host](mailto:user@Tunnel_Host)

ssh -C -f -N -g -R listen_port:DST_Host:DST_port [user@Tunnel_Host](mailto:user@Tunnel_Host)

ssh -C -f -N -g -D listen_port [user@Tunnel_Host](mailto:user@Tunnel_Host)

相关参数的解释：

-f Fork into background after authentication.

后台认证用户/密码，通常和-N连用，不用登录到远程主机。

-L port:host:hostport

将本地机(客户机)的某个端口转发到远端指定机器的指定端口. 工作原理是这样的, 本地机器上分配了一个 socket 侦听 port 端口, 一旦这个端口上有了连接, 该连接就经过安全通道转发出去, 同时远程主机和 host 的 hostport 端口建立连接. 可以在配置文件中指定端口的转发. 只有 root 才能转发特权端口. IPv6 地址用另一种格式说明: port/host/hostport

-R port:host:hostport

将远程主机(服务器)的某个端口转发到本地端指定机器的指定端口. 工作原理是这样的, 远程主机上分配了一个 socket 侦听 port 端口, 一旦这个端口上有了连接, 该连接就经过安全通道转向出去, 同时本地主机和 host 的 hostport 端口建立连接. 可以在配置文件中指定端口的转发. 只有用 root 登录远程主机才能转发特权端口. IPv6 地址用另一种格式说明: port/host/hostport

-D port

指定一个本地机器 “动态的’’ 应用程序端口转发. 工作原理是这样的, 本地机器上分配了一个 socket 侦听 port 端口, 一旦这个端口上有了连接, 该连接就经过安全通道转发出去, 根据应用程序的协议可以判断出远程主机将和哪里连接. 目前支持 SOCKS4 协议, 将充当 SOCKS4 服务器. 只有 root 才能转发特权端口. 可以在配置文件中指定动态端口的转发.

-C Enable compression.

压缩数据传输。

-N Do not execute a shell or command.

不执行脚本或命令，通常与-f连用。

-g Allow remote hosts to connect to forwarded ports.

在-L/-R/-D参数中，允许远程主机连接到建立的转发的端口，如果不加这个参数，只允许本地主机建立连接。注：这个参数我在实践中似乎始终不起作用

实例说明：

一台服务器提供ftp服务，因为ftp传输是明文密码，如果不做ssh端口之前，我们可以通过tcpdump命令很容易的捕捉到明文信息。所以我们要对21端口进行转发：

ftp-server# ssh -CNfg -R 2121:localhost:21 [root@100.0.0.50](mailto:root@100.0.0.50)

然后登录到100.0.0.50机器，我们可以通过netstat -an|grep :2121查看端口已经侦听

100.0.0.50# ftp localhost 2121

就可以登录到ftp-server了，而且tcpdump无法捕获到有效的信息。

2121端口任意选择，只要是机器上没有占用的端口就行

来一个稍微复杂一点的，做网关的例子：

假如内网有一台提供ftp(linux，port is2121，称为A机器)的机器，通过网关服务器(linux，port is8888，称为B机器)进去，现在外网有一台C机器需要访问网关服务器的某个端口(port is21)来访问内网的ftp服务器。大家可以看到，其实这就像是一个基于ssh的防火墙程序，好，下面我们来具体操作：

1、login A 机器

ssh -CNfg -R 8888:localhost:2121 [root@B](mailto:root@B)机器IP

这样我们就在B机器上开了一个8888->2121的端口转换，但是由于8888端口只能侦听在localhost主机上，因此，虽然我们已经可以在B机器上使用

ftp localhost 8888

来访问真正的ftp服务器，但仍然无法提供给外网的机器访问

2、login B机器

ssh -CNfg -L 21:localhost:8888 [root@localhost](mailto:root@localhost)

这样做，是做本地机器上的21->8888端口转换，可以侦听在任何地址上的请求。

2(1)。

如果C机器也是一台linux机器，那也可以这样设置：

ssh -CNfg -R 21:localhost:8888 [root@C](mailto:root@C)机器IP

3。使用C机器，

可以是linux下的ftp命令，也可以是windows下的客户端软件都可以访问B机器的21端口来连接后台真正的ftp服务器(真正的端口是2121)

如果是按照2(1)中的设置，则访问的地址为本机IP。