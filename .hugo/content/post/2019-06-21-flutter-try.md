---
title: "Flutter -- 写app的另一种姿态"
date: 2019-06-21T17:09:02+08:00
draft: false
categories: [app] 
tags: [Flutter]
---
# 简介
[Flutter](https://flutter.dev/)，如果不是专门去搜索，很少开发者会留意这个ui框架。即使搜索到，一了解到使用的是一种听都没听过的全新编程语言--[dart](https://dart.dev/)，也会望而却步。即使接受了[dart](https://dart.dev/)这种编程语言，一看代码的无止境的括号也会恶心到吐血。以上就是[Flutter](https://flutter.dev/)的初印象，小众(生态小，第三方库少，出问题找答案困难)， [dart](https://dart.dev/)编程语言，无限括号。

[Flutter](https://flutter.dev/)，是google的推出的跨平台的移动框架，但是不是亲儿子也很难说，会不会像其他google项目一样难产也说不准，不过目前google的态度是积极的，而且已经陆续有一些公司慢慢的使用或迁移到[Flutter](https://flutter.dev/)，加上苹果自己也推出类似的框架[SwiftUI](https://developer.apple.com/xcode/swiftui/)，再加之还是传言中的FuchsiaOS的标准开发框架，所以前景，个人觉得还是有所期待的。

说到移动框架，自然有原生/react-native/weex（小众，可忽略）等的对比，网上有这方面的文章，这里不累赘。就个人而言，我就是不满react-native存在的各种问题而寻找替代品而找到[Flutter](https://flutter.dev/)的。react-native存在一大堆说不清道不明的坑（包括各种库升级的，库本身的，兼容的等等），虽然生态大网上一般都能找到解决方法，但是依然不爽。而[Flutter](https://flutter.dev/)从学习的过程中相对少点（或许还没遇到）。[Flutter](https://flutter.dev/)与react-native的最大的不同是，react-native通过JS Bridge调用生成的是原生的UI界，而flutter通过自己的引擎绘画出来的，而不是调用生成原生的UI界，这个和以前做windows开发的时候directui原理是类似的。也由于这个原因，[Flutter](https://flutter.dev/)可以实现web端([Flutter for web](https://flutter.dev/web))，桌面端([flutter-desktop-embedding](https://github.com/google/flutter-desktop-embedding))的应用程序，google已经开始了这方面的工作，当然，有没有人用是另外一回事。

# Dart语言
一听到flutter使用的全新编程语言，很多人都会忘而却步。然而，当你有编程背景时，特别是多种不同语言背景时（比如，go，java/c#，es6等），你会惊奇的发现，[dart](https://dart.dev/)和这些语言如此的相似，不止写法相似，很多概念都可以在不同的语言中找到，比如mixin在go的结构体扩展上，Future在ES6的Promise等等。所以，从这个角度来说，[dart](https://dart.dev/)也不太像一种全新的编程语言，更像是吸收其他语言特性，混合在一起的语言。所以，对于有多种编程语言背景的开发者来说，[dart](https://dart.dev/)的确也是一种简单的编程语言，即使久而久之不写[dart](https://dart.dev/)程序，再回头依然有不陌生的感觉。

按照国际惯例，网上有很多优秀的文章参考，不在累赘语言的特征，直接看下几个入门的连接。[dart](https://dart.dev/)语言的入门参考：[language-tour](https://www.dartcn.com/guides/language/language-tour)。[dart](https://dart.dev/)语言的库的参考[library-tour](https://www.dartcn.com/guides/libraries/library-tour)。

# Flutter
这里不会详述[Flutter](https://flutter.dev/)的各个方面，因为也很难描述完整，所以这里描述大多是自己的感受。

在[Flutter](https://flutter.dev/)的世界里，几乎所有的东西都是widget，所谓的widget就是窗口小部件，而已在Flutter里面，`Widget`只是`Element`的一个配置，真正渲染的界面的是一个叫`RenderObject`的东西。做应用开发，其实不用了解到那么深入也没有什么大问题。总体来说，Flutter里面的`Widget`分为两大类一个种称之为`Stateless`的，另外一种是`Statefull`，顾名思义，就是无状态与有状态之分。Flutter里面的widget多如牛毛，那么对应普通开发者来说，是否是都有掌握所有的UI widget呢？非也，Flutter里面很多是重复的，比如说`Container`增加个`padding`属性的配置可以实现`Padding`的功能，`Row/Column`适当的配置也可以实现`Center`的功能，`GestureDetector`可以实现`InkWell`等等。感觉上是google用着用着觉得不爽了，就添加一个组件。但是常用的组件并不是很多，gayhub上面有两个不错的展示常用UI的仓库：[flutter-ui](https://github.com/efoxTeam/flutter-ui)和[flutter-go](https://github.com/alibaba/flutter-go)。虽然包括的组件不一定是齐全的，但整体上创建一个类似淘宝一样的app，这些组件应该是足够的了。那么对使用Flutter创建UI界面是否困难呢？如果你写过react-native或知道flex布局，再加上稍微了解一下flutter的基础组件，那么写起来并非困难，速度甚至比react-native还快，但是，很多的括号和嵌套会让你恶心到吐血，如果不是有IDE的支持和十分快的热加载速度，很难相信会有人选择Flutter进行开发。

做过react-native/react的都知道，跨页面数据共享时，需要全局状态管理工具，而redux几乎是react世界的标准方式。那么Flutter是否也有redux呢，答案是肯定的。然而在实际上使用时，redux在Flutter上面写起来并不顺手，总感觉生硬。在Flutter世界了状态管理大约有：EventBus, Bloc, Scoped_model, Redux(fish-redux), provider等。严格来说，EventBus和Bloc并不算是状态管理，EventBus更像它名字描述一样事件总线，对事件感兴趣的组件就监听，状态变化了就更新UI，而Bloc本质上就是一种设计模式，接口与实现分离的一种模式，数据变化通过StreamBuilder来重绘。Scoped_model, Redux(fish-redux), provider这些全局状态管理工具中，除了Redux(fish-redux)用起来没有像react那么爽外，其他两个都相对简单，网上搜索一下就有响应的文章出来，这里不再累赘。

两个练习的demo，PS.完整的的学习demo, flutter使用方式不完全正确的：[wang_finance](https://github.com/buf1024/monthproj/tree/master/wang_finance)和[hellodiary](https://github.com/buf1024/monthproj/tree/master/hellodiary)。

# 小结
总体而言，flutter的开发体验会比react-native好，虽然flutter的生态相对小，发展前景也不确定，但相比于react-native的各种坑，还是值得尝试。