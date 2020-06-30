---
title: 'Very Very Simple C++ Testing Framework'
date: 2012-02-09 20:50:00
aliases: [/2012/02/09/simple-c++-test-framework/]
categories: [unittest]  
tags: [c++, gtest, tdd] 
---
### 简介  

测试驱动开发(TDD)是敏捷开发的一种方法，TDD的一个重要的工具就是单元测试(Unit Test)。如果想详细了解什么是TDD和单元测试请GOOGLE之。简单来和简略地说TDD是在测试驱动下的开发，先写完成测试然后努力使测试通过。单元测试大体包括下面所说的断言，测试固件（非常不喜欢国人这样翻译）和测试环境。  

C++单元测试框架google 的[gtest](http://code.google.com/p/googletest/)是其中的不错一款，本测试框架是模仿它的。为什么不直接用它而要模仿它？因为公司不允许（允许但过程很麻烦）使用第三方的库，而公司又要求作单元测试，于是这个东西就写了出来。  

特点：  

* 小，只有一个头文件，其实你会觉得很啰嗦。  
* 使用方法和GTEST一样，当然也没那么全。  
* 跨平台  
* ……

下载：[test.h](https://github.com/buf1024/mydoc/blob/master/tmpcode/test.h)  

### 断言(Test Assertion)  

断言包括两种，一种是以EXPECT_XXX，一种是ASSERT_XX，前者即使断言不通过，继续执行此测试用例下面的测试，后都则立即停止执行当前的测试用例。  

| 断言宏         | 含义  |  
| ------------- |:-------------|  
| EXPECT_EQ/ASSERT_EQ| 期待数值相等 |   
| EXPECT_NEQ/ASSERT_NEQ	      | 期待数值不相等      |   
| EXPECT_STREQ/ASSERT_STREQ	 | 期待字符串相等      |     
| EXPECT_STRNEQ/ASSERT_STRNEQ		 | 期待字符串不相等    |    
| EXPECT_BINEQ/ASSERT_BINEQ		 | 期待二进制相等      |   
| EXPECT_BINNEQ/ASSERT_BINNEQ			 | 期待二进制不相等      |  
| EXPECT_TRUE/ASSERT_TRUE				 | 期待值为真      | 
| EXPECT_FALSE/ASSERT_FALSE				 | 期待值为假      | 


### 固件(Test Fixture)  

测试固件可看作是一组测试用例的集合，在执行这组测试用例之前或之后，可以先执行一些操作，同时也可以在执行单个测试用例之前或之后，也可以执行一些操作。对于单个测试用用例，这里，当作是包含一个测试用例的测试固件来处理。  

 使用Test Fixture，你要继承Test这个类。类声明：  

        class Test
        {
            ....
        public:
            static void SetUpTestCase()
            {
            }
            static void TearDownTestCase()
            {
            }
            virtual void SetUp()
            {
            }
            virtual void TearDown()
            {
            }
            ...
        };

然后重写上述方法，带static的是执行所有用例之前或后执行的，不带的是每个测试用例执行之前或之后执行的。  

提供两个宏进行操作：  

| 宏         | 含义  |  实例  |  
| ------------- |:-------------|:-------------| 
| TEST_F |测试固件|TEST_F(MyTestFixture, MyCase) <br>{ <br>} <br>MyTestFixture是你继续Test的固件。<br>MyCase是这个用例的名称。|   
| TEST |测试用用例	|TEST(MyDummyFixture, MyCase) <br>{ <br>} <br>MyTestFixture是固件名称，事实上它什么也不做。<br>MyCase是这个用例的名称。|   

### 环境(Test Environment)  

测试环境是所有Test Fixture执行之前或之后进行的准备和清理。继续类Environment，其声明为：  

        class Environment
        {
        public:
            Environment(){}
            virtual ~Environment(){}
        public:
            virtual void SetUp(){}
            virtual void TearDown(){}
        };

看的人应该明白。不解释。  
在初始化之前，如果使用测试环境，则要用宏SET_ENVIRONMENT将环境添加进去。  

### 其它  

还有一些与测试无关，但是与本框架相关的一些宏。  

| 宏         | 含义  |    
| ------------- |:-------------|  
| INIT_TEST |初始化框架，在使用调用|     
| RUN_ALL_TEST |运行测试	|   

编译好测试程序后，可执行程序接受一些选项，和GTEST的一样。  

| 选项         | 含义  |    
| ------------- |:-------------|  
| --help	 |输出帮助信息，仅显示英文，因为哥的工作的UNIX不支持中文。|     
| --list_tests	 |列出测试用例	|  
| --filter=POSTIVE_PATTERNS[-NEGATIVE_PATTERNS]	 |过虑，仅支持通佩符	|  

居然这个介绍花的时间比写这个测试框架的时间还长。  

### 示例  

更多示例参考GTEST。  

