---
title: "Flutter 一丢丢代码片段"
date: 2019-07-05T16:12:10+08:00
draft: false
categories: [flutter] 
tags: [flutter]
---
### Flutter路由

```dart
# 方式一，手动调用Navigator方法:
# 或 Navigator.of(context).push ...
Navigator.push(context,
    MaterialPageRoute(builder: (BuildContext context) {
        # 对应的页面参数这里这里传
        return PasswordPage(cb: _passChanged,);
    })
);
# 方式默认，用Navigator.pushNamed方法:
# 通过onGenerateRoute, onUnknownRoute,navigatorObservers可以监听路边变化
MaterialApp(
    routes:{
        "new_page":(context) => NewRoute(),
        ... // 省略其它路由注册信息
    },
    initialRoute: '/'
    onGenerateRoute:(RouteSettings settings){
      return MaterialPageRoute(builder: (context){
           String routeName = settings.name;
       // 如果访问的路由页需要登录，但当前未登录，则直接返回登录页路由，
       // 引导用户登录；其它情况则正常打开路由。
     }
   ),
    onUnknownRoute: xx
    navigatorObservers = xxx,
);

# 通过Navigator.pushNamed路由参数值传值
Navigator.of(context).pushNamed("new_page", arguments: "hi");
...
  @override
  Widget build(BuildContext context) {
    //获取路由参数  
    var args=ModalRoute.of(context).settings.arguments;
    //...省略无关代码
  }


```

### app表头透明(无缝连接)

```dart
import 'dart:io' show Platform;
...
void main() {
    ...
    if (Platform.isAndroid) {
    SystemChrome.setSystemUIOverlayStyle(
        SystemUiOverlayStyle(statusBarColor: Colors.transparent));
    }
}
// 或详细(可在main, 也可在顶层widget的build方法<优先>里)

    SystemChrome.setSystemUIOverlayStyle(SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      statusBarIconBrightness: Brightness.dark,
      statusBarBrightness: Platform.isAndroid ? Brightness.dark : Brightness.light,
      systemNavigationBarColor: Colors.white,
      systemNavigationBarDividerColor: Colors.grey,
      systemNavigationBarIconBrightness: Brightness.dark,
    ));
```

### app限制旋转(限定显示方向)

```dart
void main() async {
  // 确定widget初始化
  WidgetsFlutterBinding.ensureInitialized();
  await SystemChrome.setPreferredOrientations(<DeviceOrientation>[DeviceOrientation.portraitUp, DeviceOrientation.portraitDown])
      .then((_) => runApp(MyApp()));
}
```

### 监听app暂停，恢复等状态

```dart
import 'package:flutter/services.dart';
...
# 在构造函数或initState里面
    SystemChannels.lifecycle.setMessageHandler((message) {
      debugPrint('lifecycle message: $message');
      if (message == AppLifecycleState.resumed.toString()) {
        ...
      }
    });
```

### 禁止按返回键时返回

```dart
# 使用WillPopScope widget
@override
  Widget build(BuildContext context) {
    return WillPopScope(
      onWillPop: () async {
        ...
        return true;
      },
      child: Scaffold(
        ...
      )
    );
  }
```

### 检查文本行数

```dart
final painter = TextPainter(
    text: TextSpan(text: text, style: style),
    maxLines: maxLines,
    textDirection: TextDirection.ltr);
painter.layout(maxWidth: size.maxWidth);
if(painter.didExceedMaxLines)
    return Text(text, style: style);
// ...
```

### 使用空的Container

```dart
// 为了使Container不覆盖父Container的空间，可使用Align组件包装
Container(
      width: 200,
      height: 200,
      color: Colors.purple,
      child: Align(
        alignment: Alignment.bottomCenter,
        child: IntrinsicHeight(
          child: Container(
            height: 150,
          ),
        ),
      ),
    )
```

### ScrollNotification/NotificationListener可替换ScrollController

```dart
ScrollNotification and NotificationListener, which can be used to watch the scroll position without using a ScrollController.
```

### 获取widget位置

```dart
增加GlobalKey, 增加回调绑定

WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      RenderBox box = _globalKey.currentContext.findRenderObject();
      p2 = box.localToGlobal(Offset.zero);

      print('p2:$p2');
    })
    
因为GlobalKey操作很昂贵，所以，在可以确定初始化的地方，
使用Builder,取得BuildContext后，直接获取。
    
```