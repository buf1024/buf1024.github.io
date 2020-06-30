---
title: 高可用系统
date: 2018-09-01 19:54:09
aliases: [/2018/09/01/high-available-loadbalance/]
categories: [构架]
tags: [构架]
---
很久之前，在公司鼎盛的时期，还存在运维人员的时候，我们开发应用的时候，就会和运维人员讨论一下部署的方案，听的比较多的是，运维人员说的比较多的是两个专业的名词是，负载均衡和ip漂移，前者是为减轻应用服务的压力使性能均衡，后者是为了负载均衡宕机后系统的高可用性。由于这些都是运作部署的，我们只关注应用功能的实现，并不是很在意这些部署的细节，只是了解到有这样的东西。现在公司没落了，很多事情都得自己搞了。


负载均衡，无非就减轻后台应用服务器的压力，提高系统的可扩展性。方法是很多，比如有些在客户端实现，有些自己写分发程序，还有业界比较常用的F5负载均衡（贵，高大上），使用LVS/haproxy/nginx等开源的方案，这些都是支持4层/7层负载均衡。LVS已经集成在内核，没有使用过，haproxy/nginx曾经窥视过其内部数据结构，可没有非常深入的了解，nginx支持模块开发，如果对nginx比较熟悉，那么在其继承上开发对应的模块，其效率是比较高效的。这次用nginx的4层负载均衡测试一下，然而据说nginx的健康检查是惰性的，它不能及时知道后端服务的存活，所以有些人开发了相关的模块（更新年限久远，不知道是否能用）：[nginx_upstream_check_module](https://github.com/yaoweibin/nginx_upstream_check_module)

当应用部署上负载均衡后，负载均衡很容易成为单点，如果负载均衡挂掉了，这个时候keepalived上场。要了解keepalived的工作原理，首先要先了解[VRRP协议--虚拟路由器冗余协议](https://baike.baidu.com/item/%E8%99%9A%E6%8B%9F%E8%B7%AF%E7%94%B1%E5%99%A8%E5%86%97%E4%BD%99%E5%8D%8F%E8%AE%AE/2991482?fromtitle=VRRP&fromid=932628),这里有份详细的原理描述：[虚拟路由器冗余协议【原理篇】VRRP详解](http://blog.51cto.com/zhaoyuqiang/1166840)。简单来说，VRRP就是把N台路由器(机器)放到一个组里面，组里面有一个MASTER和N-1个BACKUP，对外拥有一个虚拟IP，MASTER所在的机器拥有这个虚拟IP，MASTER通过广播报文到组内的BACKUP，当BACKUP在规定时间内没有收到MASTER的广播报文，则认为MASTER宕机了，当MASTER宕机后，组内的BACKUP通过特定的选举算法机制，选择出一个MASTER，然后这个MASTER拥有这个虚拟IP，这样看起来，感觉就是“IP从一台机器漂移到另外一台机器”了。Keepalived就是一个实现VRRP协议高可用方案。

用以前在学习nodejs写的echo服务器模拟实际上的应用服务器，开三台虚拟机(VNODE-01 ~ VNODE-03), 简单记录一下简单测试步骤：

1. 启动应用，分别在VNODE-01 ~ VNODE-03部署应用服务

```shell
VNODE-01:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:43:db:4d brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.78/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 17744sec preferred_lft 17744sec
    inet6 fe80::950a:8286:4242:1da3/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

able@VNODE-01:~/luoguochun/privt/proj/privt-prj/web/nodejs/node-demo/echo-svr$ node echo-svr.js -h 172.20.46.78 -p 9978 -i vnode-01
server(vnode-01) listen 172.20.46.78:9978 started

VNODE-02:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:d0:b6:f5 brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.80/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 17513sec preferred_lft 17513sec
    inet6 fe80::146f:4835:5b3e:29c1/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

able@VNODE-02:~/luoguochun/privt/proj/privt-prj/web/nodejs/node-demo/echo-svr$ node echo-svr.js -h 172.20.46.80 -p 9980 -i vnode-02
server(vnode-02) listen 172.20.46.80:9980 started

VNODE-03:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:0e:28:a6 brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.79/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 17720sec preferred_lft 17720sec
    inet6 fe80::e7aa:1f55:1bdc:aaeb/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
able@VNODE-03:~/luoguochun/privt/proj/privt-prj/web/nodejs/node-demo/echo-svr$ node echo-svr.js -h 172.20.46.79 -p 9979 -i vnode-03
server(vnode-03) listen 172.20.46.79:9979 started
```

2. 4层负载均衡：分别在VNODE-01 ~ VNODE-02部署nginx，假设VNODE-01为主机，VNODE-02为备机

``` shell
2.1 部署nginx: 简略配置(负载策略，按权重轮询)
user able;
worker_processes 1;
error_log /tmp/nginx_error.log;
pid /tmp/nginx.pid;
include /etc/nginx/modules-enabled/*.conf;

events {
	worker_connections 768;
	# multi_accept on;
}

stream {
    upstream echo_svr {
        server 172.20.46.78:9978 weight=40;
        server 172.20.46.79:9979 weight=30;
        server 172.20.46.80:9980 weight=30;
    }
    server {
        listen 9900;
        proxy_pass echo_svr;
    }
}



2.1 SHELL查看：
VNODE-01:
able@VNODE-01:/media/sf_luoguochun/privt/proj/privt-prj/arch/nginx$ sudo nginx -c /media/sf_luoguochun/privt/proj/privt-prj/arch/nginx/nginx-lb4layer.conf
[sudo] password for able: 
able@VNODE-01:/media/sf_luoguochun/privt/proj/privt-prj/arch/nginx$ ps -ef --forest | grep -Ev 'grep' | grep nginx
root      2131     1  0 09:41 ?        00:00:00 nginx: master process nginx -c /media/sf_luoguochun/privt/proj/privt-prj/arch/nginx/nginx-lb4layer.conf
able      2132  2131  0 09:41 ?        00:00:00  \_ nginx: worker process

VNODE-02:
able@VNODE-02:/media/sf_luoguochun/privt/proj/privt-prj/arch/nginx$ sudo nginx -c /media/sf_luoguochun/privt/proj/privt-prj/arch/nginx/nginx-lb4layer.conf 
[sudo] password for able: 
able@VNODE-02:/media/sf_luoguochun/privt/proj/privt-prj/arch/nginx$ ps -ef --forest | grep -Ev 'grep' | grep nginx
root      2335     1  0 09:44 ?        00:00:00 nginx: master process nginx -c /media/sf_luoguochun/privt/proj/privt-prj/arch/nginx/nginx-lb4layer.conf
able      2336  2335  0 09:44 ?        00:00:00  \_ nginx: worker process


2.3 简单测试，可看到nginx已经对后端服务进行负载：
^_^@/Users/luoguochun]$ telnet 172.20.46.78 9900
Trying 172.20.46.78...
Connected to bogon.
Escape character is '^]'.
hello, vnode
echo(from vnode-01):hello, vnode
^]
telnet> quit
Connection closed.
^_^@/Users/luoguochun]$ telnet 172.20.46.78 9900
Trying 172.20.46.78...
Connected to bogon.
Escape character is '^]'.
hello, vnode

echo(from vnode-03):hello, vnode
echo(from vnode-03):
^]
telnet> quit
Connection closed.
```

3. 高可用：分别在VNODE-01 ~ VNODE-02部署keepalived，假设VNODE-01为MASTER，VNODE-01为BACKUP

``` shell
3.1 keepalived配置
MASTER 简略配置:
global_defs {
    router_id MASTER_ROUTER_ID
}
vrrp_script chk_svr {
    script "killall -0 node"
    interval 2
    weight -5
    fall 3  
    rise 2
}

vrrp_instance VI_1 {
    state MASTER
    interface enp0s3
    mcast_src_ip 172.20.46.78
    virtual_router_id 99
    priority 101
    advert_int 2
    authentication {
        auth_type PASS
        auth_pass 1111
    }
    virtual_ipaddress {
        172.20.46.222
    }
    track_script {
       chk_svr
    }
}
BACKUP 简略配置:
与MASTER几乎相同，不同的是:
state MASTER -> state BACKUP
mcast_src_ip 172.20.46.78 -> mcast_src_ip 172.20.46.80
priority 101 -> priority 100

2. SHELL查看，可以看到MASTER除了拥有一个真实IP外，还有一个虚拟IP（ip a 查看, ifconfig 查看不到）
VNODE-01:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:43:db:4d brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.78/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 15610sec preferred_lft 15610sec
    inet 172.20.46.222/32 scope global enp0s3
       valid_lft forever preferred_lft forever
    inet6 fe80::950a:8286:4242:1da3/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
VNODE-02:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:d0:b6:f5 brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.80/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 17513sec preferred_lft 17513sec
    inet6 fe80::146f:4835:5b3e:29c1/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

3. 测试
3.1 应用测试，可以正常使用虚拟IP进行转发处理
^_^@/Users/luoguochun]$ telnet 172.20.46.222 9900
Trying 172.20.46.222...
Connected to bogon.
Escape character is '^]'.
hello, keepalived
echo(from vnode-02):hello, keepalived
^]
telnet> quit
Connection closed.
^_^@/Users/luoguochun]$ telnet 172.20.46.222 9900
Trying 172.20.46.222...
Connected to bogon.
Escape character is '^]'.
hellow 
echo(from vnode-01):hellow
^]
telnet> quit
Connection closed.

3.2 模拟MASTER宕机(IP漂移到BACKUP)
VNODE-01:
able@VNODE-01:~/luoguochun/privt/proj/privt-prj/web/nodejs/node-demo/echo-svr$ sudo killall nginx keepalived

2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:43:db:4d brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.78/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 15355sec preferred_lft 15355sec
    inet6 fe80::950a:8286:4242:1da3/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

VNODE-02:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:d0:b6:f5 brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.80/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 15363sec preferred_lft 15363sec
    inet 172.20.46.222/32 scope global enp0s3
       valid_lft forever preferred_lft forever
    inet6 fe80::146f:4835:5b3e:29c1/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

^_^@/Users/luoguochun]$ telnet 172.20.46.222 9900
Trying 172.20.46.222...
Connected to bogon.
Escape character is '^]'.
hello, keepalived
echo(from vnode-02):hello, keepalived
^]
telnet> quit
Connection closed.
^_^@/Users/luoguochun]$ telnet 172.20.46.222 9900
Trying 172.20.46.222...
Connected to bogon.
Escape character is '^]'.
hellow 
echo(from vnode-01):hellow
^]
telnet> quit
Connection closed.

可见IP正确漂移到备机，除了正连接主机处理的有问题外，新请求正确负载到备机。


3.3 模拟MASTER恢复
在VNODE-01重启nginx和keepalived后，可以看到IP又漂移会主机

VNODE-01:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:43:db:4d brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.78/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 15206sec preferred_lft 15206sec
    inet 172.20.46.222/32 scope global enp0s3
       valid_lft forever preferred_lft forever
    inet6 fe80::950a:8286:4242:1da3/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

VNODE-02:
2: enp0s3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 08:00:27:d0:b6:f5 brd ff:ff:ff:ff:ff:ff
    inet 172.20.46.80/23 brd 172.20.47.255 scope global dynamic noprefixroute enp0s3
       valid_lft 17513sec preferred_lft 17513sec
    inet6 fe80::146f:4835:5b3e:29c1/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever

^_^@/Users/luoguochun]$ telnet 172.20.46.222 9900
Trying 172.20.46.222...
Connected to bogon.
Escape character is '^]'.
what
echo(from vnode-02):what
^]
telnet> quit
Connection closed.
^_^@/Users/luoguochun]$ telnet 172.20.46.222 9900
Trying 172.20.46.222...
Connected to bogon.
Escape character is '^]'.
youe
echo(from vnode-01):youe
^]
telnet> quit
Connection closed.
^_^@/Users/luoguochun]$

可见IP正确漂移到主机，除了正连接备机处理的有问题外，新请求正确负载到主机。

```

由此，使用简单的开源方法便可以相对简单的部署高可用的负载均衡系统，当然这里的练习测试没有涉及具体的业务，现实业务情况可能会更复杂。不过，由于nginx本身的良好设计，在nginx上进行模块的开发，不失为一种高效的方法。
