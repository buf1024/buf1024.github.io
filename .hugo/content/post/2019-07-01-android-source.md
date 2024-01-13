---
title: "android国内源"
date: 2019-07-01T15:12:00+08:00
draft: false
categories: [Misc] 
tags: [Android]
---

如果是Flutter框架，同时要flutter框架修改文件

`flutter\packages\flutter_tools\gradle\flutter.gradle`

grep找出build.gradle文件

凡是和google相关的，全部替换为

```properties
maven{ url 'https://maven.aliyun.com/repository/central/'}
maven{ url 'https://maven.aliyun.com/repository/public/'}
maven{ url 'https://maven.aliyun.com/repository/google/'}
maven{ url 'https://maven.aliyun.com/repository/gradle-plugin/'}
maven{ url 'https://maven.aliyun.com/repository/spring/'}
maven{ url 'https://maven.aliyun.com/repository/spring-plugin/'}
maven{ url 'https://maven.aliyun.com/repository/grails-core/'}
maven{ url 'https://maven.aliyun.com/repository/apache-snapshots/'}
```

或者万能的解决办法：

新建~/.gradle/init.gradle

(此法貌似无效)

```properties

allprojects{
    repositories {
        def ALIYUN_REPOSITORY_URL = 'https://maven.aliyun.com/repository/central'
        def ALIYUN_JCENTER_URL = 'https://maven.aliyun.com/repository/public'
 def ALIYUN_GOOGLE_URL = 'https://maven.aliyun.com/repository/google'
        def ALIYUN_GOOGLE_PLUGIN_URL = 'https://maven.aliyun.com/repository/gradle-plugin'
        def ALIYUN_SPRING_URL = 'https://maven.aliyun.com/repository/spring'
  def ALIYUN_SPRING_PLUGIN_URL = 'https://maven.aliyun.com/repository/spring-plugin'
  def ALIYUN_GRAILS_CORE_URL = 'https://maven.aliyun.com/repository/grails-core'
  def ALIYUN_APACHE_SNAPSHOTS_URL = 'https://maven.aliyun.com/repository/apache-snapshots'
        all { ArtifactRepository repo ->
            if(repo instanceof MavenArtifactRepository){
                def url = repo.url.toString()
                if (url.startsWith('https://repo1.maven.org/maven2/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_REPOSITORY_URL."
                    remove repo
                }
                if (url.startsWith('https://jcenter.bintray.com/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_JCENTER_URL."
                    remove repo
                }
  if (url.startsWith('https://maven.google.com/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_GOOGLE_URL."
                    remove repo
                }
  if (url.startsWith('https://plugins.gradle.org/m2/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_GOOGLE_PLUGIN_URL."
                    remove repo
                }
  if (url.startsWith('https://repo.spring.io/libs-milestone/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_SPRING_URL."
                    remove repo

                }
  if (url.startsWith('https://repo.spring.io/plugins-release/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_SPRING_PLUGIN_URL."
                    remove repo
                }
  if (url.startsWith('https://repo.grails.org/grails/core')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_GRAILS_CORE_URL."
                    remove repo
                }
  if (url.startsWith('https://repository.apache.org/snapshots/')) {
                    project.logger.lifecycle "Repository ${repo.url} replaced by $ALIYUN_APACHE_SNAPSHOTS_URL."
                    remove repo
                }
            }
        }

        maven {
                url ALIYUN_REPOSITORY_URL
  url ALIYUN_JCENTER_URL
  url ALIYUN_GOOGLE_URL
  url ALIYUN_GOOGLE_PLUGIN_URL
  url ALIYUN_SPRING_URL
  url ALIYUN_SPRING_PLUGIN_URL
  url ALIYUN_GRAILS_CORE_URL
  url ALIYUN_APACHE_SNAPSHOTS_URL
        }
    }
}
```
