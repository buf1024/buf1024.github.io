---
title: gcov 简述
date: 2013-05-31 17:08:14
aliases: [/2013/05/31/Linux-gcov/]
categories: [Linux, GCC] 
tags: [C, GCC, TDD]
---

代码覆盖率是单元测试的一个指标，通常覆盖率越高，单元测试就做得更完备。（然而，覆盖率是不是和软件质量成正比关系呢？）gcov是GNU工具链中的一个重要的工具，虽然gcov是覆盖率很好的工具，但是gcov的更重要的应用是性能的调优。gcov通过监视程序的执行，从而确定某行代码有没有执行，执行了多少次。gcov的报告是基于文本的格式的，看起来是比较难看点。但是，有个叫lcov的工具，将gcov的报告格式转换为html的直观形式，后面介绍。  

gcov使用：  

如有以下代码：  

         1:  #include <stdio.h>
         2:   
         3:  void bubbleSort( int list[], int size )
         4:  {
         5:      int i, j, temp, swap = 1;
         6:   
         7:      while (swap) {
         8:   
         9:          swap = 0;
        10:   
        11:          for ( i = (size-1) ; i >= 0 ; i-- ) {
        12:   
        13:              for ( j = 1 ; j <= i ; j++ ) {
        14:   
        15:                  if ( list[j-1] > list[j] ) {
        16:   
        17:                      temp = list[j-1];
        18:                      list[j-1] = list[j];
        19:                      list[j] = temp;
        20:                      swap = 1;
        21:   
        22:                  }
        23:   
        24:              }
        25:   
        26:          }
        27:   
        28:      }
        29:   
        30:   
        31:  }
        32:   
        33:  int main()
        34:  {
        35:      int theList[10]={10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
        36:      int i;
        37:   
        38:      /* Invoke the bubble sort algorithm */
        39:      bubbleSort( theList, 10 );
        40:   
        41:      /* Print out the final list */
        42:      for (i = 0 ; i < 10 ; i++) { 
        43:          printf("%d\n", theList[i]);
        44:      }
        45:      if(i == 0){
        46:          printf("i = 0\n");
        47:      }else{
        48:          printf("i != 0\n");
        49:      }
        50:   
        51:  }

1. 编译程序是增加 -ftest-coverage -fprofile-arcs 选项。  

        [heidong@HEIDONGVM gcov]$ GCC -o bbsort bbsort.c -ftest-coverage -fprofile-arcs

生成.gcno文件。  

2. 执行程序，将生成.gcda文件，用gcov程序检查相应的源代码文件，将生成结果文件。  

        [heidong@HEIDONGVM gcov]$ ./bbsort 
        1 
        2 
        3 
        4 
        5 
        6 
        7 
        8 
        9 
        10 
        i != 0 
        [heidong@HEIDONGVM gcov]$ gcov bbsort.c 
        File‘bbsort.c’ 
        已执行的行数：95.24% (共 21 行) 
        bbsort.c：正在创建‘bbsort.c.gcov

3. 检查相应的结果文件  

        [heidong@HEIDONGVM gcov]$ cat bbsort.c.gcov 
                -:    0:Source:bbsort.c
                -:    0:Graph:bbsort.gcno
                -:    0:Data:bbsort.gcda
                -:    0:Runs:1
                -:    0:Programs:1
                -:    1:#include <stdio.h>
                -:    2:
                1:    3:void bubbleSort( int list[], int size )
                -:    4:{
                1:    5:    int i, j, temp, swap = 1;
                -:    6:
                4:    7:    while (swap) {
                -:    8:
                2:    9:        swap = 0;
                -:   10:
               22:   11:        for ( i = (size-1) ; i >= 0 ; i-- ) {
                -:   12:
              110:   13:            for ( j = 1 ; j <= i ; j++ ) {
                -:   14:
               90:   15:                if ( list[j-1] > list[j] ) {
                -:   16:
               45:   17:                    temp = list[j-1];
               45:   18:                    list[j-1] = list[j];
               45:   19:                    list[j] = temp;
               45:   20:                    swap = 1;
                -:   21:
                -:   22:                }
                -:   23:
                -:   24:            }
                -:   25:
                -:   26:        }
                -:   27:
                -:   28:    }
                -:   29:
                -:   30:
                1:   31:}
                -:   32:
                1:   33:int main()
                -:   34:{
                1:   35:    int theList[10]={10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
                -:   36:    int i;
                -:   37:
                -:   38:    /* Invoke the bubble sort algorithm */
                1:   39:    bubbleSort( theList, 10 );
                -:   40:
                -:   41:    /* Print out the final list */
               11:   42:    for (i = 0 ; i < 10 ; i++) { 
               10:   43:        printf("%d\n", theList[i]);
                -:   44:    }
                1:   45:    if(i == 0){
            #####:   46:        printf("i = 0\n");
                -:   47:    }else{
                1:   48:        printf("i != 0\n");
                -:   49:    }
                -:   50:
                1:   51:}
                -:   52:
        [heidong@HEIDONGVM gcov]$

可以看到某行执行了多少次，哪些行没有执行过（####标示）。gcov 还可以检查其他很多的信息，如分支，函数等，详细参考gcov的帮助文档，并测试之。  

对于文本格式，相信很多人的觉得不是很直观，于是便有了lcov这个工具，它可以算是gcov的前端工具，这样生成html文件，可以很直观的看到代码覆盖情况。  

lcov不是标准的unix/Linux工具，需要下载，地址是：ltp.sourceforge.net/coverage/lcov.php 注意要FQ才可以访问。  

使用方式：  

1. 执行完gcov的步骤后，执行下列命令：

        [heidong@HEIDONGVM gcov]$ lcov --capture --directory ./  --output-file bbsort.info 
        Capturing coverage data from ./ 
        Found gcov version: 4.4.6 
        Scanning ./ for .gcda files ... 
        Found 1 data files in ./ 
        Processing bbsort.gcda 
        Finished .info-file creation

2. 生成html文件：

        [heidong@HEIDONGVM gcov]$ genhtml bbsort.info --output-directory ./lcov/ 
        Reading data file bbsort.info 
        Found 1 entries. 
        Found common filename prefix "/home/heidong/tmp" 
        Writing .CSS and .png files. 
        Generating output. 
        Processing file gcov/bbsort.c 
        Writing directory view page. 
        Overall coverage rate: 
          lines......: 95.2% (20 of 21 lines) 
          functions..: 100.0% (2 of 2 functions

看下生成的html文件：
![html report](/img/gcov/gcov.jpg)  
完毕。  
