---
title: fabric极简简介
date: 2021-01-15T19:12:00+08:00
draft: false
categories: [Python] 
tags: [Python]
---

[fabric](fabfile.org)主要用于自动化运维，可以使用类似于`make`命令一样执行Python的函数。

[fabric](fabfile.org)主要依赖于[pyinvoke](http://pyinvoke.org/)和[paramiko](http://paramiko.org/)两个库，前者也是类似于make命令一样的工具，后者是处理ssh远程的库。

[pyinvoke](http://pyinvoke.org/)和[fabric](fabfile.org)的主要区别是，[fabric](fabfile.org)是建立在[pyinvoke](http://pyinvoke.org/)和[paramiko](http://paramiko.org/)之上，提供远程操作的功能。

所以安装[fabric](fabfile.org)后，其依赖库[pyinvoke](http://pyinvoke.org/)和[paramiko](http://paramiko.org/)也会一起安装。网上搜索到的中文教程是旧版本的，已经被淘汰了。

#### pyinvoke最简介

项目目录建立`task.py`,内容如下:

```Python
from invoke import task

@task
def build(ctx):
    print('Building...')
```

运行方式:

```sh
^_^@~]$ inv build
Building...
```

参数`ctx`里面可运行命令，命令分组，读配置(`ctx.config`获得)等，配置文件可位于项目当前目录，用户目录，系统目录等，文件名`invoke.yml`或`.invoke.yml`格式不限，不过推荐`yml`格式。`task`函数里面可有参数，依赖关系等。

### fabric最简介

fabric和invoke类似，是invoke的远程版本。`tasks.py`变成`fabfile.py`, 配置文件名， `invoke.yml`改为`fabric.yml`。运行的命令由`invoke`改为`fab`，既然如此，那么用invoke替代fabric是否可以，答案是可以。但是，fab客户端提供更多的和网络相关的功能，省去很多手写的样板代码。

```Python
from fabric import task
import os

@task
def compile(ctx):
    ctx.run("mvn compile", env=os.environ)

```

输出:

```sh
^_^@~]$ fab compile
[INFO] Scanning for projects...
[INFO] ------------------------------------------------------------------------
[INFO] BUILD FAILURE
[INFO] ------------------------------------------------------------------------
[INFO] Total time: 0.138 s
[INFO] Finished at: 2020-06-29T17:07:46+08:00
[INFO] Final Memory: 6M/123M
[INFO] ------------------------------------------------------------------------
[ERROR] The goal you specified requires a project to execute but there is no POM in this directory (/Users/luoguochun/tmp/fabric). Please verify you invoked Maven from the correct directory. -> [Help 1]
[ERROR] 
[ERROR] To see the full stack trace of the errors, re-run Maven with the -e switch.
[ERROR] Re-run Maven using the -X switch to enable full debug logging.
[ERROR] 
[ERROR] For more infORMation about the errors and possible solutions, please read the following articles:
[ERROR] [Help 1] http://cwiki.apache.org/confluence/display/MAVEN/MissingProjectException

```

### 总结

没细到fabric的细节，非常粗暴的介绍，细节用时，需要参考官网。不过好像，不用fabric也可以完成那么一回事，那么用fabric/invoke做系统管理运维的好处是啥？最大的好处估计是，用统一的方式去处理一件事，提供类似于make的方式。
