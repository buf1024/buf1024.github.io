---
title: "Flutter 桌面端多窗口支持"
date: 2023-07-19T18:00:10+08:00
draft: false
categories: [flutter] 
tags: [Dart, flutter]
---

### 多窗口

Flutter的已经支持桌面端开发已经一段时间了，不过关注桌面端开发的应该也知道，flutter的桌面端并不支持多窗口。虽然官方表示以后会支持，但，Flutter作为一个一开始就定位为移动端的跨平台开发框架，Flutter多窗口的支持的优先级一直排在很低很低。所以什么时候真正得到官方的支持，还不知道等到猴年马月。因为桌面端的开发解决方案很多，而且基于web方案，如Electron等框架的流行起来，似乎采用Flutter作为解决方案的实际应用并不多，所以社区的解决方案也不多。能用，并且有实际的程序的似乎就只有一个: [desktop_multi_window](https://github.com/MixinNetwork/flutter-plugins/tree/main/packages/desktop_multi_window "desktop_multi_window")。

对于[desktop_multi_window](https://github.com/MixinNetwork/flutter-plugins/tree/main/packages/desktop_multi_window "desktop_multi_window")，他能提供的功能并不多，有些实际上开发所需要的功能（比如，窗口resize事件，失去焦点时间等等），它并没有提供。不过，社区对于上述缺失去的功能，其他库有提供，比如：[window_manager](https://github.com/leanflutter/window_manager), [bitsdojo_window](https://github.com/bitsdojo/bitsdojo_window "bitsdojo_window")等，这单窗口的库相对还是多一点点。因此，就有些项目将此类库合并一起，作为自己项目使用，比如[restdesk](https://github.com/rustdesk/rustdesk)的[restdesk_desktop_multi_window](https://github.com/Kingtous/rustdesk_desktop_multi_window)。[restdesk_desktop_multi_window](https://github.com/Kingtous/rustdesk_desktop_multi_window)，是专门为自己项目定制的，而且更新也是基于自己项目，所以，不具备通用性。真正使用时还是结合使用[desktop_multi_window](https://github.com/MixinNetwork/flutter-plugins/tree/main/packages/desktop_multi_window "desktop_multi_window")和[window_manager](https://github.com/leanflutter/window_manager), [bitsdojo_window](https://github.com/bitsdojo/bitsdojo_window "bitsdojo_window")等。相对来说[desktop_multi_window](https://github.com/MixinNetwork/flutter-plugins/tree/main/packages/desktop_multi_window "desktop_multi_window")，也是为了自己项目而剥离出来的，似乎也不为了推广，它文档什么的几乎为零。当然，使用这个东西也不复杂，看看他提供的例子便知道如何上手。不过，也因为没有文档，所包含的坑也必须一个一个的踩过。

### 坑

一个正常的项目，肯定有依赖很多其他库的。[desktop_multi_window](https://github.com/MixinNetwork/flutter-plugins/tree/main/packages/desktop_multi_window "desktop_multi_window")的第一个坑就是`MissingPluginException`​。这个异常是在创建子窗口的时候抛出来的，创建子窗口的时候`main`​函数会再调用一次，子窗口和主窗口的不同在意，创建主窗口时传过来的参数是空的，而子窗口会传三个参数过来，分别是第一个固定值`multi_window`​，第二个窗口id，已经传给子窗口的和应用相关的参数。根据此来区分主子窗口以及业务功能。不过在子窗口时，如果不把子窗口所依赖的`Plugin`​注册，那么就会抛出`MissingPluginException`​，这个修改简单。以macos为例，所修改的代码如下：

```swift
// macos/Runner/MainFlutterWindow.swift
import Cocoa
import FlutterMacOS
+ import desktop_multi_window
+ import screen_retriever
+ import window_manager
+ import hotkey_manager

class MainFlutterWindow: NSWindow {
  override func awakeFromNib() {
    let flutterViewController = FlutterViewController()
    let windowFrame = self.frame
    self.contentViewController = flutterViewController
    self.setFrame(windowFrame, display: true)

    RegisterGeneratedPlugins(registry: flutterViewController)

+      FlutterMultiWindowPlugin.setOnWindowCreatedCallback { controller in
+      // Register the plugin which you want access from other isolate.
+    
+
+       WindowManagerPlugin.register(with: controller.registrar(forPlugin: "WindowManagerPlugin"))
+       HotkeyManagerPlugin.register(with: controller.registrar(forPlugin: "HotkeyManagerPlugin"))
+       ScreenRetrieverPlugin.register(with: controller.registrar(forPlugin: "ScreenRetrieverPlugin"))
+    }

    super.awakeFromNib()
  }
}

```

这个注册，不单单是依赖的插件的`Plugin`​，还有所依赖的`Plugin`​依赖的`Plugin`​, 比如`window_manager`​这个，它还依赖`screen_retriever`​，你必须把两个都注册进来。这就比较烦人了，你怎么知道你导入的库，还依赖哪个插件，所依赖的那个插件由叫什么名字呢？所以，必须一个一个的试等到异常抛出来之后，去网上找到对应的库，找到代码中的名称，才能确定，然后才能添加到代码之中。

踩过第一坑之后，马上便迎来第二个坑。在我设置场景中，主窗口丢失焦点之后，就进行关闭，由系统托盘管理，关闭子窗口的时候，子窗口将完全关闭，还有一个功能，就是窗口关闭之后，点docker程序是恢复到主窗口的（系统托盘了，不应该还有docker window的，不过这是测试）。实现docker恢复，大体增加的代码:

```swift
// macos/Runner/AppDelegate.swift
import Cocoa
import FlutterMacOS

@NSApplicationMain
class AppDelegate: FlutterAppDelegate {
  override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
 -   return true
 +   return false
  }
 +override func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
 +   if !flag {
 +     for window in NSApp.windows {
 +       // if window.parent == nil {
 +       if !window.isVisible {
 +         window.setIsVisible(true)
 +       }
 +       window.makeKeyAndOrderFront(self)
 +       NSApp.activate(ignoringOtherApps: true)
 +       // break
 +       // }
 +     }
 +   }
 +   return true
 + }
}
```

场景示意：

​![image](/img/flutter/hiqspotlight.png)​

由代码恢复看，是把所有非可见的窗口，全部重新显示出来，包括所有的主窗口和子窗口。当然也可以像，代码注释里的那样，只显示主窗口，不过那窗口一直都在那里，根本就没有关闭。

所以解决的方式也简单，就是调用关闭窗口时，完全关闭窗口即可。则[window_manager](https://github.com/leanflutter/window_manager)里面，关闭的函数:

```swift
// lib/macos/Classes/WindowManager.swift
  public func close() {
        mainWindow.perfORMClose(nil)
    }
```

而`perfORMClose`​在苹果文档里面只是模拟关闭的这按钮而已，所以肯定是关闭不了的了。查看所有[window_manager](https://github.com/leanflutter/window_manager)的函数，并没有一个可以关闭的函数了。不过好在苹果的文档够详细，而且有了样板代码在那里，即使不会swift语言，不会写Flutter Plugin，往里面添加功能还是很容易的。完全关闭，也很简单（当然，使之能够被flutter调用，还有添加其他代码，此处忽略）:

```swift
    public func forceClose() {
        mainWindow.isReleasedWhenClosed = true
        mainWindow.close()
    }
```

解决了上述坑之外，就以为没坑了？非也，最大的坑还在后面，[desktop_multi_window](https://github.com/MixinNetwork/flutter-plugins/tree/main/packages/desktop_multi_window "desktop_multi_window")所创建的字窗口，和其他窗口是完全独立的，几乎相当于新起了一个程序，所以，应用程序的状态并不能共享，更加不会得到更新。所以它用自己独特的消息传递方式，用`DesktopMultiWindow.setMethodHandler`​监听消息，用`DesktopMultiWindow.invokeMethod`​发送消息，把名字写得像远程调用似的，不过也像是远程调用。而这种通讯方式，最大的弊端就是，对代码有极高的入侵性，你必须要按照这种模式来思考和构建你的程序。如果Flutter真的开始支持了多窗口，那么程序也必须重写一次，不过好在Flutter界面开发的高效率，重构所花费的时间感觉也是可以接受。

### 小结

你以为上面就上面一点坑？感觉解决起来也不难啊！哈，上面只是罗列一小点点而已，Flutter桌面开发的坑是一步一个坑，两步三个坑，三步五个坑，坑坑不断啊。不过相对于Electron而言，Flutter桌面的运行效率高不少，打包的尺寸也少一半以上，开发效率更是杠杆的，在坑与效率之间均衡其实也是可以选择的。当然，Flutter的劣势就是人员的缺乏，缺乏人才储备的企业使用的确要慎重，个人或者有人才的则大胆用之。

‍
