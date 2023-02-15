---
title: 'ld常用特别选项'
date: 2016-01-19 11:12:36
categories: [os]
tags: [os, minix3]
---
链接器ld的一些特别选项，编译OS时有用到。

-N

       --omagic

           Set the text and data sections to be readable and writable.  Also, do not page-align the data segment, and disable linking against shared libraries.  If the output format supports Unix style

           magic numbers, mark the output as "OMAGIC". Note: Although a writable text section is allowed for PE-COFF targets, it does not conform to the format specification published by Microsoft.

致命text段和data段可读写，同事关闭data段页面对齐以及禁止链接动态库

       --no-omagic

           This option negates most of the effects of the -N option.  It sets the text section to be read-only, and forces the data segment to be page-aligned.  Note - this option does not enable linking

           against shared libraries.  Use -Bdynamic for this.

-e entry

       --entry=entry

           Use entry as the explicit symbol for beginning execution of your program, rather than the default entry point.  If there is no symbol named entry, the linker will try to parse entry as a number,

           and use that as the entry address (the number will be interpreted in base 10; you may use a leading 0x for base 16, or a leading 0 for base 8).

           指明程序入口点

--section-start=sectionname=org

           Locate a section in the output file at the absolute address given by org.  You may use this option as many times as necessary to locate multiple sections in the command line.  org must be a

           single hexadecimal integer; for compatibility with other linkers, you may omit the leading 0x usually associated with hexadecimal values.  Note: there should be no white space between

           sectionname, the equals sign ("="), and org.

-Tbss=org

-Tdata=org

-Ttext=org

           Same as --section-start, with ".bss", ".data" or ".text" as the sectionname.

 指定段起启位置