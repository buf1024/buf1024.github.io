---
title: sqlalchemy orm 概览
date: 2014-12-28 21:08:34
aliases: [/2014/12/28/sqlalchemy-orm-overview/]
categories: [orm]
tags: [python, sqlalchemy, orm]
---

#### ***SQLAlchemy*** 概述
在`python`的`orm`世界里，[SQLAlchemy](www.sqlalchemy.org),无疑是数据库工具链里面的一个重量级的工具。SQLAlchemy简略构架图如下:  
![SQLAlchemy简略构架图](/img/python/sqlalchemy-orm-overview_arch_small.png)  

如图所述，SQLAlchemy主要包括两个大方面: **CORE**和**ORM**，其中**CORE**里面最重要的是**SQL Expression Language(SQL EL)**, **ORM**是建立在**SQL EL**之上，而**SQL EL**通过**ENGINE**,**DIALECT**建立在具体的**DBAPI**之上。  

##### 数据库连接
使用[SQLAlchemy](www.sqlalchemy.org)，必须关联一个数据库后端，后端使用`create_egine`创建，如：
```python   
    >>> from sqlalchemy import * #导入sqlalchemy所有包
    >>> engine = create_engine("sqlite:///:memory:", echo=True) #绑定数据库后端
```

`create_engine`通过不同的[DB URL](http://docs.sqlalchemy.org/en/rel_0_9/core/engines.html#database-urls)来识别不同数据库后端，数据库接口必须是符合[python dbapi](http://docs.sqlalchemy.org/en/rel_0_9/glossary.html#term-dbapi)规范的。需要注意的是，`create_engine`并不实际和数据库进行连接，等到第一次执行[Engine.execute](http://docs.sqlalchemy.org/en/rel_0_9/core/connections.html#sqlalchemy.engine.Engine.execute)或[Engine.connect](http://docs.sqlalchemy.org/en/rel_0_9/core/connections.html#sqlalchemy.engine.Engine.connect)显式连接时，才进行真正的数据库连接。  

##### 库表数据表示
[SQLAlchemy](www.sqlalchemy.org)有3种方式表示关系数据库的库表：
- 用户定义的`Table`对象
- 使用`declerative`基类
- 从现有的数据里面反射建立

##### 数据类型关系映射
![SQLAlchemy数据关系映射](/img/python/sqlalchemy-datatype.png)

#### ***SQLAlchemy Core***
**SQLAlchemy EL**提供了用`python`构建数据库对象的方式，而且这些`python`对象是数据库中立的，**EL**对数据操作也是数据库中立的。  
[SQLAlchemy](www.sqlalchemy.org)使用的**DTATBASE METADATA**和内部数据库结构绑定，能加速[SQLAlchemy](www.sqlalchemy.org)对数据库访问。如:  
```python           
    from sqlalchemy import *
    engine = create_engine("sqlite:///test.db", echo=True)
    metadata = MetaData()
    user = Table('user', metadata,
      Column('id', Integer, primary_key=True),
      Column('name', String(64)),
      Column('sex', String(2)),
    )
    
    address = Table('address', metadata,
      Column('id', Integer, primary_key=True),
      Column('user_id', None, ForeignKey('user.id')),
      Column('address', String(256), nullable=False)
    )
    metadata.create_all(engine)
```
执行后输出如下:  
```             
    ……忽略无关内容
    CREATE TABLE user (
            id INTEGER NOT NULL,
            name VARCHAR(64),
            sex VARCHAR(2),
            PRIMARY KEY (id)
    )

    2014-12-31 12:21:45,555 INFO sqlalchemy.engine.base.Engine ()
    2014-12-31 12:21:45,704 INFO sqlalchemy.engine.base.Engine COMMIT
    2014-12-31 12:21:45,707 INFO sqlalchemy.engine.base.Engine
    CREATE TABLE address (
            id INTEGER NOT NULL,
            user_id INTEGER,
            address VARCHAR(256) NOT NULL,
            PRIMARY KEY (id),
            FOREIGN KEY(user_id) REFERENCES user (id)
    )
    
    2014-12-31 12:21:45,710 INFO sqlalchemy.engine.base.Engine ()
    2014-12-31 12:21:45,716 INFO sqlalchemy.engine.base.Engine COMMIT
```
`create_engine`会根据数据库表名而决定是否需要在数据库创建库表，如果通过`metadata`增加库表，数据库如果没有该库表的话，就会创建。但如果修改列或在`metadata`删除表，则不受影响，不会执行。另外还有另外一种方法描述库表：**TABLE REFLECTION**，在已有的数据库的基础上通过反射获取库表的描述，这种方法虽然方便，但是可控程度稍低。具体可参考[Refelction Database Objects](http://docs.sqlalchemy.org/en/rel_0_9/core/reflection.html)。  

##### ***SQLAlchemy Core*** 数据操作 -- 插入
数据的插入可通过`table.insert()`获取**INSERT**对象，然后通过[SQLAlchemy](www.sqlalchemy.org)获取`Connection`对象来执行，这是最直观的方式，默认是一执行完毕就提交数据库的。**INSERT**对象式可以通过绑定变量或者在`Connection`对象绑定变量构建数据。`Connection`可以执行一条或多条数据，如：  
```python         
    conn = engine.connect()
    inst = user.insert()
    rst = conn.execute(inst, id = 1, name="hello", sex="F")
    
    conn.execute(address.insert(), [ 
       {'user_id': 1, 'address' : 'Guangdong'},
       {'user_id': 1, 'address' : 'Beijing'},
       {'user_id': 1, 'address' : 'Shanghai'},
       {'user_id': 1, 'address' : 'Hongkong'},
    ])
```
结果如下：
```
    sqlite> select a.*, b.* from user a left join address b on b.user_id = a.id;
    1|hello|F|1|1|Guangdong
    1|hello|F|2|1|Beijing
    1|hello|F|3|1|Shanghai
    1|hello|F|4|1|Hongkong
```    

如果需要事务控制，则需要获取[Transection](http://docs.sqlalchemy.org/en/rel_0_9/core/connections.html#sqlalchemy.engine.Transaction)对象，事务一般需要在`try/except`块中控制，如:
                
    try:
        addr_inst = address.insert()
    
        r = conn.execute(addr_inst, [ 
           {'user_id': 1, 'address' : 'Guangdong'},
           {'user_id': 1, 'address' : 'Beijing'},
           {'user_id': 1, 'address' : 'Shanghai'},
           {'user_id': 1, 'address' : 'Hongkong'},
        ])
        r.close()
        trans.commit()
    except:
        trans.rollback()
                
  同理，数据的查询可通过`table.select()`获取**SELECT**对象。但这种方法只能针对一张表，如果用使用多张表或者选择不同对象，则需要使用`select`函数。如：  
                
    s = select([user, address]).where(user.c.id == address.c.user_id)
    r = conn.execute(s)
    
    for t in r:
        print t
        
    print 'xxx'
    s = user.select().where(user.c.id == 2)
    r = conn.execute(s)
    for t in r:
        print t
    结果如下：
    2014-12-31 15:29:07,825 INFO sqlalchemy.engine.base.Engine SELECT user.id, user.
    name, user.sex, address.id, address.user_id, address.address
    FROM user, address
    WHERE user.id = address.user_id
    2014-12-31 15:29:07,825 INFO sqlalchemy.engine.base.Engine ()
    (1, u'ZhangSan', u'M', 1, 1, u'Guangdong')
    (1, u'ZhangSan', u'M', 2, 1, u'Beijing')
    (1, u'ZhangSan', u'M', 3, 1, u'Shanghai')
    (1, u'ZhangSan', u'M', 4, 1, u'Hongkong')
    xxx
    2014-12-31 15:29:07,828 INFO sqlalchemy.engine.base.Engine SELECT user.id, user.
    name, user.sex
    FROM user
    WHERE user.id = ?
    2014-12-31 15:29:07,828 INFO sqlalchemy.engine.base.Engine (2,)
    (2, u'LiShi', u'M')
                
  `select`函数需要的参数是列表，需要选择的结果集放到这里面，具体的列可以通过`table.c.colum`的方式来获取。为了一致性处理，通常对于`select`出来的结果集，如果不需要使用，还需调用结果集的`close`方法来关闭，从而释放数据库资源。这里里例子里面包含了`where`**CLAUSE**，这是个布尔表达式，使用的是`python`而非`sql`的语法，详细可参考[ColumnElement](http://docs.sqlalchemy.org/en/rel_0_9/core/sqlelement.html#sqlalchemy.sql.expression.ColumnElement)和[SelectTable](http://docs.sqlalchemy.org/en/rel_0_9/core/selectable.html)。[SQLAlchemy](www.sqlalchemy.org)的选择运算符包括and_, or_, not_, group_by,func.sum聚集函数等，使用这些基本上可以构建日常使用的查询语句。如果还不行，还可以像普通`sql`一样，直接写，如：  
                
    s = text("select * from user where id = :id")
    r = conn.execute(s, id=1)
    for t in r:
        print t
    结果：
    2014-12-31 16:00:32,207 INFO sqlalchemy.engine.base.Engine select * from user wh
    ere id = ?
    2014-12-31 16:00:32,207 INFO sqlalchemy.engine.base.Engine (1,)
    (1, u'ZhangSan', u'M')
                  
  关于查询再看看**JOIN**和**SUBQUERY**。

#### **SQLALchemy ORM**

  
