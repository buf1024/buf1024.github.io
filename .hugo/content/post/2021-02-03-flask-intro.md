---
title: flask极简简介
date: 2021-02-03T19:10:00+08:00
draft: false
categories: [Python] 
tags: [Python]
---
todo:

- flask环境变量  -- FLASK_ENV, FLASK_DEBUG

----

flask可以支持热更新式的开发，PyCharm的设置方式是在`Run Configuration`上面勾上DEBUG选项。

假设已有以下代码:

```Python
from flask import Flask
app = Flask(__name__)
```

### 路由

路由使用`@app.route`装饰。路由如包括变量，格式是：`/user/<username>`或`<converter:variable_name>`。对应的函数可传入同名的参数， 如：

```Python
@app.route('/user/<username>')
def show_user_profile(username):
    return 'User %s' % escape(username)

@app.route('/post/<int:post_id>')
def show_post(post_id):
    return 'Post %d' % post_id
```

转换器包括:

| `string` | （缺省值） 接受任何不包含斜杠的文本 |
| -------- | ----------------------------------- |
| `int`    | 接受正整数                          |
| `float`  | 接受正浮点数                        |
| `path`   | 类似 `string` ，但可以包含斜杠      |
| `uuid`   | 接受 UUID 字符串                    |

路由重定向规则，如果一个路由，如`/projects/`，以`/`结尾，那么访问不带`/`的url将重定向到带`/`结尾的路由。反之，如果没有以`/`结尾，访问以`/`结尾的路由，将报404错误。

使用`url_for`函数，可以获取到对应的url，不用写死，如:

```Python
fORM flask import url_for

url_for('index')
```

### 模板

flask使用的是`jinja2`模板, 模板放置的目录默认位于项目的`templates`目录。如果使用的前后分离的模式，貌似用模板的几率并不是很高 :happy:。 但`jinja2`模板本身却够简单，使用flask里面的, `render_template`，即可渲染。如:

```html
<!doctype html>
<title>Hello from Flask</title>
{% if name %}
  <h1>Hello {{ name }}!</h1>
{% else %}
  <h1>Hello, World!</h1>
{% endif %}
```

### 请求数据

web请求的数据位于flask的`request`对象中，这个对象看起来是全局对象，在多线程的环境中不可以使用，实际上，这个是本地安全的。如:

```Python
from flask import request
```

### 响应数据

响应数据规则:

1. 如果视图返回的是一个响应对象，那么就直接返回它。
2. 如果返回的是一个字符串，那么根据这个字符串和缺省参数生成一个用于返回的 响应对象。
3. 如果返回的是一个字典，那么调用 `jsonify` 创建一个响应对象。
4. 如果返回的是一个元组，那么元组中的项目可以提供额外的信息。元组中必须至少 包含一个项目，且项目应当由 `(response, status)` 、 `(response, headers)` 或者 `(response, status, headers)` 组成。 `status` 的值会重载状态代码， `headers` 是一个由额外头部值组成的列表 或字典。
5. 如果以上都不是，那么 Flask 会假定返回值是一个有效的 WSGI 应用并把它转换为 一个响应对象。

如果想要在视图内部掌控响应对象的结果，那么可以使用 `make_response()`函数。如:

```Python
@app.errorhandler(404)
def not_found(error):
    resp = make_response(render_template('error.html'), 404)
    resp.headers['X-Something'] = 'A value'
    return resp
```

### 会话，日志

会话`from flask import session`, 日志`app.logger.debug`
