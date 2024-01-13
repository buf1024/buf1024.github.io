---
title: MyJson, JSON C++ 的另一种实现
date: 2012-02-10 21:05:10
categories: [Misc]
tags: [C]
---

JSON，JavaScript Object Notation, 是一种轻量级的数据交换格式。本质上来说，它和XML, YAML等格式化的数据格式没有什么区别。都是为了方便（人机）阅读和交换的数据格式。

JSON，是键值的数据结构，键是主要是指字符串，键主要是指字符串，数值，JSON对象，JSON数组，true, false, null这几种类型。要详细了解这种简单而又实用的数据格式，请参阅，[英文官网](http://www.json.org/)，[中文官网](http://www.json.org/json-zh.html)。

JSON的实现有很多，基本上世界上每种语言都有实现，用C++实现的也有不少，各有特色。因为年前很空闲，于是也随手实现了一个，且叫myjson，特点是：

* 相对来说小，其实比较啰嗦
* 使用非常方便和直观
* 虽然不完全实现JSON，使用[jsoncpp](http://sourceforge.net/projects/jsoncpp/)的测试数据完全测试过
* 在实时交互的系统中，可能效率不高
* 一时兴起写的，可能比较粗略
* ……

  下载：[myjson.zip](http://download.imlgc.com/code/c++/myjson/myjson.zip)

  myjson.zip 文件列表

| 文件/目录 | 说明       |
| ----------- | ------------------------------------ |
| json.h    | myjson 的头文件    |
| json.cpp  | myjson 的实现文件  |
| test.h    | 简单的测试框架，在[/post/2012-02-09-simple-c++-test-framework/](3/post/2012-02-09-simple-c++-test-framework/)文章中说的的框架 |
| test.cpp  | myjson的测试文件   |
| testdata  | 测试数据目录，jsoncpp的测试数据    |

# 实现

myjson的主要类的结构：

![image](/img/myjson/myjson.png "image")

其中JsonValueItem是一时手痒写的，仅充当桥梁作用，实际上我们使用时用到的是JsonArray, JsonValue，和Json这三个类。

各个类的说明：

| 类    | 说明     |
| --------------- | ------------------------------------------------------------------------------------------ |
| Json  | 表示一个JSON对象。       |
| JsonValue     | 表示JSON的值，值包含JSON对象，JSON数组，数值，字符串，false, true, null。|
| JsonArray     | 表示一个JSON数组，数组元素为JSON的值。   |
| JsonValueItem | 用于组织JSON数据结构的类，实际不会使用到，具体来说就是以键作为关键字，组成一有序的链表。 |

主要类主要函数说明：

| 类| 函数      | 说明   |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Json      | static int Parse(Json*&amp; pJson, const char* pBuf);      | 解析JSON数据。<br />参数：pJson 返回的JSON对象<br />参数：pBuf  要解析的数据<br />返回：0成功，其它参考错误码      |
|   | static int Load(Json*&amp; pJson, const char* pFilePath);  | 从文件解析JSON数据。<br />参数：pJson 返回的JSON对象<br />参数：pFilePath文件路径<br />返回：0成功，其它参考错误码 |
|   | int Save(const char* pFilePath);  | 将JSON对象保存到文件。<br />参数：pFilePath文件路径<br />返回：0成功，其它参考错误码   |
|   | JsonValue*Set(const char* szKey, JsonValue& sVal);       | 设置JSON键值。<br />参数：szKey 健<br />参数：sVal 值<br />返回：JSON值，0则出错   |
|   | JsonValue*Get(const char* szKey) const;  | 获取JSON值。<br />参数：szKey 健<br />返回：JSON值，0则出错    |
|   | void Dump(std::string& strDump) const;    | 将JSON对象以最紧密的方式导出。<br />参数：strDump返回的数据<br />返回：无      |
|   | void DumpFORMat(std::string& strDump, int nSpace = FORMAT_SPACE) const;   | 将JSON对象以格式化的方式导出。<br />参数：strDump返回的数据<br />参数：Space 空格数<br />返回：无  |
|   | JsonValue& operator [] (const char*szKey);/const JsonValue& operator [] (const char* szKey) const;       | JSON对象[]操作符，有点类似STL的MAP，当键不存在时，插入一个。<br />参数：szKey 健<br />返回：JSON值（可能无效） |
| JsonValue | operator const char*();
operator char*();<br />operator int();<br />operator long long();<br />operator float();<br />operator double();<br />operator Json();<br />operator JsonArray();    | 各种转换操作符。       |
|   | JsonValue& operator = (bool bVal);<br />JsonValue& operator = (const char* szVal);<br />JsonValue& operator = (int nVal);<br />JsonValue& operator = (long long llVal);<br />JsonValue& operator = (float fVal);<br />JsonValue& operator = (double fVal);<br />JsonValue& operator = (const Json& sVal);<br />JsonValue& operator = (const JsonArray& sArr); | 各种转换赋值符。       |
|   | JsonValue& operator [] (const char*szKey);<br />const JsonValue& operator [] (const char* szKey) const;      | 当值是JSON对象是，此操作符有效。       |
| JsonArray | JsonValue& Get(int nIndex) const; | 获取某下标的JSON值。<br />参数：下标<br />返回：JSON值 |
|   | JsonValue& operator[](int nIndex);/const JsonValue& operator[](int nIndex) const; | 同上   |
|   | int Add(JsonValue& sVal); | 增加一JSON值到数组里。<br />参数：sVal<br />返回：0    |
|   | int GetSize() const;      | 返回数值的长度。       |

# 示例

简单的演示：

```
    Json json;
    JsonValue jv = true;
    json["a"] = jv;
    jv = json;
    json.Set("c", jv);
    jv = (char*)0;
    json.Set("d", jv);
    JsonArray sArr;
    sArr[0] = 100;
    sArr[1] = "abcdefg";
    sArr[2] = false;
    jv = sArr;
    json.Set("e", jv);
  
 // 上述代码产生以下JSON数据
{
  "a":true,
  "c":{
     "a":true
   },
   "d":null,
   "e":[
     100,
     "abcdefg",
     false
    ]
}
```

# 其它

一时兴趣的东西，没有经过大脑的设计，如果用于实时系统还要应该考虑一下其性能，NEW太多了。但是用于实时系统的初始化和清理保存信息还是不错的。注意，这里说的是实时系统。

不过还是很快的啦！两个测试用例在我非常烂的是电脑上还有下面的结果（实际上统计的时间不准确的，是相对大很多的）：

```
[==========] Running 2 tests from 1 test cases
[----------] 2 tests from Json
[ RUN      ] Json.ConstructJson
[       OK ] Json.ConstructJson (1 ms)
[ RUN      ] Json.ParseJson
[       OK ] Json.ParseJson (7 ms)
[----------] 2 tests from Json (17 ms total)

[==========] 2 tests from 1 test case ran (26 ms total)
[  PASSED  ] 2 tests
```
