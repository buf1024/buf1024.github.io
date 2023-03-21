---
title: "android https 抓包"
date: 2023-03-21T08:10:12+08:00
draft: false
categories: [flutter, 量化] 
tags: [android, 量化]
---
前几天有个需求，要破解一下某个app的部分通讯协议，需要了解一下某部分功能如何实现，甚至通过其协议抓取一些数据。而这个app只有android版本和ios版本。从初步看，这个app是同React Native写的。

虽然https是加密的，不过查看https本来感觉这并不是什么太复杂的事情，引入一个中间人就可以了。虽然在浏览器会弹出CA不可信任的警告，不过对于app来说，如果不作特殊处理（一般人也不会特殊处理），一般没什么影响。而这事在很久之前也是做过的了。没想到现在的android却提高了难度。记录一下步骤，以备后续可能需要查看。

### 普通 https 抓包

在mac下面最常用的是charles，其他平台有类似的软件。 步骤如下：

- charles打开代理

    如图示，如此经过操作后，经过此代理的http/https报文即可被捕获（中间人）。

    代理设置:

    ![代理设置](/img/charles/step0-1.png)

    SSL设置:

    ![SSL设置](/img/charles/step0-0.png)

    打开代理:

    ![打开代理](/img/charles/step0-2.png)

- 配置服务端客户端证书

    经过上述配置，虽然可以实现了中间人捕获报文，可是https的内容是无法查看的，所以需要配置证书后才可以解密。步骤如图示：

  - 电脑端：

        添加证书：

        ![添加信任证书](/img/charles/step1-1.png)

        设置信任证书：

        ![设置信任](/img/charles/step1-2.png)

  - 手机端：

        设置代理：

        ![设置代理](/img/charles/step2-1.png)

        添加信任的证书：

        ![添加信任的证书](/img/charles/step2-2.png)

以上便是网上能搜索到的方式，也是以前自己抓取https报文的步骤，如果一切那么顺利便不会有这篇记录。因为在从android7.0开始后，android手机即使你添加信任的证书后，系统依然是不信任你的证书的，只信任系统的证书。所以，任何请求都不会使用到这个用户证书，也因此，https的报文依然是无法解密的。

### android7以上https抓包

针对android7.0以上系统，这个办法的解决方式也是挺粗暴的，既然只信任系统证书，那么就移动真是到系统目录不就可以了？而此方式必须要系统root方可以。这是不愿意做的，除非可以使用一个已经root的模拟器，不过这个好像比较难找。

除此之外，网上还有一种方式，那就是自己编码让其信任用户证书。具体的步骤也挺简单的，如下：

- 新建network_security_conf.xml文件
  
    放在res/xml下面，此文件意思就告诉系统，让系统信任用户证书，内容大体如下：

  ```xml
  <?xml version="1.0" encoding="utf-8"?>

<network-security-config>
    <trust-anchors>
        <!-- 信任系统预装 CA 证书 -->
        <certificates src="system" />
        <!-- 信任用户添加的 CA 证书，Charles 和 Fiddler 抓包工具安装的证书属于此类 -->
        <certificates src="user" />
    </trust-anchors>
</network-security-config>
  ```

- AndroidManifest.xml修改

    与此同时在AndroidManifest.xml文件的application字段后增加`android:usesCleartextTraffic="true" android:networkSecurityConfig="@xml/network_security_config"`内容。

然而，这个app并非自己所写的，本来就没有源码，所以必须反编译后修改，再重新签名。这个方式网上也有比较详细的方式，大体涉及三个工具：


1. apktool，编译和反编译apk，从apk中提取图片和布局资源

2. dex2jar，将可运行文件classes.dex反编译为jar源码文件

3. jd-gui，查看jar源码文件

网上已经有比较详细的方式，这里不再累赘。而现实修改的只涉及资源文件，所以，只用到apktool即可，而且只需要两个命令。

解包：`apktool d -o dec xx.apk`  重新打包：`apktool b -o yy.apk dec/`

重新打包后的apk需要经过重新签名之后，方可安装，步骤：
```shell
#1 使用keytool生成密钥，如
keytool -genkeypair -v -keystore hello.keystore -alias hello -keyalg RSA \
          -keysize 2048 -validity 10000

#2 重新打包
apktool b -o yy.apk dec/

#3 重新签名，使用android sdk tool自带的工具
$ANDROID_HOME/build-tools/30.0.3/apksigner sign --ks hello.keystore --ks-key-alias hello yy.apk
```

apktool重新打包时，有一个坑。在AndroidManifest.xml文件里，如果`android:extractNativeLibs="false"`，则需要修改为`android:extractNativeLibs="true"`，否则按装时会报错。

按照上述修改后，本以为就可以了，可事实上，压根就没有解决https无法查看的问题。这就如同说TCP挥手必须要四次才行的那些背八股文的人一样，根本就没有经过实践，把不完整的知识当是真理。当然，因为这是修改别的软件，不知道自己编写的软件这么修改后，有没有效果。

经过一番瞎折腾和摸索，才找到网上基本没有说过的修改方式（重点）：

既然你是android 7.0以下才可以用，那么我就把你运行目标修改为这个目标，是不是可以？修改点：

```
#1 AndroidManifest.xml 修改为
platformBuildVersionCode="23"

#2 apktool.yml修改为
sdkInfo:
  minSdkVersion: '23'
  targetSdkVersion: '23'
```

然后，然后，结果居然行了！！！

![https-crack](/img/charles/https_crack.png)

### 小小结

1. 原来apk反编译重新打包如此简单（修改源码例外，特别是混淆之后的）
2. android 7后http是抓包变得繁杂
3. 网上的答案很多的复制黏贴，不一定正确，自己实践过的才是可靠的，然而并不是所有人实践过，万一他们一直认为他们没实践过的是真理……
