---
title: "Macos一丢丢小技巧"
date: 2018-07-06T11:02:16+08:00
draft: false
categories: [mac] 
tags: [mac]
---

有的人认为用mac是纯粹了了装X，不过用过的人自然才知道什么是无知。

摘录一些常见的问题和使用用的技巧。

### Mac cpu占用高
Mac系统中 mdworker、mds、mds_stores进程占领CPU

新买的电脑，打开后发现变得很烫，CPU占用很高

这几个进程都是为了Spotlight索引而疯狂的，解决方案：

`sudo mdutil -a -i off`

打开

`sudo mdutil -a -i on`

### Mac Homebrew 更新慢
其实修改为Git协议就好了，gfw不封git协议，https协议会被封

---

用清华的源就非常好，去清华镜像的官网看一下说明，

[https://mirrors.tuna.tsinghua.edu.cn/help/homebrew/](https://links.jianshu.com/go?to=https%3A%2F%2Fmirrors.tuna.tsinghua.edu.cn%2Fhelp%2Fhomebrew%2F)

```rust
$ cd /usr/local/Homebrew
$ git remote set-url origin https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git

$ cd /usr/local/Homebrew/Library/Taps/homebrew/homebrew-core
$ git remote set-url origin https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git

$ brew update

# 还原:
$ cd "$(brew --repo)"
$ git remote set-url origin https://github.com/Homebrew/brew.git

$ cd "$(brew --repo)/Library/Taps/homebrew/homebrew-core"
$ git remote set-url origin https://github.com/Homebrew/homebrew-core

$ brew update
# "$(brew --repo)" 这里填/usr/local/Homebrew
```

### mac update文件删除
自动更新下载的问题，好就几个G，删除是个问题：

1. Restart your Mac in recovery mode(press and hold Command (⌘)-R at the start-up).
2. Open the Terminal Utility.
3. Type the command csrutil disable (This gives full unrestricted access to your Mac's entire OS and every file and folder, so, be cautious!)
4. After restarting, open the Terminal and delete the desired folders by running the command:
```
sudo rm -R /Library/Updates/0##-#####
```
Now the files or folders which were restricted can be removed.

NOTE: (0##-##### is the folder name)

To delete specific file inside the folder 0##-##### you can simply use rm provided the path of that file.

5. When done with all the desired removing, follow steps 1 and 2 to turn SIP back on using the command:
```
csrutil enable
```
6. Restart your Mac and SIP should be back on track.

NOTE: To check the status of the SIP. Use command csrutil status

7. Download and Install fresh updates if available.

### mac 设置ulimit
这个有别于linux，如果不设置，导致运行docker时，某些程序会挂。

```shell
$ sudo launchctl limit maxfiles 100000 500000

$ sudo launchctl limit maxproc 100000 100000

 $ launchctl limit
```

新建文件/etc/sysctl.conf写入

```shell
kern.maxfiles=1024000000

kern.maxfilesperproc=1024000000
```

设置好之后不能再设置大

---

以上方法适应于bash运行的程序

### Mac的截屏
Mac的截屏保存文件至桌面的快捷键分别是 command + shift + 3 （将屏幕捕捉到文件）和 command + shift + 4 （将所选内容捕捉到文件）。以前我都是用鼠标去框窗口，这样当然可以，但是手工选定捕捉到的窗口图片尺寸可能会不标准。昨天才发现Mac还有给固定窗口截屏的快捷键：

command + shift + 4 之后把鼠标指针放在要截取的窗口上按空格。


### 键盘快捷键
至于常用的快捷键，参考官网[Mac 键盘快捷键](https://support.apple.com/zh-cn/HT201236)