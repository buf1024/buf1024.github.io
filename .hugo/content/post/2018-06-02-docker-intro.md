---
title: "Docker -- 简介"
date: 2018-06-02T11:40:39+08:00
draft: true
---

## 背景
docker从几年前开始就是开源社区一颗耀眼的明星，自己也在那个时候接触docker，也是因为docker学习了golang（golang是一门好语言，用go实现了某一后台服务，偷偷在开发机器换了：~），后来因为没有机会使用，也就渐渐忘记了。现在据说，可能要使用到docker，正好复习一下，记录一下复习笔记（不一定完全正确）。

每一种技术的流行，必有它解决问题的背景。回想下我们软件开发，应用的发布是按照程序的级别分发的，而应用可能依赖各种各样的库，那在开发，测试，uat，生产多种环境中，都必须重新安装一次而且祈祷不要和其他应用的依赖冲突，而且有时候生产环境的部署有可能不是开发人员进行操作的，所以运维人员又必须一次又一次打扰你。在这过程中，步骤比较繁琐而且容易出错，这就是docker解决的第一个痛点：docker是按照容器级别分发的，docker像集装箱一样包括了应用所需要的几乎所有东西，简化统一了开发，测试，运维的步骤。为了隔离应用，限制资源分配等，我们都习惯在生产是使用虚拟化技术，低级一点的就使用vmware，稍微高级一点的就使用kvm、xen等这些半虚拟方式，然而这些，对资源的消耗都是非常大。这就是docker解决的第二个痛点。

很多人以为docker是轻量级的虚拟机，但docker并非是虚拟机，虽然它提供类似于虚拟机的功能，本质上它是主机上的应用程序。

## 使用
docker的安装略过，直接参考官网，安装配置完毕后，启动docker服务。`docker --help`可以看到所有的选项，对应某个命令也可以查看帮助，如`docker image --help`。  

docker里面有三个重要的概念: 镜像，容器和仓库。镜像和容器的关系就像是类和实例，程序和经常的关系，仓库是存放镜像的地方，官方的仓库一般很慢很慢，通常安装docker后需要设置(和maven，npm一样，大都是墙的原因)，如：
```shell
able@ABLE-VM:~/tmp$ cat /etc/docker/daemon.json 
{
    "registry-mirrors": ["http://ef017c13.m.daocloud.io"]
}
```
也可以`pull`的时候指定地址，当然你也可以像maven一样建立私服。

#### 镜像
镜像对于开发人员来说是比较重要的一个概念，通常需要指明怎么构建镜像，而使用`Dockerfile`来构建。

镜像操作的命令(不完全，够用)：
``` shell
docker pull          # 从仓库拉取镜像
docker push          # 推送镜像到仓库
docker image         # 大部分镜像操作命令
docker commit        # 为运行中容器提交进行镜像，不用次命令构建镜像
docker build         # 使用Dockerfile构建镜像
```
`Dockerfile`是构建镜像的最主要方式，指令并发很多，作为开发人员需要掌握的，具体指令参考官方文档，这些需要在实践中才能记住。

#### 容器
容器是镜像的实例，容器是非常轻量，通常制作镜像时，需要以非后台的模式启动镜像里面的应用，以后台模式启动应用，容器起来后自动退出, 而且容器应该像普通进程一样，退出就没有了。常用的两个操作命令：
```shell
# 启动docker
able@ABLE-VM:~/tmp$ docker run -d --name my-nginx --rm -p 8081:80 nginx
0ed5ce3580067d301ebca05af55b0fced69c841f8cccbe1e976ab01a4f09a5eb
#进入运行中的docker
able@ABLE-VM:~/tmp$ docker exec -it nginx bash
root@17d67ddec2e0:/# 
root@17d67ddec2e0:/# exit
exit
# ps 运行中的docker
able@ABLE-VM:~/tmp$ docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                  NAMES
0ed5ce358006        nginx               "nginx -g 'daemon of…"   47 seconds ago      Up 46 seconds       0.0.0.0:8081->80/tcp   my-nginx
17d67ddec2e0        nginx:v3            "nginx -g 'daemon of…"   2 hours ago         Up 2 hours          0.0.0.0:8080->80/tcp   nginx
able@ABLE-VM:~/tmp$ 
```
`docker container`

## 原理

## docker周边

## docker生成应用

## 容器云


