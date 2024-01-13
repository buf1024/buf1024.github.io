---
title: git推送多个远程分支
date: 2017-07-05 20:56:18
categories: [git]
tags: [git, vcs]
---

1. 查看.git/config

```shell
[core]
 repositoryfORMatversion = 0
 filemode = true
 bare = false
 logallrefupdates = true
 ignorecase = true
 precomposeunicode = true
[remote "origin"]
 url = git@gitee.com:heidonglgc/k-map.git
 fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
 remote = origin
 merge = refs/heads/master
```

2. 增加一个名为mirror的远程仓库后

```shell
^_^@/Users/luoguochun/privt/proj/k-map]$ git remote add mirror git@e.coding.net:toyent/k-map/k-map.git  
^_^@/Users/luoguochun/privt/proj/k-map]$ cat .git/config
[core]
 repositoryfORMatversion = 0
 filemode = true
 bare = false
 logallrefupdates = true
 ignorecase = true
 precomposeunicode = true
[remote "origin"]
 url = git@gitee.com:heidonglgc/k-map.git
 fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
 remote = origin
 merge = refs/heads/master
[remote "mirror"]
 url = git@e.coding.net:toyent/k-map/k-map.git
 fetch = +refs/heads/*:refs/remotes/mirror/*
^_^@/Users/luoguochun/privt/proj/k-map]$ git remote -v
mirror git@e.coding.net:toyent/k-map/k-map.git (fetch)
mirror git@e.coding.net:toyent/k-map/k-map.git (push)
origin git@gitee.com:heidonglgc/k-map.git (fetch)
origin git@gitee.com:heidonglgc/k-map.git (push)
```

3. pull/push操作

```shell
git pull origin master 
git push origin master 
git pull mirror master 
git push mirror master 
```

4. 一次要打两次命令不爽，合并一条push

```shell
^_^@/Users/luoguochun/privt/proj/k-map]$ git remote set-url --add origin git@e.coding.net:toyent/k-map/k-map.git
^_^@/Users/luoguochun/privt/proj/k-map]$ cat .git/config 
[core]
 repositoryfORMatversion = 0
 filemode = true
 bare = false
 logallrefupdates = true
 ignorecase = true
 precomposeunicode = true
[remote "origin"]
 url = git@gitee.com:heidonglgc/k-map.git
 fetch = +refs/heads/*:refs/remotes/origin/*
 url = git@e.coding.net:toyent/k-map/k-map.git
[branch "master"]
 remote = origin
 merge = refs/heads/master
[remote "mirror"]
 url = git@e.coding.net:toyent/k-map/k-map.git
 fetch = +refs/heads/*:refs/remotes/mirror/*
 
^_^@/Users/luoguochun/privt/proj/k-map]$ git remote -v
mirror git@e.coding.net:toyent/k-map/k-map.git (fetch)
mirror git@e.coding.net:toyent/k-map/k-map.git (push)
origin git@gitee.com:heidonglgc/k-map.git (fetch)
origin git@gitee.com:heidonglgc/k-map.git (push)
origin git@e.coding.net:toyent/k-map/k-map.git (push)
```

意思是多个url指向同一个orgin，这样就可以一次push到多个仓库。
