---
title: ReactJS  --  试水篇
date: 2018-01-25 09:32:32
categories: [reactjs]
aliases: [/2018/01/25/reactjs-try/]
tags: [frontend]
---
## 简介
在大前端里，有三个特别流行的MVVM（参考：[MVC，MVP 和 MVVM 的图示](http://www.ruanyifeng.com/blog/2015/02/mvcmvp_mvvm.html)）框架，[AngularJS](https://angularjs.org/)，[ReactJS](https://reactjs.org/)和[Vue](https://vuejs.org/)。在MVVM之前，前后端的开发是紧耦合的，前端设计页面，后端服务端生成渲染页面。这中方式开发最大的坏处是，前后端的代码混合在一起，不容易维护，当然最可能好处就是，前后端紧密合作或许会产生感情的火花~~。使用MVVM框架，前后分离，定好交互的接口后，前后端就可以进行独立的开发，互不干扰，代码分别维护。基于这个优点，毫不犹豫的产生了想使用MVVM的想法。

关于AngularJS，ReactJS和Vue，网上有很多的资料参考。AngularJS没有接触过，由google维护，使用的是[TypeScript](http://www.typescriptlang.org/)，据说使用起来稍微复杂和笨重。ReactJS，是Facebook维护的，兼容相对低版本的浏览器，使用自己发明的JSX语法，因为JSX把JS和HTML混合起来，所以相对一部分人第一次接触，并不会这么喜欢它，但是当你使用时，也不会感觉到什么异样。Vue是个人维护的项目，感觉参考了吸收了AngularJS和ReactJS，上手相对简单，遗憾的是不支持低版本的浏览器。经过对比后，开发使用了ReactJS，最主要的原因是，兼容相对低版本的浏览器。

## 入门
这里说的是后台人员的入门。因为前端的发展大都是借鉴后台的技术，甚至不少前端框架是后台人员开发的，所以后台开发人员进行前端开发，门槛不会太大。那么对于后端需要掌握什么的技术呢？

JS 是必须的，除了 JS 还不够，你还得熟悉ES6，ES6的最好参考在: [ECMAScript 6 入门](http://es6.ruanyifeng.com/)。看完ES6之后，你才会觉得JS是一门编程语言。JS的参考也很重要，mozilla的参考比较详细：[JavaScript reference](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference)。

ES6代码并不能在浏览器执行，所以需要将ES6代码转换成普通的JS代码，这是[babeljs](http://babeljs.io/)干的事情，不过你很少会直接接触babeljs，更多的情况下，你是通过使用构建工具间接使用到。[webpack](http://webpack.github.io/)是一个很好的构建工具，这篇入门级别介绍挺好：[入门Webpack，看这篇就够了](https://www.jianshu.com/p/42e11515c10f)。当然还有很多构建工具，不过webpack用的人相对多点，webpack的配置对于入门者来说，是比较复杂的，但是很多情况下，也不需要深入。

ReactJS看[react-china](https://doc.react-china.org/)中文翻译，总体上也能入门，入门估计实战可能还有点距离，很多情况下，你可能还需要[React Router](https://reacttraining.com/react-router/web/guides/redux-integration)。React Router据说每次版本变化都比较大，对于新开发的项目，我们当然选择最新版本，这篇文章可能有助于理解: [初探 React Router 4.0](https://www.jianshu.com/p/e3adc9b5f75c/)。你可能还需要全局的状态管理，你需要[Redux](https://redux.js.org/)，看中文翻译即可[Redux 中文文档](http://www.redux.org.cn/)，Redux的代码仓库里面带有不少例子，看里面的例子对理解Redux和ReactJS都很有帮助。[阮一峰](http://www.ruanyifeng.com/)的三篇入门教程对理解Redux也是非常有帮助的。[Redux 入门教程（一）：基本用法](http://www.ruanyifeng.com/blog/2016/09/redux_tutorial_part_one_basic_usages.html), [Redux 入门教程（二）：中间件与异步操作](http://www.ruanyifeng.com/blog/2016/09/redux_tutorial_part_two_async_operations.html), [Redux 入门教程（三）：React-Redux 的用法](http://www.ruanyifeng.com/blog/2016/09/redux_tutorial_part_three_react-redux.html)。

web的请求大多数情况下都是异步，异步的数据流Redux推荐的数据流管理是[redux-thunk](https://github.com/gaearon/redux-thunk)，但是实际上使用时，你不用redux-thunk也没什么毛病，自己控制`dispatch action`。所以，对于异步数据，更喜欢[redux-saga](https://redux-saga.js.org/)，他利用了es6的generator，使用起来像后台的协程这样的，用顺序的方式写异步的代码。redux-saga的中文文档，最好看繁体翻译的，因为简体中文翻译好像有点拗口，而且没有跟进最新版本：[redux-saga](https://neighborhood999.github.io/redux-saga/)。

另外推荐一看下[dva](https://github.com/dvajs/dva)这个项目(据说维护不积极)，但是个人觉得封装的很深入，用起来会很顺手，只是使用了之后，很容易忘记原来的技术栈，基于据说的维护不积极，所以我们也不考虑使用它。不过，里面一篇，最佳实践比较值得一看：[React + Redux 最佳实践](https://github.com/sorrycc/blog/issues/1)

以上，看起需要掌握的东西比较多，但是实践上对于后台人员并不会有太多的困难。对于后台开发人员来说，困难的不是以上部分，而是没有提及的css部分，这是要靠天赋和美感的。[less](http://lesscss.org/)和[sass](http://sass-lang.com/)可以简化css的编写，但是对于没有大量实践经验的后台人员来说，css编写很耗时间。

## React部分技术栈
React相关入门的参考上一节已经做了简介，这里是看上面部分关于React部分的笔记，当然简单不完整：

#### React
`ReactDOM.render` 是React最基本方法，用于将模板转为HTML语言，并插入指定的DOM节点。React元素都是immutable不可变的。当元素被创建之后，你是无法改变其内容或属性的。一个元素就好像是动画里的一帧，它代表应用界面在某一时间点的样子。

React使用的是JSX语法，JSX的基本语法规则其实非常简单：遇到HTML标签（以<开头），就用HTML规则解析；遇到代码块（以{开头），就用JavaScript规则解析。推荐在JSX代码的外面扩上一个小括号，这样可以防止分号自动插入的bug。

组件是React复用的基本单元，组件从概念上看就像是函数，它可以接收任意的输入值（称之为`props`），并返回一个需要在页面上展示的React元素。组件类的第一个字母必须**大写**。

`this.props.children`属性。它表示组件的所有子节点，React 提供一个工具方法 React.Children 来处理 `this.props.children`。我们可以用`React.Children.map`来遍历子节点，而不用担心`this.props.children`的数据类型是 `undefined`还是`object`。

组件最简单的方式是使用JavaScript函数，接收一个单一的`props`对象并返回了一个React元素，缺点是用不了生命周期钩子，写法：
```js
function Welcome(props) {
  return <h1>Hello, {props.name}</h1>
}
// 类型检查propTypes：
import PropTypes from 'prop-types'
Welcome.propTypes = {
  name: PropTypes.string
}
// 为属性指定默认值:
Welcome.defaultProps = {
  name: 'Stranger'
}
```

另外的组件方法是用类实现，可以包括状态和生命周期。`props`和`state`的区别：`props`为外部传递给组件的，而`state`为组件内部的状态。`this.state`代表组件的状态，关于state：
1. 不要直接更新状态
  更新方式：`this.setState({comment: 'Hello'})`

2. 状态更新可能是异步的 
  ``` js
  this.setState(function(prevState, props) {
    return {
      counter: prevState.counter + props.increment
    }
  })
  ```

3. 状态更新支持合并
  ```js
  this.state = {
      posts: [],
      comments: []
  };
  ```

类组件实现方法

1. 继承React.Component
2. 实现render()方法
3. 创建构造函数实现增加state状态（可选）
4. 添加生命周期钩子（可选）

例子：
```js
class Clock extends React.Component {
  constructor(props) {
    super(props);
    this.state = {date: new Date()};
  }
  render() {
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>It is {this.state.date.toLocaleTimeString()}.</h2>
      </div>
    )
  }
}
ReactDOM.render(
  <Clock />,
  document.getElementById('root')
)
```

组件的生命周期(详细参考: [React：组件的生命周期](https://segmentfault.com/a/1190000004168886))分成三个状态：
  - Mounting：已插入真实 DOM
  - Updating：正在被重新渲染
  - Unmounting：已移出真实 DOM

组件对应的生命周期
  - componentWillMount()
  - componentDidMount()
  - componentWillUpdate(object nextProps, object nextState)
  - componentDidUpdate(object prevProps, object prevState)
  - componentWillUnmount()

额外的生命周期
  - componentWillReceiveProps(object nextProps)：已加载组件收到新的参数时调用
  - shouldComponentUpdate(object nextProps, object nextState)：组件判断是否重新渲染时调用

组件类似于面向对象的类，外部是不需要知道内部的状态。

在事件处理，最好使用箭头函数，这样可以避免`this`的影响。在 React 中你不能使用返回 false 的方式阻止默认行为。你必须明确的使用 `preventDefault`，如。
```js
  handleClick = (e) => {
    e.preventDefault();
    console.log('The link was clicked.');
  }
  ```
也可以在构造函数里面绑定`this`, 向事件传递参数：
``` js
<button onClick={(e) => this.deleteRow(id, e)}>Delete Row</button>
<button onClick={this.deleteRow.bind(this, id)}>Delete Row</button>
```
参数 e 作为 React 事件对象将会被作为第二个参数进行传递。通过箭头函数的方式，事件对象必须显式的进行传递，但是通过 bind 的方式，事件对象以及更多的参数将会被隐式的进行传递


#### REDUX

应用中所有的 state 都以一个对象树的形式储存在一个单一的 store 中。惟一改变 state 的办法是触发 action，一个描述发生什么的对象。为了描述 action 如何改变 state 树，你需要编写 reducers。

三大原则：
- 单一数据源, 整个应用的 state 被储存在一棵 object tree 中，并且这个 object tree 只存在于唯一一个 store 中。
- State 是只读的, 惟一改变 state 的方法就是触发 action，action 是一个用于描述已发生事件的普通对象。
- 使用纯函数来执行修改, 为了描述 action 如何改变 state tree ，你需要编写 reducers。

一般来说你会通过 store.dispatch() 将 action 传到 store。action 内必须使用一个字符串类型的 type 字段来表示将要执行的动作。除了 type 字段外，action 对象的结构完全由你自己决定。我们应该尽量减少在 action 中传递的数据
Action 创建函数 就是生成 action 的方法。Action 只是描述了有事情发生了这一事实，并没有指明应用如何更新 state。而这正是 reducer 要做的事情。reducer 就是一个纯函数，接收旧的 state 和 action，返回新的 state。

注意:
- 不要修改 state。 使用 Object.assign() 新建了一个副本。
- 在 default 情况下返回旧的 state

Store 有以下职责：
  - 维持应用的 state；
  - 提供 getState() 方法获取 state；
  - 提供 dispatch(action) 方法更新 state；
  - 通过 subscribe(listener) 注册监听器;
  - 通过 subscribe(listener) 返回的函数注销监听器

Redux 的设计思想很简单
1. Web 应用是一个状态机，视图与状态是一一对应的。
2. 所有的状态，保存在一个对象里面。

异步操作的差别是它要发出三种 Action。
  - 操作发起时的 Action
  - 操作成功时的 Action
  - 操作失败时的 Action

  - 操作开始时，送出一个 Action，触发 State 更新为"正在操作"状态，View 重新渲染
  - 操作结束后，再送出一个 Action，触发 State 更新为"操作结束"状态，View 再一次重新渲染

action creator的写法：
```js
const asyncAction = (base) => {
  return ['REQUEST', 'SUCCESS', 'FAILURE'].reduce((acc, type) => {
    acc[type] = `${base}_${type}`
    return acc
  }, {})
}
const action = (type, payload) => {
  return {type, ...payload}
}
export const LOAD_COMMODITY = asyncAction('LOAD_COMMODITY')
export const loadCommodity = {
  request: () => action(LOAD_COMMODITY['REQUEST']),
  success: (payload) => action(LOAD_COMMODITY['SUCCESS'], {...payload}),
  failure: () => action(LOAD_COMMODITY['FAILURE'])
}
```

UI组件和容器组件通过([react-redux](github.com/reactjs/react-redux))connect连接在一起, 通过Provider注入，原型：
```js
connect(mapStateToProps, mapDispatchToProps)(UIComponent)

const mapStateToProps = (state, ownProps) => {
  return {
    active: ownProps.filter === state.visibilityFilter
  }
}
const mapDispatchToProps = (
  dispatch,
  ownProps
) => {
  return {
    onClick: () => {
      dispatch({
        type: 'SET_VISIBILITY_FILTER',
        filter: ownProps.filter
      });
    }
  };
}
```
当state变化时，导致filter的改变，用reselect可以缓存state，输入不变，则结果不变。
如果connect第而个参数是action creator，那么直接调用action creator就可以进行dispatch

Redux中间件是对dispatch的封装（redux-saga是一个中间件），原型：
``` js
const logger = store => next => action => {
  console.log('dispatching', action)
  let result = next(action)
  console.log('next state', store.getState())
  return result
}
```
reducer的原型:
``` js
const todos = (state = initialState, action) => {
}
//必须返回新的state
```


## 优化与踩坑
当你写比较多的reducer的时候，你会发现，所有的reducer的写法都类似，只要定义好action传过来的字段是payload，那很多reducer的写法基本和以下类似：
```js
import merge from 'lodash/merge'
import * as userActions from '../actions/user'

const initialState = {
  // ... 其他字段
}

const user = (state = initialState, action) => {
  if (action.payload) {
    return merge({}, state, action.payload)
  }
  switch (action.type) {
    case userActions.USER_LOGOUT['SUCCESS']:
      return merge({}, state, initialState)
    default:
      return state
  }
  return state
}

export { user }
```

ReactJS真正使用起来还是有不少坑，印象比较深的是key字段，在渲染列表时，或其他组件时遇到多次。举个栗子，如下代码，如果没有key字段，那么DeliveryList组件的生命周期只有创建一次。
```js
class WorkBenchLayout extends Component {
  render () {
    return (
      <Layout className="workbench">
          <Switch>
            {/* ... 其他 */}
            <PrivateRoute path="/workbench/delivery/list" exact component={DeliveryList} key='list'/>
            <PrivateRoute path="/workbench/delivery/todo" exact component={DeliveryList} key='todo'/>
            {/* ... 其他 */}
          </Switch>
      </Layout>
    )
  }
}

```
## 总结
这次试水React并不算怎么顺利。可能的原因：
1. 不安规定制定接口
前后分离的开发模式，需要开发前制定接口，然后各自开发。但现实却是，接口制定完全由后端控制，后端自己先开发完毕了一个接口功能，就制定一个接口。这样前端就处于一个非常被动的地位，很容易成为开发的瓶颈。

2. 后端接口实现的缺陷，沟通困难。
安照原型图，后端制定的接口里面，有不少需要的字段缺少的，而且对接口测试不足，完全依赖于前端输入。举个栗子，有个入库的接口，前端填好相关ID和名字后，让后端进行入库，后端完全不校验数据的合法性，不管前端传输的任何和数据，都直接入库。这样导致后台的审核的系统，报错。当然这些缺陷的软件开发这不是问题，最关键的问题是，后台人员沟通比较困难，要后台修改一个问题，像大爷一样求着修改。后来了解到，之前另外一个java后台沟通更困难。本来前后分离，是减轻后端的开发工作量的，没有料到的是，沟通和不合作却成为一个困难。可能这也跟公司的前景相关

3. 前端优化页面编写
前端的开发中有不少相似的页面和逻辑，按照复用的原则理论上可以让代码更少，然而可能是经验方面的不足，写了很多冗余的代码，这是需要提高

最后，总结：
1. 如果决定了采用前后分离的开发模式，那么前后端的开发都必须达成一致，同意这种开发的模式，积极参与进来，否则导致后面的沟通问题
2. 前后方的接口制定，需要双方同时制定，不能由一方制定后，再给另外一方，中途出现问题，再做修改，然后同时开发，必须同时分别进行接口测试。
3. 如果后端同事较多，可以让后端同事适当参与前端开发，毕竟后端向端开发的转化还相对容易
4. 如果原来的技术栈可以适应当前的业务情景，组内兴趣对新技术兴趣不高，那么还是保留使用原来的技术栈

P.S:
这里没有针对任何人。


