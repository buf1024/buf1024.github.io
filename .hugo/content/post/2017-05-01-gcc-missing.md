---
title: GCC不常见，但有用特别选项
date: 2017-05-01 22:10:31
categories: [GCC]
tags: [Http, C]
---
 -isystem dir

           Search dir for header files, after all directories specified by -I but before the

           standard system directories.  Mark it as a system directory, so that it gets the same

           special treatment as is applied to the standard system directories.  If dir begins

           with "=", then the "=" will be replaced by the sysroot prefix; see --sysroot and

           -isysroot.

标记 DIR 为系统DIR，搜索头文件顺序： 1. -I路径 2. -isystem DIR 3. 系统路径

--sysroot=dir

           Use dir as the logical root directory for headers and libraries.  For example, if the

           compiler nORMally searches for headers in /usr/include and libraries in /usr/lib, it

           instead searches dir/usr/include and dir/usr/lib.

           If you use both this option and the -isysroot option, then the --sysroot option

           applies to libraries, but the -isysroot option applies to header files.

标记DIR为头文件的根目录

-static

           On systems that support dynamic linking, this prevents linking with the shared libraries.

           On other systems, this option has no effect.

链接静态库

-M

          Instead of outputting the result of preprocessing, output a rule suitable for make describing the dependencies of the main source file.

          The preprocessor outputs one make rule containing the object file name for that source file, a colon, and the names of all the included files,

          including those coming from -include or -imacros command line options.

          Unless specified explicitly (with -MT or -MQ), the object file name consists of the name of the source file with any suffix replaced with object

          file suffix and with any leading directory parts  removed.  If there are many included files then the rule is split into several lines using \-newline.

          The rule has no commands.

          This option does not suppress the preprocessor's debug output, such as -dM.  To avoid mixing such debug output with the dependency

          rules you should explicitly specify the dependency output file

           with -MF, or use an environment variable like DEPENDENCIES_OUTPUT.  Debug output will still be sent to the regular output stream as nORMal.

           Passing -M to the driver implies -E, and suppresses warnings with an implicit -w.

生依赖文件，-E 为预编译，和-MF “文件”，生产依赖到文件，-MD生产依赖到，XXX.d文件，以上所描述几个，均为加快编译。

-O

-O1 Optimize.  Optimizing compilation takes somewhat more time, and a lot more memory for

           a large function.

           With -O, the compiler tries to reduce code size and execution time, without perfORMing

           any optimizations that take a great deal of compilation time.

           -O turns on the following optimization flags:

           -fauto-inc-dec -fcompare-elim -fcprop-registers -fdce -fdefer-pop -fdelayed-branch

           -fdse -fguess-branch-probability -fif-conversion2 -fif-conversion -fipa-pure-const

           -fipa-profile -fipa-reference -fmerge-constants -fsplit-wide-types -ftree-bit-ccp

           -ftree-builtin-call-dce -ftree-ccp -ftree-ch -ftree-copyrename -ftree-dce

           -ftree-dominator-opts -ftree-dse -ftree-forwprop -ftree-fre -ftree-phiprop -ftree-slsr

           -ftree-sra -ftree-pta -ftree-ter -funit-at-a-time

           -O also turns on -fomit-frame-pointer on machines where doing so does not interfere

           with debugging.

 -O2 Optimize even more.  GCC perfORMs nearly all supported optimizations that do not

           involve a space-speed tradeoff.  As compared to -O, this option increases both

           compilation time and the perfORMance of the generated code.

           -O2 turns on all optimization flags specified by -O.  It also turns on the following

           optimization flags: -fthread-jumps -falign-functions  -falign-jumps -falign-loops

           -falign-labels -fcaller-saves -fcrossjumping -fcse-follow-jumps  -fcse-skip-blocks

           -fdelete-null-pointer-checks -fdevirtualize -fexpensive-optimizations -fgcse

           -fgcse-lm -fhoist-adjacent-loads -finline-small-functions -findirect-inlining

           -fipa-sra -foptimize-sibling-calls -fpartial-inlining -fpeephole2 -fregmove

           -freorder-blocks  -freorder-functions -frerun-cse-after-loop -fsched-interblock

           -fsched-spec -fschedule-insns  -fschedule-insns2 -fstrict-aliasing -fstrict-overflow

           -ftree-switch-conversion -ftree-tail-merge -ftree-pre -ftree-vrp

           Please note the warning under -fgcse about invoking -O2 on programs that use computed

           gotos.

           NOTE: In Ubuntu 8.10 and later versions, -D_FORTIFY_SOURCE=2 is set by default, and is

           activated when -O is set to 2 or higher.  This enables additional compile-time and

           run-time checks for several libc functions.  To disable, specify either

           -U_FORTIFY_SOURCE or -D_FORTIFY_SOURCE=0.

-O3 Optimize yet more.  -O3 turns on all optimizations specified by -O2 and also turns on

           the -finline-functions, -funswitch-loops, -fpredictive-commoning, -fgcse-after-reload,

           -ftree-vectorize, -fvect-cost-model, -ftree-partial-pre and -fipa-cp-clone options.

-O0 Reduce compilation time and make debugging produce the expected results.  This is the

           default.

-Os Optimize for size.  -Os enables all -O2 optimizations that do not typically increase

           code size.  It also perfORMs further optimizations designed to reduce code size.

           -Os disables the following optimization flags: -falign-functions  -falign-jumps

           -falign-loops -falign-labels  -freorder-blocks  -freorder-blocks-and-partition

           -fprefetch-loop-arrays  -ftree-vect-loop-version

 -Ofast

           Disregard strict standards compliance.  -Ofast enables all -O3 optimizations.  It also

           enables optimizations that are not valid for all standard-compliant programs.  It

           turns on -ffast-math and the Fortran-specific -fno-protect-parens and -fstack-arrays.

-Og Optimize debugging experience.  -Og enables optimizations that do not interfere with

           debugging. It should be the optimization level of choice for the standard edit-

           compile-debug cycle, offering a reasonable level of optimization while maintaining

           fast compilation and a good debugging experience.

           If you use multiple -O options, with or without level numbers, the last such option is

           the one that is effective.

          优化等级

-fstack-protector

           Emit extra code to check for buffer overflows, such as stack smashing attacks.  This

           is done by adding a guard variable to functions with vulnerable objects.  This

           includes functions that call "alloca", and functions with buffers larger than 8 bytes.

           The guards are initialized when a function is entered and then checked when the

           function exits.  If a guard check fails, an error message is printed and the program

           exits

           检查栈溢出，相反的      -fno-stack-protector

-nostdinc

           Do not search the standard system directories for header files.  Only the directories

           you have specified with -I options (and the directory of the current file, if

           appropriate) are searched.

           不使用标准C头文件

-nostdinc++

           Do not search for header files in the C++-specific standard directories, but do still

           search the other standard directories.  (This option is used when building the C++

           library.)

          不使用标准C++头文件

 -Wall

           This enables all the warnings about constructions that some users consider

           questionable, and that are easy to avoid (or modify to prevent the warning), even in

           conjunction with macros.  This also enables some language-specific warnings described

           in C++ Dialect Options and Objective-C and Objective-C++ Dialect Options.

打开警告（并非所有），打开绝大部分，还要加-Wextral

-Wextra

           This enables some extra warning flags that are not enabled by -Wall. (This option used

           to be called -W.  The older name is still supported, but the newer name is more

           descriptive.)

           打开额外的警告，同-W（旧名）

-Werror

           Make all warnings into hard errors.  Source code which triggers warnings will be

           rejected.

警告即报错。

-w  Suppress all warnings, including those which GNU CPP issues by default.

关闭所有的错误警告

-Wa,option

           Pass option as an option to the assembler.  If option contains commas, it is split

           into multiple options at the commas

 传选项给汇编器
