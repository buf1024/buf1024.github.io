---
title: "Flutter Trouble shoot"
date: 2019-06-01T19:13:02+08:00
draft: false
categories: [flutter] 
tags: [flutter]
---

## Flutter升级无法翻墙

```
升级没法fq的，运行报错的，可以做以下操作将sdk中
flutter/packages/flutter_tools/gradle/resolve_dependencies.gradle
flutter/packages/flutter_tools/gradle/aar_init_script.gradle
flutter/packages/flutter_tools/gradle/flutter.gradle
3个文件中的 https://storage.googleapis.com/download.flutter.io 
替换为：
http://download.flutter.io
```

## Flutter upgrade 经常失败或超时(最后重新用git协议重新下载)

```
1. git remote -v 检查git 仓库是否是https协议，如果不是修改为git协议

2. git remote rm orgin
   git remote add origin git@github.com:flutter/flutter.git
   git checkout master
   git pull orign master
   
3. flutter channnel stable

4. flutter upgrade -v
```

## flutter build apk 包错

```
/Users/luoguochun/.gradle/caches/transforms-1/files-1.1/support-compat-28.0.0.aar/600bd9322c1a43475e
7c0ab512969467/res/values/values.xml:133:5-70: AAPT: error: resource
android:attr/fontVariationSettings not found.

# 解决
修改： android/app/build.gradle
compileSdkVersion 28

# 参考:
https://stackoverflow.com/questions/49208772/error-resource-androidattr-fontvariationsettings-not-found
```

## 库延迟载入

```
1. 使用defered as 导入
2. 使用 async loadLibrary.
如：
import 'mylib.dart' defered as lazylib

void main() {
    loadLazy()
}

void loadLazy() async {
    await lazylib.loadLibary()
  
    # lazylib.doxxx
}
```