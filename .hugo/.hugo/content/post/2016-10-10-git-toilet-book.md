---
title: 'git 版本控制'
date: 2016-10-10 14:46:23
aliases: [/2016/10/10/git-toilet-book/]
categories: [vcs]
tags: [git, vcs]
---

### 简介
git，是目前最流行的版本控制系统，没有之一。git作为分布式版本控制系统，与其他集中式版本控制系统(如svn)的一个重要区别是，git `clone`的是整个完整版本仓库，在git目录中`.git`目录为仓库目录，这里记录了仓库的所有信息。git对于每一个版本，存储的是整个系统的一个快照(未改变内容，使用指针引用)，而不是版本之间的差异。因为这种存储方式，git在创建分支时非常快，只需要简单的用指针引用对应的版本即可(典型的空间换时间)。git的存储数据，提交信息，分支信息，标签信息等都使用`SHA1`计算校验和以及用作引用使用，这使得git有非常强的保持完整性的特征。在git的概念中，文件分为**未跟踪**和**已跟踪** 两种，对于**已跟踪**的文件，存在三种状态：**已提交(committed)**，**已修改(modified)**，**已暂存(staged)**。这三种状态分别对应三个工作区域：git仓库，工作目录，暂存区域。
git使用`git config`来进制配置git。git的配置存在于三个地方：/etc/gitconfig(针对所有用户的配置，使用--system选项读写), ~/.gitconfig或~/.config/git/config(针对当前用户，使用--global读写), git项目的.git/config(针对当前项目)。三个配置的地方，最细一级的配置覆盖上一级的配置。对于新安装的git，最基本需要配置`user.name`和`user.email`这两个配置，因为git项目的几乎所有的操作都会用的这两个信息。

### 基本操作
`git config`读取或写入配置。
`git help`查看帮助。

`git init`或`git clone [url] [alias]`初始化git仓库,前者为创建一个新的git仓库，后者为克隆一个已有的仓库。初始化仓库会创建一个`.git`的目录，仓库的默认分支为`master`，远程仓库的默认为`origin`。
`git status`查看当前工作目录文件的状态，并提示相关操作。简洁显示方式为`git status -s`或`git status --short`
`git add` 将文件添加到暂存区，以待下一步进行提交。若要取消暂存区文件的提交，则可用`git reset HEAD`进行取消。若想丢弃本地目录修改，则可使用`git checkout -- `。`git add -i`加上这个参数后，可以进行交互式操作。对于无需要跟踪或文件或目录，在git项目目录增加`.gitignore`文件，`gitignore`文件格式：1) 所有空行或者以 # 开头的行都会被 Git 忽略。 2) 可以使用标准的 glob 模式匹配。3) 匹配模式可以以(/)开头防止递归。4) 匹配模式可以以(/)结尾指定目录。5) 要忽略指定模式以外的文件或目录,可以在模式前加上惊叹号(!)取反。
`git diff`可查看未暂存区和暂存去文件的差异，如果想查看暂存区和上次提交的仓库差异，使用`git diff --staged`或`git diff --cached`。可使用`git difftool`使用差异分析软件分析差异。
`git commit`将暂存区内容提交到仓库。如果需要跳过暂存区，直接提交到仓库，使用`git commit -a`，该选项只针对已经跟踪的文件有效。如果提交时，发现少提交了几个文件或做了修改，想修正上次提交信息，进行相关操作后，使用`git commit -amend`。
`git rm`将仓库和本地的文件删掉，如果需要保存本地文件，则可使用`git rm --cached`，删除的文件将不再纳入版本控制。
`git mv`将文件改名，只是改变`tree`对象的信息。
`git clean`清清理工作目录，可使用`git clean -i`选项交互操作。
`git log`查看提交历史，支持多种参数格式。
`git tag`打标签，有两种类型标签，轻量级的标签，和带附注的。删除标签使用`git tag -d <标签明>`。分享标签到远程分支可使用`git push origin <标签名>`或`git push origin --tags`将所有的标签都推送。如果需要删除远程标签可使用`git push origin :refs/tags/<标签名>`。标签并不能像分支一样被`checkout`，如需要，则需要先创建标签对应的分支。
`git remote add <shortname> <url>`增加远程仓库。`git remote -v`查看远程仓库。`git remote show origin`可以查看到更加详细的信息。`git remote rename`远程仓库重命名。`git remote rm`删除远程仓库。`git fetch`抓取远程仓库，但并不进行合并。`git pull`抓取远程仓库，并进行合并。`git push`提交本地修改到远程仓库。
`git reflog` 显示`HEAD`引用的变化日志。
`git stash`，如果不想对当前的分支进行提交，但是又想切换到其他分支，可以使用该命令，改命令是将当前的修改先保存起来。`git stash list`可以显示存起来的内容。`git stash apply`进行恢复。

分支被称之为git的杀手锏，因为它创建一个分支出奇的快。创建分支使用`git branch <分支名>`(不带参数为列车所有分支)，创建分支即是创建一个引用指向一个`commit`对象，所以特别快。切换分支使用`git checkout <分支名>`，切换分支，即是简单的将`HEAD`指针指向对应的分支，所以也非常快。
远程分支使用`git remote show <remote name>`可以查看，远程分支跟踪是对远程分支状态的引用。向远程仓库分享分支使用`git push origin <分支名>`，如果不想远程分支和本地分支有同样的名称，使用`git push origin <本地分支名:远程分支名>`。当从远程仓库抓取远程分支时，git不会自动在本地创建远程分支的副本。使用`git checkout -b <本地分支名> <远程分支名>`,从远程分支创建分支。跟踪一个分支使用`git checkout --track <远程分支名>`，对已有的分支进行跟踪使用`git branch -u <远程分支名>`(或`--set-up-stream-to`选项)，这样可以`git pull`时直接从远程分支抓取。删除一个远程分支`git push origin --delete <分支名>`。
分支合并的方式有两种，一种是`git merge`，另外一种是`git rebase`。对于任何一种方式，都可以加上`--abort`参数进行合并的取消。先看第一种方式，`git merge`:
对于要合并的分支在同一条线上，被合并的分支比较前，这种情况比较简单。如：
![git fast-forward](/img/vcs/git-merge-fastforward.png)
将`hotfix`合并到`master`分支：`git merge master hotfix`。由于`hotfix`在`master`前，直接将`master`指向`hotfix`所值的`commit`对象即可，这种合并方式称为`fast forward`。合并后：
![git fast-forward 完成](/img/vcs/git-merged-fastforward.png)
对于要合并的分支不在同一条线上，这种方式相对复杂点。如：
![git 合并](/img/vcs/git-merge-normal.png)
将`iss53`合并到`master`分支：`git merge master iss53`。git需要计算两个分支的共同祖先，和各个分支的`HEAD`指针，然后做3方的对比，产生一个新的快照。合并后：
![git 合并完成](/img/vcs/git-merged-normal.png)
另外一种合并的方式是：`git rebase`。由以上`merge`可看出，由多少条分支就由多少条历史，`rebase`的目的是为了有更少更清晰的提交历史。如:
![git rebase](/img/vcs/git-rebase.png)
先对`experiment`分进行`rebase`：`git rebase experiment master`。git取出C4和C3的补丁，然后在C3那里进行一次合并，生成快照C4'，再在`master`分支上面进行一次`fast forward`合并：`git merge master experiment`。合并后：
![git rebase完成](/img/vcs/git-rebased.png)
可以看到，进行`rebase`后，分支信息没有了。所以，请注意，**千万不要在共同协作的仓库里面使用`rebase`**
合并的过程中，不可避免的产出冲突。当合并产生冲突时，`git status`可以看到冲突的状态。在冲突的文件里面，git会在冲突的文件中加入标志的冲突标记，需要手工解决冲突，并进行提交方可认为冲突解决了。对于冲突的文件，git使用一种特殊语法查看冲突文件，Stage 1 是它们共同的祖先版本,stage 2 是你的版本,stage 3 来自于 MERGE_HEAD,即你将要合并入的版本(“theirs”)。如：`git show :1:test.yy`, `git show :2:test.yy`和`git show :3:test.yy`分别显示公共祖先版本，本地目录版本，和将要合并的版本。当合并后，使用`git diff --ours`比较合并文件和本地文件差异，`git diff --theirs`比较合并后文件和要合并的版本的差异。也可以使用`git mergetool`启动外部合并工具进行合并。

注意，这里的基本操作不包括git的全部。

### 内部原理
git又称为内容寻址系统，因为它计算出“内容”的SHA1，然后用这个SHA1作为键来索引内容。对于一个新建的git仓库目录，它的仓库目录`.git`目录如下：

        HEAD config* description hooks/
        info/ objects/ refs/
        
对于一个已经存在的目录，还可能存在`COMMIT_EDITMSG FETCH_HEAD ORIG_HEAD`等文件(含义为最近一次提交信息，`git fetch`远程仓库的`HEAD`，远程参考的`HEAD`)或其他目录。 `config`目录包括针对仓库的配置信息，`description`是提供给GitWeb程序使用的，`HEAD`文件标示当前分支的`HEAD`指针，`index`包括暂存区的数据，`objects/`目录包括了仓库数据信息，`refs/`目录包括数据提交对象(如分支，标签，远程仓库标签分支，head引用等)的引用。在git内部，包括4种对象：`blob tree commit tag`。

- `blob`对象
 存储文件内容数据的`blob`对象。使用git的底层命令`hash-object`可以写入命令，`cat-file`可以读回，如：
 
        heidong@HEIDONG:~/tmp/mygit-repo$ echo "hello, git world!" | git hash-object -w --stdin
        76f2ae3955bfd0b4e9e6260a0e48a001c9336f2b
        heidong@HEIDONG:~/tmp/mygit-repo$ git cat-file -p 76f2ae3955bfd0b4e9e6260a0e48a001c9336f2b
        hello, git world!
        heidong@HEIDONG:~/tmp/mygit-repo$ git cat-file -t 76f2ae3955bfd0b4e9e6260a0e48a001c9336f2b
        blob
        heidong@HEIDONG:~/tmp/mygit-repo$ ls .git/objects/76/f2ae3955bfd0b4e9e6260a0e48a001c9336f2b 
        .git/objects/76/f2ae3955bfd0b4e9e6260a0e48a001c9336f2b
 
 `blob`对象的存储也是比较简单的，git为每个对象添加一个头信息，存储的格式为：头信息+内容。头信息的格式为：`blob 内容长度\0`。git将这些信息组装完毕后，进行压缩，然后存储的`objects`目录里面(在目录内部，取SHA1的前两个字母作为父目录，后面字母作为子目录文件名)。
 
- `tree`对象
 `blob`对象存储的是数据的内容，并没有存储文件的文件名，权限等信息。git采用的解决办法是类似*NIX系统的文件系统类似的方式，类似于`inode`，但有所简化，包括了文件名称，类型，权限，其他`tree`对象，`blob`对象引用等等。
 
- `commit`对象
 当保存一个仓库的快照时(提交)，需要标明提交者信息，提交信息，提交的文件文件等，这是通过提交对象提交的。`commit`对象通常指向一个`tree`对象，`commit`对象可供`git log`提取提交信息。

- `tag`对象
 `tag`对象类似于`commit`对象，与`commit`对象不同的是，`tag`对象通常指向一个`commit`对象。`tag`对象如果是轻量级标签，直接指向`commit`对象，否则，带上提交信息，注解信息等，指向一个`commit`对象。

git作为内容寻址系统，键为SHA1，要记住这个键或者记住这个键的一部分，是非常困难的，git的解决办法是采用名字引用的方式，这个名字引用的数据就存在`refs`目录下面。如：

        heidong@HEIDONG:~/tmp/mygit-repo/.git$ find refs/ -print
        refs/
        refs/tags
        refs/tags/v1.0
        refs/heads
        refs/heads/master
        refs/heads/devel

经过历次操作后，假设一下图:
![git 原理](/img/vcs/git-internal.png)
该图的含义是(注意，这里每次提交，"master"指针总指向最新提交)：
1. 创建文件"test.txt"，其内容为"version 1"，并提交到仓库，提交信息为"first commit"
2. 创建文件"new.txt"，其内容为"new file"，修改"test.txt"文件的内容为"version 2"，并提交到仓库，提交信息为"second commit", 提交后，创建分支"test"。
3. 创建目录"bak"，"bak"目录包含文件"test.txt"，文件内容为"version 1"

如果仓库中存在一个比较大的文件，而每次进行修改至少修改那么丁点数据的话，那么在git仓库里面将会保存大量的快照，比较占用空间。git使用`git gc`命令，进行仓库的压缩，把对象进行压缩增量存储的方式，并把相关信息存储在`objects/info/ objects/pack/`目录下面。

### 其他
1. 关某些系统中午乱码解决办法
 1.1 git配置修改
         
        [gui]
        encoding = utf-8 #代码库统一用urf-8,在git gui中可以正常显示中文
        [i18n]
        commitencoding = GB2312 #log编码，window下默认gb2312,声明后发到服务器才不会乱码
        [svn]
        pathnameencoding = GB2312 #支持中文路径

 1.2 /etc/git-completion.bash修改
       
        alias ls='ls --show-control-chars --color=auto'  #ls能够正常显示中文

 1.3 /etc/inputrc修改

        set output-meta on   #bash中可以正常输入中文
        set convert-meta off

 1.4 /etc/profile修改

        export LESSCHARSET=utf-8   #$ git log 命令不像其它 vcs 一样，n 条 log 从头滚到底，它会恰当地停在第一页，按 space 键再往后翻页。这是通过将 log 送给 less 处理实现的。以上即是设置 less 的字符编码，使得 $ git log 可以正常显示中文。
        
2. 使用配置

        [alias]
	    lg = log --color --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%ce>%Creset' --abbrev-commit #带颜色输出日志
        # 简写
        st = status -s
        br = branch
        co = checkout
        cm = commit

3. github上fork项目，如何同步原始的项目

```
1. 首先要先确定一下是否建立了主repo的远程源：
   git remote -v
2. 如果里面只能看到你自己的两个源(fetch 和 push)，那就需要添加主repo的源：
   git remote add upstream URL
   git remote -v
3. 然后你就能看到upstream了。如果想与主repo合并：
   git fetch upstream
   git merge upstream/master
```

完。

