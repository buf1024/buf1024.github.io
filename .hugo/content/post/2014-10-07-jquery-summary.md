---
title: jquery selector小结 
date: 2014-10-07 21:36:56 
aliases: [/2014/10/07/jquery-summary/]
categories: [Web] 
tags: [JS, JQuery, WebUI] 
---
## jQuery 概述  

 现代网站基本上都离不开jQuery。它是轻量级的js库，兼容css3，还有各种浏览器。文档齐全而详细，还有众多而成熟的插件可用。jQuery就个人理解，主要有以下几方面功能： 统一的DOM选择器，统一事件处理，动画和ajax等，它大大的简化了html，css和js的耦合度。jQuery UI，bootstrap等非常优秀和方便的前端UI库也是紧紧依赖jQuery，对于这些UI的使用后面将也会做些小结，对于不懂前端而又不想非常深入的人来说，这些优秀UI库，是非常好的神器。  
 jQuery库 使用 jQuery() 或 $()使用，$是jQuery的一个别名，如果使用第三方库和$冲突，可以调用`jQuery.noConfig()`来避免冲突。  
 废话少说，总结下jQuery，不是专门的UI人员，也没有很深的研究，所以避免不了很多错误。  

## jQuery DOM选择器  

 jQuery的选择器很灵活，这里是非常笼统的归类，而且不一定齐全(官方有自己的归类方法，可以参考相关文档: [selectors](http://api.jquery.com/category/selectors/))，以下归类是有可能重复的。而且所有的选择器都不是分开独立使用的，可以任君随意组合。  

##### 1. CSS选择器  

 jQuery是兼容CSS的，因而CSS选择器也适用于jQuery。如 `#id .class element`等。  

        例子：         
                 
        $("p")            -> 匹配所有的<p>的元素     
        $(".class")       -> 匹配所有的class属性为"class"的元素   
        $("#id")          -> 匹配所有的id属性为"id"的元素   
        $("p.class#id")   -> 匹配所有的<p>元素,class属性为"class", id属性为"id"的元素   
        $("p a.class#id") -> 匹配所有的<p>元素, 后代（孩子，孙子等）
                          -> 为<a>元素class属性为"class", id属性为"id"的元素 

除此之后， 还多个选择器可以同时使用，用`,`分隔。

        例子：
                         
        $("p,div")        -> 匹配所有的<p>和<div>的元素        

##### 2. 容器属性选择器  

有时元素的选择需求是比较bt的，选择的元素可能是某个元素的直接孩子，或者又是要求某属性值等于多少，于是就有了这类元素的选择器。  

        例子：       
              
        $("*")              -> 匹配所有的元素
        $("E")              -> 匹配所有的<E>元素
        $("E F")            -> 匹配所有的<E>后代为<F>元素
        $("E>F")            -> 匹配所有的<E>直接孩子为<F>元素
        $("E+F")            -> 匹配所有的<E>直接兄弟为<F>元素
        $("E~F")            -> 匹配所有的<E>兄弟为<F>元素
        $("E.C")            -> 匹配所有的<E>class属性为"C"元素
        $("E#I")            -> 匹配所有的<E>id属性为"I"元素
        $("E[A]")           -> 匹配所有的<E>含有属性A元素
        $("E[A='V']")       -> 匹配所有的<E>含有属性A且值为'V'元素
        $("E[A^='V']")      -> 匹配所有的<E>含有属性A且值以'V'开头元素
        $("E[A$='V']")      -> 匹配所有的<E>含有属性A且值以'V'结尾元素
        $("E[A!='V']")      -> 匹配所有的<E>不含有属性A或者含属性A且值不为'V'元素
        $("E[A*='V']")      -> 匹配所有的<E>含有属性A且值包含'V'元素

##### 3. 位置过滤器

有时，我们还需要根据元素的位置选择，于是有了这类选择器。 这类选择器是从匹配的元素中选出特定位置的元素。如，$("p a:first")，选择所有p元素，后代为a中的第一个元素集合。

        例子：       
              
        :first               -> 在所有匹配元素中匹配所有的第一个元素，
                             -> 如, $("li a:first")
        :last                -> 在所有匹配元素中匹配所有的最后一个元素，
                             -> 如, $("li a:last")
        :first-child         -> 在所有匹配元素中匹配所有的第一个孩子元素，
                             -> 如, $("li:first-child")
        :last-child          -> 在所有匹配元素中匹配所有的最后一个孩子元素，
                             -> 如, $("li:last-child")
        :only-child          -> 在所有匹配元素中匹配所有的没有兄弟元素，
                             -> 如, $("li:only-child")
        :nth-child(n)        -> 在所有匹配元素中匹配所有的第n个孩子元素，
                             -> 如, $("li:nth-child(2)")
        :nth-child(even|odd) -> 在所有匹配元素中匹配所有的第偶数或奇数个孩子元素，
                             -> 如, $("li:nth-child(even)")
        :nth-child(Xn+Y)     -> 在所有匹配元素中匹配所有的符合公式"Xn+Y"个孩子元素，
                             -> 如, $("li:nth-child(3n+1)")
        :event               -> 在所有匹配元素中匹配所有的偶数孩子元素，
                             -> 如, $("li:even")
        :odd                 -> 在所有匹配元素中匹配所有的奇数元素，
                             -> 如, $("li:odd")
        :eq(n)               -> 在所有匹配元素中匹配所有的第n个元素，
                             -> 如, $("li:eq(2)")
        :gt(n)               -> 在所有匹配元素中匹配所有的大于n元素，
                             -> 如, $("li:gt(4)")
        :lt(n)               -> 在所有匹配元素中匹配所有的小于n元素，
                             -> 如, $("li:lt(5)")

##### 4. 选择过滤器

有时，我们还需要根据选择的元素进行过滤，于是有了这类选择器。jQuery提供的大量的过滤器给我们使用，基本上很bt的需求都可以满足。如，$(":checkbox:checked:enabled")，选择所有"enabled"和"checked"的checkbox。

        例子：       
              
        :animated            -> 匹配所有的正动画控制中元素
        :button              -> 匹配所有的"button"元素,
                             -> "button"包括: input[type='submit'], input[type='reset'], 
                             ->               input[type='button']和 button。
        :checkbox            -> 匹配所有的"checkbox"元素，指input[type='checkbox']
        :checked             -> 匹配所有的"checkbox"或"radio"在"checked"状态元素
        :contains(food)      -> 匹配所有的包含文本"food"元素
        :disabled            -> 匹配所有的在"disabled"状态元素
        :enabled             -> 匹配所有的在"enabled"状态元素
        :file                -> 匹配所有的文件输入元素，指input[type='file']
        :has(selector)       -> 匹配所有的包含在"selector"中元素
        :header              -> 匹配所有的"header"元素，指<h1> 到 <h6>
        :hidden              -> 匹配所有的隐藏状态元素
        :image               -> 匹配所有的图像元素，指input[type='image']
        :input               -> 匹配所有的"fORM"元素，指input, select, textarea, button
        :not(selector)       -> 匹配所有的不包含在"selector"中元素
        :parent              -> 匹配所有的包括孩子而且孩子非空元素
        :password            -> 匹配所有的文件密码元素，指input[type='password']
        :radio               -> 匹配所有的文件"radio"元素，指input[type='radio']
        :reset               -> 匹配所有的文件"reset"元素，
                             -> 指input[type='reset']或button[type='reset']
        :selected            -> 匹配所有的在"selected"状态元素
        :submit              -> 匹配所有的文件"submit"元素，
                             -> 指input[type='submit']或button[type='submit']
        :text                -> 匹配所有的"text"元素，指input[type='text']
        :visible             -> 匹配所有的可见状态元素

## jQuery 其他杂项

参考[jQuery API文档](http://api.jquery.com/)
