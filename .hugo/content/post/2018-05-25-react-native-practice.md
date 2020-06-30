---
title: react-native 实践部分
date: 2018-05-25 17:04:10
aliases: [/2018/05/25/react-native-practive/]
categories: [reactjs]
tags: [react-native]
---
这两月，零零散散的折腾**react-native**, 使用**react-native**将公司的h5微盘前端转化为原生的app。从零开始搭框架到app主要功能的完成，山寨得有95%以上的相似度。总体来说，**react-native**相对而言还是比较简单的。facebook强调的是，**learn once, write everywhere**, 所以熟悉react的话，那么**react-native**就不会遇到任何问题，当然是指react。甚至相对而言，**react-native**有限的界面元素和flex布局系统比复杂的浏览器更加简单。问题当然也是遇到不少的，然而因为**react-native**不是发布了这边久，所以几乎所有的问题别人都可以能找到，都可以在网上找到相应的解决方案，即使一时难以找到解决方案，通过设计的技巧也是可以完成对应的需求。

开发使用到的开发依赖以及依赖库，如下：
```json
  "dependencies": {
    "axios": "^0.18.0",
    "md5": "^2.2.1",
    "native-echarts": "^0.4.0",
    "react": "16.3.1",
    "react-native": "0.55.2",
    "react-native-linear-gradient": "^2.4.0",
    "react-native-network-info": "^3.2.2",
    "react-native-orientation": "^3.1.3",
    "react-native-root-toast": "^3.0.1",
    "react-native-splash-screen": "^3.0.6",
    "react-native-swiper": "^1.5.13",
    "react-navigation": "^1.5.11",
    "react-redux": "^5.0.7",
    "redux": "^3.7.2",
    "redux-saga": "^0.16.0",
    "url": "^0.11.0"
  },  
  "devDependencies": {
    "babel-jest": "22.4.3",
    "babel-plugin-transform-remove-console": "^6.9.2",
    "babel-preset-react-native": "4.0.0",
    "jest": "22.4.3",
    "react-test-renderer": "16.3.1",
    "redux-devtools-extension": "^2.13.2",
    "remote-redux-devtools": "^0.5.12"
  }
```
是的，主要的技术栈: react-native + redux + redux-saga + react-navigation。因为是山寨，h5页面(页面是在服务端渲染--java,jsp)原本是有的，本以为可以直接使用页面的css排版，后来发现，h5的css属性写得过于复杂，反而**react-native**默认的flex布局加上牛掰的IDE提示更加简单写起来更顺畅。所以，除了取一下颜色，所有的css干脆自己来写了。为了最大的兼容性，只使用**react-native**自带的UI组件，所以使用到的radio, radio group, checkbox，timer button等等组件都实现了一遍，这其实并不困难。

开发中是遇到不少问题的，比如怎么支持iconfont(如 [在React Native中优雅的使用iconfont](http://www.imbeta.cn/zai-react-nativezhong-you-ya-de-shi-yong-iconfont.html) )，怎么让android支持gif(如 [gif-and-webp-support-on-android](https://facebook.github.io/react-native/docs/image.html#gif-and-webp-support-on-android) )图像，怎么`polyfill`低版本android不支持`Promise`(如 [stefanpenner/es6-promise](https://github.com/stefanpenner/es6-promise) )，还如退出后要跳转到主页面以免重新打开停留在其他页面等等各种小问题，这些网上很多参考，都可以找到相应的解决方案，这里就不细列。我也不能细列，因为我的记忆不是很行，我从一开始从事开发工作所遇到的问题多如牛毛，无论困难或粗心的，没有多少是深刻的，深刻记得的只是由于自己失误造成的系统奔溃的严重后果。但对于遇到的问题，我总是都可以找到解决方案（现在打部分都在stackoverfow和github issue page上面找，找不到就直接问）。提一下“怎么让android支持gif(如 [gif-and-webp-support-on-android](https://facebook.github.io/react-native/docs/image.html#gif-and-webp-support-on-android) )图像”，facebook上面的说法是不行的，正确打开方式是：
```gradle
dependencies {
    ...
    compile group: 'com.facebook.fresco', name: 'fresco', version: '1.8.1'
    compile "com.facebook.fresco:animated-gif:1.8.1"
    ...
}
```

列一下两个遇到的问题，虽然不是很困难的问题，但也是比较常用的。一个是接口重要求登录的问题：绝大部分接口都是需要登录权限的，如果长时间不进行操作，那么就需要重定向到登录页面。这在h5上面是非常容易实现的，直接用history跳转，但在**react-native**上面需要详细了解一下react-navigation的文档。你需要重写`getStateForAction`这个函数。如：
``` js
const defaultGetStateForAction = Router.router.getStateForAction

Router.router.getStateForAction = (action, state) => {
  let store = GlobalStore.store
  let {user, app} =  GlobalStore.store.getState()
  if (user.isLogin && user.reLoginNeed) {
    store.dispatch(usrAct.quitApp())
    store.dispatch(usrAct.userReloginNeed(false))
    const routes = [
      ...state.routes,
      {key: app.navKey['LoginPage'], routeName: 'LoginPage'},
    ]
    return {
      ...state,
      routes,
      index: routes.length - 1, // 登录页面
    }
  }
  return defaultGetStateForAction(action, state)
}
```

另外一个问题，属于设计技巧问题，其实也不怎么算技巧。比如，有个预定订单列表的页面，页面上需要实时显示商品原料行情，做法每3秒刷新一次行情，页面还有详情等几个按钮。如果在列表里面进详情页面，那么这个定时刷行情是不会自动停止的，详情页面是不需要显示行情的。如果在进入页面前停止行情，那么会遇到另外一个问题：在详情页面按返回按钮，那么在列表页面行情是不会启动刷新。所以我在查找按返回键时是否有可回调的函数，react-navigation肯定是有类似的解决方案，这个方案暂时没找到。但可以换种思路，可以在订单列表里面设置定时调用的函数，在redux里面设置对于调用条件，设置你可以设置任意的调度条件，那么这个问题就简单解决了。甚至你可以在最顶层的组件设置这样的函数，通过redux判断目前属于哪个页面处于怎么样的状态，实现全局的调用。如：
```js
 taskTimer() {
    const {active, navigation} = this.props
    if (navigation.isFocused()) {
      if (active === ORDER_LIST) {
        this.onData[active]()
      } else if (active === WAIT_RECV_LIST || active === UNPAY_LIST) {
        this.getQuotation()
      }
    }

    if (this.tid < 0) {
      this.tid = setInterval(() => this.taskTimer(), 3000)
    }
  }
```

人们都喜欢一次编写到处运行，很多人都期待react-native编写一次，同时也可以在web中运行(当然原生功能不支持)。网上也要有不少关于这方面的框架，不过好像都不是很理想，比较值得期待脱颖而出的一个。

 