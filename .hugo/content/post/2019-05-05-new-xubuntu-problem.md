---
title: "物理机新建xubuntu一些问题"
date: 2019-05-05T11:12:02+08:00
draft: false
categories: [Misc] 
tags: [Misc]
---
* 无线网卡驱动问题 -- 无法显示wifi

  解决方法: [https://jingyan.baidu.com/article/ca2d939dd4f1b4eb6c31ce09.html](https://jingyan.baidu.com/article/ca2d939dd4f1b4eb6c31ce09.html)

  PS： 下载需要联网，可以使用手机usb共享网络。
* 新建用户密码过于简单

  ```markdown
  # 去掉Cracklib password strength checking 
  sudo pam-auth-update
  ```

* bash控制台不变颜色，删除~/.bash_profile文件或在里面. .bashrc
* GRUB多系统启动顺序修改

  ```markdown
  cd /etc/default/grub.d
  # 修改启动顺序，可能需要记得启动界面顺序修改
  sudo vi grub
  # 生效
  sudo update-grub
  ```

* 修改系统生成的目录的名称

  ```markdown
  # 直接看修改
  vim .config/user-dirs.dirs
  ```

* 增加systemd启动服务

  ```markdown
  sudo cp frpc.service /lib/systemd/system/sudo 
  # 开机启动
  sudo systemctl enable frpc.service
  ```

* 设置Python

  ```markdown
  sudo update-alternatives --install /usr/bin/Python Python /usr/bin/Python3 150
  sudo update-alternatives --install /usr/bin/pip pip /usr/bin/pip3 150

  # 修改安装源(不存在则新建)
  vim .pip/pip.conf
  [global] 
  index-url=http://mirrors.aliyun.com/pypi/simple 
  [install] 
  trusted-host=mirrors.aliyun.com
  ```

* 修改docker相关配置

  ```markdown
  # 修改下载源（不存在则新建）
  sudo vim /etc/docker/daemon.json
  {
      "registry-mirrors": [
          "https://2gpinm2f.mirror.aliyuncs.com"
      ]
  }

  # docker ps报权限问题
  # 增加用户组
  sudo usermod -aG docker able
  sudo groupadd docker
  sudo gpasswd -a able docker
  newgrp docker

  # docker 使用volume权限问题
  chmod a+w xxx.dir
  ```

* unlimit无权限

  ```markdown
  # 修改pam， 末尾增加
  sudo /etc/pam.d/common-session
  session required pam_limits.so

  # 修改系统配置, 末尾增加，重启或sysctl -p生效
  sudo vim /etc/sysctl.conf
  fs.file-max=1024000000
  fs.nr_open=1024000000

  # 末尾增加, 单加这个，此方式不生效，需要加
  sudo vim /etc/security/limits.conf
  able              soft    nofile 1024000000
  able              hard    nofile 1024000000
  able              soft    nproc  1024000000 
  able              hard    nproc  1024000000
  ```

* root 修改可接受简单密码
  一般的错误提示为：

> Resolve and Remove “BAD PASSWORD: it is based on a dictionary word “in Linux
>
> If you know what you’re doing and want to use weak password in Linux (CentOS, RedHat … etc), follow the steps below.
>
> 1. vi /etc/pam.d/system-auth as root.
> 2. Look for the following two lines:
>
> password    requisite     pam_cracklib.so try_first_pass retry=3 password    sufficient    pam_unix.so md5 shadow nullok try_first_pass use_authtok

按提示打开文件：

```shell
1. password    requisite     pam_cracklib.so try_first_pass retry=3
2. password    sufficient    pam_unix.so md5 shadow nullok try_first_pass use_authtok
3. Comment out the first of the two lines:   注释掉此行

#password    requisite     pam_cracklib.so try_first_pass retry=3

4. Remove use_authtok on the second line. Otherwise you’ll get “passwd: Authentication infORMation cannot be recovered” error.

删除掉 "use_authtok “从第二行，否则你会得到"passwd : Authentication infORMation canot be recovered "的错误提示

password    sufficient    pam_unix.so md5 shadow nullok try_first_pass

5. That’s it. Try changing your password again.
```

 搞定，再次尝试修改密码（）
