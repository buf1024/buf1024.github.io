---
title: 'gtest 小结'
date: 2011-12-05 12:00:00
aliases: [/2011/12/05/gtest-summary/]
categories: [unittest]
tags: [c,gtest] 
---
GTEST 是简单而且又非常实用的测试框架。下面关于GTEST的一些简单总结。  

1. 编译  
  在GNU系统下：  

        g++ -I${GTEST_DIR}/include -I${GTEST_DIR} -c ${GTEST_DIR}/src/gtest-all.cc        
    
        ar -rv libgtest.a gtest-all.o    
        
   在WINDOWS下：  
   GTEST在MSVC下面提供了相应的工程文件，直接用VS编译即可。  
2. 简单用法  
  在GNU系统下，直接包含GTEST的头文件，连接到GTEST库。

        g++ -I${GTEST_DIR}/include path/to/your_test.cc libgtest.a –lpthread -o your_test        

  在WINDOWS下直接连接到静态库。  
3. SetUp & TearDown
  GTEST提供了以下级别的SETUP和TEARDOW。  
  3.1 程序级别  
   程序级别的是指程序在启动的时候和结束的时候分别SETUP和TEARDOWN一次。其作用与全局变量类似。  
   第一步，继承类：  

        class Environment {
 
        public:
 
        // The d'tor is virtual as we need to subclass Environment.
 
        virtual ~Environment() {}
 
        // Override this to define how to set up the environment.
 
        virtual void SetUp() {}
 
        // Override this to define how to tear down the environment.
 
        virtual void TearDown() {}
 
        private:
 
        // If you see an error about overriding the following function or
 
        // about it being private, you have mis-spelled SetUp() as Setup().
 
        struct Setup_should_be_spelled_SetUp {};
 
        virtual Setup_should_be_spelled_SetUp* Setup() { return NULL; }
 
        };
                
   重写里面的SETUP和TEARDOWN方法。  
   第二步，调用Environment* AddGlobalTestEnvironment(Environment* env)，注意这个要在RUN_ALL_TESTS前调用。如  

        int main(int argc, char* argv[])
        {
            testing::AddGlobalTestEnvironment(new XXX Environment);
            testing::InitGoogleTest(&argc, argv);
            return RUN_ALL_TESTS();
        }

  3.2 Fixture 级别  
  Fixture 级别是指，对于在这FIXTURE里的所有TESTCASE，只调用一次SETUP和TEARDOWN。  
  在你的FIXTURE里，添加两个静态的函数SetUpTestCase和TearDownTestCase  

        class XXXTest : public ::testing::Test {
         protected:
          static void SetUpTestCase() {
          }
          static void TearDownTestCase() {
          }
        }

  3.3 TestCase 级别  
  TestCase 级别是指TESTCASE调用之前和调用后分别调用的。  
  在你的TESTCASE里，重写虚函数SetUp和TearDown即可。如。

        class XXXTest : public ::testing::Test {
         protected:
          virtual void SetUp() { ... }
          virtual void TearDown() { ... }
        }

4. Assertion  
  GTEST提供两种，一种是ASSERT_XXX和EXPECT_XXX。前者表示不继续执行TESTCASE，后者表示继续执行。  
  比如ASSERT_EQ, ASSERT_STREQ,EXPECT_EQ,EXPECT_TRUE等。  

