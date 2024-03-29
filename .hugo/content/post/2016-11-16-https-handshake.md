---
title: 'ssl/tls 握手'
date: 2016-11-16 11:17:11
aliases: [/2016/11/16/https-handshake/]
categories: [TCP]
tags: [https, ssl/tls]
---

http协议是互联网的基石，然而http却是基于文本的协议，在网络传输中容易被窃听，冒充和篡改。如果对安全性要求相对高，那么一般使用https协议，https是在http上增加ssl/tls加密层，ssl/tls所有的信息都是加密的，而且有身份校验和证书支持。一般来说，应用开发者很少需要关注ssl/tls这一层，api库或应用服务器等中间件已经封装好了,只需要简单的调用和配置则可以使用ssl/tls的功能。正因为封装的太好了，所以很多人对于ssl/tls的细节稍微了解的并不是很多，当然也包括自己在内。这里简单探索一下ssl/tls建立的过程，只是简单的探索，不会太深入。

ssl/tls包括单向和双向认证，单向认证即是客户端认证服务端的，双向认证即是客户端和服务端互相认证。ssl/tls握手的过程大体可以描述以下：

        CLIENT                             SERVER
        1. Client Hello                    2. Server Hello
                                           3. Certificate
                                           4. Server Key Exchange
                                           5. Certificate Request(可选)
                                           6. Server Helo Done
        7. Certificate(可选)
        8. Client Key Exchange
        9. Certificate Verify(可选)
        10. Change Chiper Spec
        11. Encrypted Handshake Message
                                           11. New Session Ticket
                                           12. Change Chiper Spec
                                           13 Encrypted Handshake Message
        
        PS: 可选在指在单向认证中没有的，在双向认证中必须

下面我们通过抓包的了解下ssl/tls握手过程，抓包命令和访问https命令如下(相关报文: [https.cap](/raw/tcp/https.cap)，这是个单向认证的过程)：

       sudo tcpdump -i eth0 port 443 -s 0 -w https.cap
       wget https://www.baidu.com/favicon.ico

在wireshark里面，我们通过过滤器`ssl`，可只显示ssl的报文(`ip.addr == 163.177.151.110`，则可以显示完整的tcp交互过程)，过滤后的报文如下：
![ssl/tls握手](/img/tcp/ssl-wireshark.png "ssl/tls握手")

握手的过程发生在No.6 到No.16的帧上面。我们来详细了解一下。

1. Client Hello
 ![Client Hello](/img/tcp/ssl-client-hello.png "Client Hello")
 客户端向服务端发送加密通信请求，主要包括的一些信息：

 1) 支持的协议的版本
 2) 一个随机数(**Client Random**)，明文，用于生成加密秘钥
 3) 支持的加密算法和压缩算法
 4) 支持的一些ssl/tls扩展
 客户端的请求中并不包括域名，所以理论上，只能包括一个服务器，这对于支持多个虚拟主机的服务器来说，这并不方便，因为并不清楚向那个虚拟主机请求证书。所以，为了解决这个痛点，在Client Hello的报文中，扩展信息里面，增加了server_name这个扩展，如果上图示，图中显示的是令人不齿的又经常用来测试的baidu。

2. Server Hello, Certificate, Server Key Exchange, Server Hello Done
 ![Server Hello](/img/tcp/ssl-server-hello.png "Server Hello")
 服务端收到客户端的Client Hello后，向客户端发送Server Hello，主要包括的一些信息：

 1) 确认双方使用的协议版本
 2) 一个随机数(**Server Random**)，明文，用于生成加密秘钥
 3) 确认双方使用的加密算法和压缩算法
 4) 支持的一些ssl扩展

 ![Certificate](/img/tcp/ssl-certificate.png "Certificate")
 服务端发送Server Hello后，发送Certificate，向客户端发送证书信息

 ![Server Key Exchange](/img/tcp/ssl-server-key-exchange.png "Server Key Exchange")
 服务端发送Certificate后，发送Server Key Exchange，向客户客户端发送密钥信息。如果采用的是RSA算法，则不需要这一步。这里采用的是DH算法，所以发送的是DH算法服务端的参数。

 ![Server Hello Done](/img/tcp/ssl-server-hello-done.png "Server Hello Done")
 服务器向客户端发送Server Hello Done，表明所有的Server Hello信息发送完毕。

 PS.: 如果是双向认证，在Server Hello Done之前，还会发送Certificate Request，主要的含义是要求客户端提出证书，主要包括：

 1) 客户端应该提供的证书类型
 2) 服务端接收的证书列表

3. Client Key Exchange, Change Chiper SpeC, Encrypted Handshake Message
 客户端收到服务端的Server Hello Done后，就会对接收的的证书进行校验，如果发现证书不是可信机构签发的，或这域名等信息和证书不对应，浏览器则后弹出警告，由使用者确认是否继续。
 ![Client Key Exchange](/img/tcp/ssl-client-key-exchange.png "Client Key Exchange")
 Client Key Exchange将产生一个随机数(Pre Master Secret)，如果是采用RSA加密，则提取证书的公钥，用公钥对随机数进行加密，并发送到服务端。如果是采用的是DH算法，则和服务端发送的 Server Key Exchange类似，发送的是DH算法的参数。这样Pre Master Secret，就由双方的各种DH算法参数算出来，在传输的过程中不传输实际的Pre Master Secret，这样可以提高传输的安全性。至此，我们有了三个随机数，Client Random， Server Random和Pre Master Secret，由这三个随机数，我们可以算出另外一个随机数，Master Secret，之后，我们就使用这个Master Secret密钥进行加密传输。

 Change Chiper Spec 通知服务端，以后的报文采用加密的方式传输
 Encrypted Handshake Message 客户端的发送的第一个加密报文，内容是加密后的，所有接收到和发送的报文的摘要信息。

 PS.: 如果是双向认证，客户端还会向服务端发送 Certificate和Certificate Verify报文，Certificate报文包括客户端的证书，Certificate Verify包括客户端发送证书后所有握手过程报文的签名信息。服务端会对证书进行校验，如果发现证书不对，则直接终止ssl/tls连接。

4. New Session Ticket, Change Chiper SpeC, Encrypted Handshake Message
 ![New Session Ticket](/img/tcp/ssl-new-session-ticket.png "New Session Ticket")
 服务端收到客户端的加密报文后，进行解密和校验处理，最重要的是生成一个session ticket。这个ticket在异常的情况下比较有用
 Change Chiper Spec和客户端发送的含义一致，告知客户端，以后的报文采用加密的方式传输。
 Encrypted Handshake Message和客户端发送的含义一致，内容是加密后的，所有接收到和发送的报文的摘要信息。

至此ssl/tls四次握手完毕，握手成功后，我们就可以通过加密的方式传输报文。从上面的交互可看出，ssl/tls建立的过程需要多次交互，而且交互的过程中交互的数据量也比较大，成功建立一个ssl/tls链接需要几K的数据量。如果ssl/tls在传输的过程中由于网络等原因中断了，再进行4次握手，代价是很高的。所以为了解决这个痛点，在握手的第四步，服务端生成了New Session Ticket，网络中断后客户端可以通过发送这个报文，来重新建立链接ssl/tls链接，而不再需要4次握手。

简单的ssl/tls握手简单的探讨完了，下面附一下ssl/tls证书生成过程，作为平时参考：
 一般而言，扩展名以下的文件的含义：

* \*.key：RSA密钥文件
* \*.csr：证书请求文件，包括公钥等信息，通过签名后可生成证书文件。
* \*.crt, \*.cert：证书文件
* \*.pem：包含私钥和证书文件

1. 服务端生成证书

        1) 生成RSA密钥
        heidong@HEIDONG:~/tmp/cert$ openssl genrsa -des3 -out server.key 2048
        Generating RSA private key, 2048 bit long modulus
        ........................+++
        ....................................................................................................................+++
        e is 65537 (0x10001)
        Enter pass phrase for server.key:
        Verifying - Enter pass phrase for server.key:
        
        这样生成的密钥是有密码保护的，可去掉密码
        heidong@HEIDONG:~/tmp/cert$ openssl rsa -in server.key -out server.key
        Enter pass phrase for server.key:
        writing RSA key
        
        2) 生成证书请求文件
        heidong@HEIDONG:~/tmp/cert$ openssl req -new -key server.key -out server.csr
        You are about to be asked to enter infORMation that will be incorporated
        into your certificate request.
        What you are about to enter is what is called a Distinguished Name or a DN.
        There are quite a few fields but you can leave some blank
        For some fields there will be a default value,
        If you enter '.', the field will be left blank.
        -----
        Country Name (2 letter code) [AU]:CN
        State or Province Name (full name) [Some-State]:Guangdong
        Locality Name (eg, city) []:Guangzhou
        Organization Name (eg, company) [Internet Widgits Pty Ltd]:PAIDU, Ltd
        Organizational Unit Name (eg, section) []:Hello
        Common Name (e.g. server FQDN or YOUR name) []:www.paidu.com
        Email Address []:paidu@paidu.com

        Please enter the following 'extra' attributes
        to be sent with your certificate request
        A challenge password []:
        An optional company name []:
        
        在证书请求文件生成的过程中Common Name比较重要，必须和域名匹配
        有了证书请求文件后，就可以直接用该文件到CA认证中心请求证书文件，当然是要收费的。
        也可以生成自己测试用的证书
        
        3) 生成证书
        heidong@HEIDONG:~/tmp/cert$ openssl req -x509 -days 1024 -key server.key -in server.csr > server.crt 
        
        如果使用CA签名的方法生成证书，检查以下目录是否存在，如果步存在先新建
        1) mkdir -p ./demoCA/newcerts 
        2) touch demoCA/index.txt 
        3) touch demoCA/serial 
        4) echo 01 > demoCA/serial

        3.1) 生成CA证书
        heidong@HEIDONG:~/tmp/cert$ openssl req -new -x509 -keyout ca.key -out ca.crt 
        Generating a 2048 bit RSA private key
        ............................................................+++
        ........................................+++
        writing new private key to 'ca.key'
        Enter PEM pass phrase:
        Verifying - Enter PEM pass phrase:
        -----
        You are about to be asked to enter infORMation that will be incorporated
        into your certificate request.
        What you are about to enter is what is called a Distinguished Name or a DN.
        There are quite a few fields but you can leave some blank
        For some fields there will be a default value,
        If you enter '.', the field will be left blank.
        -----
        Country Name (2 letter code) [AU]:CN
        State or Province Name (full name) [Some-State]:Guangdong
        Locality Name (eg, city) []:Guangzhou
        Organization Name (eg, company) [Internet Widgits Pty Ltd]:PAIDU, Ltd
        Organizational Unit Name (eg, section) []:Hello
        Common Name (e.g. server FQDN or YOUR name) []:www.paidu.com
        Email Address []:paidu@paidu.com
        
        3.2) 用CA证书签名生成证书
        heidong@HEIDONG:~/tmp/cert$ openssl ca -in server.csr -out server.crt -cert ca.crt -keyfile ca.key
        Using configuration from /usr/lib/ssl/openssl.cnf
        Enter pass phrase for ca.key:
        Check that the request matches the signature
        Signature ok
        Certificate Details:
        Serial Number: 1 (0x1)
        Validity
            Not Before: Nov 16 09:00:43 2016 GMT
            Not After : Nov 16 09:00:43 2017 GMT
        Subject:
            countryName               = CN
            stateOrProvinceName       = Guangdong
            organizationName          = PAIDU, Ltd
            organizationalUnitName    = Hello
            commonName                = www.paidu.com
            emailAddress              = paidu@paidu.com
        X509v3 extensions:
            X509v3 Basic Constraints: 
                CA:FALSE
            Netscape Comment: 
                OpenSSL Generated Certificate
            X509v3 Subject Key Identifier: 
                C0:2B:60:1A:2F:B2:38:B7:26:0A:F4:CA:F0:7B:BD:AE:BE:B7:56:C9
            X509v3 Authority Key Identifier: 
                keyid:E9:9C:19:5C:66:D7:6C:F5:61:8C:72:19:39:6D:73:E1:B8:A3:C5:9B

        Certificate is to be certified until Nov 16 09:00:43 2017 GMT (365 days)
        Sign the certificate? [y/n]:y

        1 out of 1 certificate requests certified, commit? [y/n]y
        Write out database with 1 new entries
        Data Base Updated

2. 客户端证书生成

        客户端证书生成过程和服务端一致，但是需增加一个步骤让转换为浏览器可识别的格式
        heidong@HEIDONG:~/tmp/cert$ openssl pkcs12 -export -clcerts -in client.crt -inkey client.key -out client.pfx
        Enter pass phrase for client.key:
        Enter Export Password:
        Verifying - Enter Export Password:

使用上面生成的证书和密钥（服务端，server.key, server.crt, 客户端，client.crt, client.pfx，ca证书, ca.key, ca.cert），则可以在相关的服务器(nginx，apache，tomcat等)实现ssl的配置双向或单向认证，具体配置不再描述。
