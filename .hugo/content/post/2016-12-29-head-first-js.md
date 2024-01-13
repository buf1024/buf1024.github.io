---
title: js快速参考
date: 2016-12-29 17:08:46
aliases: [/2016/12/29/head-first-JS/]
categories: [Web]
tags: [Html, CSS, JS]
---
P.S.: 因为极少用，所以js一直都忘记，所以，现在记录一下, 适合于曾以为入门js却又少使用的参考……

JS 运行于宿主环境，常见宿主环境有web浏览器和node。在web浏览器里，主要是放到html页面，通过`<script>`标签实现。`<script>`包括6个属性:

        async            -> 可选。异步加载，不影响界面其他元素加载
        charset          -> 可选。制定字符集，浏览器会忽略该值
        languange        -> 可选。已经废弃，指明脚本的语言
        src              -> 可选。外部脚本的地址
        type             -> 可选。可看成是language的替代，已经不推荐使用，目前一般是"text/javascript"
        defer            -> 可选。脚本延迟到页面被加载完成后才执行

如果不包括`defer`和`async`，那么脚本的执行顺序为出现的顺序。`<script>`放的位置可放到`<head>`或`<body>`中，前者需等到脚本加载完毕后，才呈现页面，这样给人的感觉是慢和假死，后者是脚本加载之前，页面可以呈现。`<noscript>`可以检测脚本是否被禁用或步支持。

## JS 核心

#### 基本语法

js区分大小写，注释风格和c语言一致。js有严格模式的编译指令:`"use strict;"`，严格模式下不安全的代码将抛出异常。`var`定义变量，在非严格模式下，在作用域内，如果忽略则变成全局变量，不忽略则为局部变量，如：

        function test() {
            var localVar; // 定义局部变量
            globalVar;    // 定义全局变量
        }

js数据类型有，5中基本操作类型: `Undefinded Null Boolean Number String`，一种复杂数据类型: `Object`，以及用户定义函数类型：`function`。`typeof`操作符(注意，是操作符不是函数，类似c语言的`sizeof`)可返回数据类型，字符串表示分别为: `undefined object boolean number string object function (Null类型会返回object)`。

`Boolean()`函数，可将非`Boolean`类型的数据转换为`Boolean`类型数据，转换规则： 任何非空的字符串，任何非零数据，任何非空对象都转换非`true`，其他则为`false`。

`Number.MIN_VALUE Number.MAX_VALUE`分别表示数值的最小和最大值，超出这个范围，则为`-Infinity或Infinity`, `isInfinity()`函数判断数值是否超出范围。`NaN`该返回数值的操作无法返回数值，如除以0则会返回`NaN`，函数`isNaN()`可以判断，另外`isNaN()`也可以应用于对象，首先会调用对象的`valueOf()`方法，确认是否可以转换为数值，如果不可以，再调用`toString()`进行测试。数值转换函数：`Number() parseInt() parseFloat()`。`Number()`转换规则:如果是`Boolean true false`，分别转换为1或0；如果是`null`，转换为0；如果是`undefined`，转换为`NaN`；如果是字符串，字符串前面数值转换为对应数值，空转换为0，有效的16进制，转换为对应的数值，其余的转换为`NaN`；如果是对象，首先会调用对象的`valueOf()`方法，确认是否可以转换为数值，如果不可以，再调用`toString()`。由于函数`Number()`使用不是很方便，所以衍生了`parseInt() parseFloat()`，转换为对应的整型和浮点型。

字符串类型用`“”或者‘’`括起来，两者没区别，匹配即可，可以使用`+`将多个字符传连接起来。除了`null`和`undefined`外，转换为字符串，直接调用`toString`方法即可。

对象类型`Object`是其他所以对象的基础，包括以下方法:

        Constructor                         -> 构造函数
        hasOwnProperty(propertyName)        -> 属性是否存在实例中
        isPrototypeOf(object)               -> 是否另外一个对象的原型
        propertyIsEnumerable(propertyName)  -> 属性是否可以枚举
        toLocalString()                     -> 本地字符串表示
        toString()                          -> 转换为字符串表示
        valueOf()                           -> 返回对象的字符串，数值或布尔值表示，一般和toString一样

js函数与其他函数不同，js函数不关心传递参数的个数和数据类型，函数体内，`arguments`数组保存传递过来的参数。函数没有重载，如果定义两个一样的函数，后面的会覆盖前面的参数。

执行环境是js的一个比较**重要**的概念。每个执行环境都有与只关联的**变量对象**，定义的变量或函数都在这个**变量对象**中(虽然我们代码无法访问该**变量对象**)。对于web宿主环境，全局执行环境为`window`对象，所有定义的函数或变量都在这个对象中。每个函数都有自己的执行环境，执行函数时，就会将函数的执行环境压入`环境栈`，执行完毕，则弹出。在执行环境里面，会创建**变量对象**的**作用链域**，用以控制变量或函数的访问，**作用链域**一直延申到全局执行环境，标识符的解析是从当前执行环境一直搜索到全局执行环境。执行环境虽然只包括全局执行环境和函数执行环境，但是可以延长: `try-catch`中的`catch`语句块，会创建**变量对象**，包括抛出错误对象的声明和`with`语句，将指定对象添加到**作用链域**中。记住的是，**js没有像c语言那种块级的作用域**。

#### 基本引用类型

`Object`类型，有点像键值数据结构，常用写法如，`var values = {name:"who", age:16}`

`Array`类型，数组类型 。确认一个变量是否是该类型，使用方法`Array.isArray()`。数组定义了5个函数用于迭代：

        every       -> 对数组中的每一个值调用回调函数，如果所有的回调都返回true，则返回true
        filter      -> 对数组中的每一个值调用回调函数，返回所有返回true的数组
        forEach     -> 对数组中的每一个值调用回调函数，没有返回值
        map         -> 对数组中的每一个值调用回调函数，每次调用的返回值组成数组返回
        some        -> 对数组中的每一个值调用回调函数，如果某一回调都返回true，则返回true

`Date`类型，表示时间类型。

`RegExp`类型，正则表达式类型，写法为，`var patten = /regexp/flags;`。支持3个标志，`gim`，分别表示，全局模式，不区分大小写，多行模式。对于任何一个正则表达式的实例，都包括以下属性：

        global          -> 是否设置g标记
        ignoreCase      -> 是否设置i标记
        lastIndex       -> 下一个开始搜索的下标
        multiline       -> 是否设置m标记
        source          -> 表达式的字符串表示

正则表达式类型，重要的方法是`exec`，返回匹配组。

`Function`类型，函数类型。函数类型除了包括`arguments this`属性外，还包括`callee caller`，分别代表函数本身和调用者。由于函数也是对象，所有，也有自己的属性。如，`length prototype`， `length`表示接收的命名参数的个数。每个函数都包含两个方法：`apply() call()`，最只要的作用的扩大函数运行的作用域， 除此之外，还包括了`bind`方法。

#### 面向对象

创建对象常用的模式：

- 使用`Object`构造函数或对象常量，如：

         var person = {
                 name: "hello",
                 sayName: function() {
                         console.log(this.name);
                 }
         }

 缺点明显，单个对象，大量重复代码。

- 工厂模式，如：

         function create() {
                 var o = new Object();
                 o.name = "hello";
                 o.sayName = function() {
                         console.log(this.name);
                 }
         }
         var o = create();

 解决了问题，创建多个类似对象，没有解决，对象识别。

- 构造函数模式，如：

         function Person(name) {
                 this.name = name;
                 this.sayName = function() {
                         console.log(this.name);
                 }
         }

         var p = new Person("hello");

 写法类似于类，函数对象每实例化一次，就多一个对象。如果将函数放到构造函数外，就有多个全局函数，无封装性。

- 原型+构造函数模式，最常用模式，如：

         function Person(name) {
                 this.name = name;
         }
         Person.prototype = {
                 constructor: Person,
                 sayName: function() {
                        console.log(this.name);
                 }
         }

 原型+构造函数模式，使用最广泛，认可度最高，默认的创建自定义对象模式。

继承对象，js主要通过实现继承来实现，而且使用原型链。主要的原理是，用一个引用类型继承另外一个引用类型的属性和方法。

- 原型链继承，如：

         function SuperType(name) {
                 this.name = name;
         }
         SuperType.prototype.sayName = function() {
                 console.log(this.name);
         }

         function SubType(prop) {
                 this.prop = prop;
         }
         SubType.prototype = new SuperType();

 上述继承的问题是，SuperType的属性被SubType共享，若包括引用类型，则SubType实例互相影响。稍微改进一下，借用构造函数组合继承。

- 组合继承，如:

         function SuperType(name) {
                 this.name = name;
         }
         SuperType.prototype.sayName = function() {
                 console.log(this.name);
         }

         function SubType(prop) {
                 SuperType.call(this, "hello"); // 注意
                 this.prop = prop;
         }
         SubType.prototype = new SuperType();

 组合继承为js最常用的继承方式。

## 浏览器js

#### bom对象&dom

`window对象`是bom对象的核心对象，代表一个浏览器的实例，同时又是js的全局对象。访问未声明的对象是会抛出异常的，但是，通过window对象查询，不会，会返回`undefined`。`location对象`是bom最有用的对象之一，它提供当前窗口加载的文档信息。还包括，`navigation对象`， `screen对象`，`history对象`等。js通过`Document`类型表示文档，在浏览器中，表示整个页面，`document对象`是`window对象`的一个属性。

事件冒泡是指，从事件发生的节点，一直往上传播到根节点。事件的捕获是指，从根节点，一直往下传播到发生事件的节点。dom事件流包括三个阶段：事件捕获阶段，处于目标阶段和事件冒泡阶段。事件类型有：

- UI事件(`load unload abort error select resize scroll`等。
- 焦点事件(`blur focus focusin focusout`)
- 键盘事件(`keydown keypress keyup`)
- 复合事件(`compositionstart compositionend compositionupdate`)
- 变动事件(`DOMSubtreeModified DOMNodeInserted DOMNodeRemoved DOMNodeInsertedIntoDocument DOMNodeRemovedFromDocument DOMAttrModified DOMCharacterDataModified`)
- H5事件(`contextmenu beforeunload DOMContentLoaded readystatechange pageshow pagehide hashchange`)
- 设备事件(`orientationchange MozOrientation deviceorientation devicemotion`)
- 触摸和手势事件(`touchstart touchmove touchend touchcancel gesturestart gesturechange gestureend`)

## ES6

es6为2015发布的的js标准，es6用起来更新现代的编程语言。目前并不是所有的浏览器都支持，[babel](https://babeljs.io/)。es6新特性：

- let, const
 这两个特性是解决块级作用域和，常量定义问题。es6之前，js没有块级变量作用域，`let`就是做这个事情。`const`是用来声明常量的，任何改变`const`值的行为都导致报错。如：

         var a = [];
         for (var i = 0; i < 10; i++) {
           a[i] = function () {
             console.log(i);
           };
         }
         a[6](); // 10

         var a = [];
         for (let i = 0; i < 10; i++) {
           a[i] = function () {
             console.log(i);
           };
         }
         a[6](); // 6

- class, extends, super
 这几个特性，是为了解决js不明显的面向对象特征。如：

         class Animal {
             constructor(){
                 this.type = 'animal'
             }
             says(say){
                 console.log(this.type + ' says ' + say)
             }
         }
         
         let animal = new Animal()
         animal.says('hello') //animal says hello
         
         class Cat extends Animal {
             constructor(){
                 super()
                 this.type = 'cat'
             }
         }
         
         let cat = new Cat()
         cat.says('hello') //cat says hello

- 箭头函数
 箭头函数比普通的函数简洁，同时函数体内的this对象，就是定义时所在的对象，而不是使用时所在的对象。如：

         class Animal {
            constructor(){
                this.type = 'animal'
            }
            says(say){
                setTimeout( () => {
                    console.log(this.type + ' says ' + say)
                }, 1000)
            }
         }
         var animal = new Animal()
         animal.says('hi')  //animal says hi

- template string
 用反引号（`）来标识起始，用${}来引用变量，而且所有的空格和缩进都会被保留在输出之中。如：

         $("#result").append(`
          There are <b>${basket.count}</b> items
           in your basket, <em>${basket.onSale}</em>
          are on sale!
        `);

- destructuring
 ES6允许按照一定模式，从数组和对象中提取值，对变量进行赋值，这被称为解构（Destructuring）。如：

         let cat = 'ken'
         let dog = 'lili'
         let zoo = {cat, dog}
         console.log(zoo)  //Object {cat: "ken", dog: "lili"}
         
         let dog = {type: 'animal', many: 2}
         let { type, many} = dog
         console.log(type, many)   //animal 2

- default, rest
 默认值和剩余参数。如：

         function animal(type = 'cat'){
             console.log(type)
         }
         animal()
         
         function animals(...types){
             console.log(types)
         }
         animals('cat', 'dog', 'fish') //["cat", "dog", "fish"]

## 终

对于一个常年工作于古老语言的码农，这里的记录必定是错漏百出的。

最后，多写才是硬道路。[javascript参考](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/)
