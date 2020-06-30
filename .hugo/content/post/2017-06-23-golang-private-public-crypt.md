---
title: golang 私钥"加密"公钥"解密"
date: 2017-06-23 20:57:11
aliases: [/2017/06/23/golang-private-public-crypt/]
categories: [go]
tags: [rsa, go]
---
之前工作主要使用C/C++与银行/第三方支付对接，但C/C++无法满足客户”当天给协议明天实盘上载“的开发速度以及现公司一些特殊情况，所以决定用go来尝试实现。基本的框架已经按照原来C/C++非阻塞框架实现一次，内部涉及加密方式也用go重新实现一遍，但一个数字证书加密的方式着实坑爹了一把，同时这个问题，也看到了openssl的命名混乱。

关于这个加密方式的描述是：发送方用私钥进行rsa加密，接受方使用公钥进行rsa解密。看到这样的加密方式描述，感觉和自己的理解是有点不一样，不知道是不是自己对这方面了解不够深入，自己的理解是（之前使用过的加密方式）：公钥是公开的，私钥是自己保存的，用私钥对数据进行**签名**，用公钥**验证**签名。感觉画风不一样，翻查一下openssl，的确也是存在这样的函数：
`RSA_private_encrypt`和`RSA_public_decrypt`，[参考文档](https://www.openssl.org/docs/manmaster/man3/RSA_private_encrypt.html)。用openssl很容易就实现这样一个加密解密。但用纯go语言实现，不可能再用cgo来调用c函数，翻查一下go的文档，存在在类似的函数(`crypt/rsa`)：
`func DecryptPKCS1v15(rand io.Reader, priv *PrivateKey, ciphertext []byte) ([]byte, error)`和`func EncryptPKCS1v15(rand io.Reader, pub *PublicKey, msg []byte) ([]byte, error)
`。但仔细看，这里是使用**公钥进行加密**，使用**私钥进行解密**，和描述刚好相反。除了这两个涉及公私钥加密的函数外，似乎在go里面找不到其他类似的函数了。

在google(科学上网lanttern)里面，能够搜索到的答案似乎不多，最后在stackoverflow找到结果：[Encrypt message with RSA private key (as in OpenSSL's RSA_private_encrypt](https://stackoverflow.com/questions/18011708/encrypt-message-with-rsa-private-key-as-in-openssls-rsa-private-encrypt)。一哥们手工搞定，其代码放在[goplaygound](https://play.golang.org/p/jrqN2KnUEM)。看了一下代码，如果不是对go内部的数据结构非常熟悉，而且对rsa机制非常清楚，很难写出正常代码。难道go就没有现成的代码完成这个功能？后面，有人就说，这是什么狗屁加密，压根就是一rsa签名，就用`crypt/rsa`里面，`func SignPKCS1v15(rand io.Reader, priv *PrivateKey, hash crypto.Hash, hashed []byte) ([]byte, error)`实现的。尼玛的，测试结果还真是一样。后来，回头看一下openssl里面的[参考文档](https://www.openssl.org/docs/manmaster/man3/RSA_private_encrypt.html)说明，**These functions handle RSA signatures at a low level.**，这就是签名啊，既然是签名，为何命名**encrypt/decrypt**？是不是因为命名问题，广为传播为私钥"加密"公钥"解密"呢？

go不知道是不是受不了这样私钥"加密"公钥"解密"这种混乱的说法，不像其他语言一样提供类似的函数呢？至于公钥"解密"，网上搜索不到满意答案，不过，既然私钥"加密"是rsa签名，那么公钥"解密"那么应该就是验证签名了。既然网上找不到满意的答案，那么只能修改一下go的`func VerifyPKCS1v15(pub *PublicKey, hash crypto.Hash, hashed []byte, sig []byte) error`函数。证实，这个想法是可行的。用openssl加密的数据，可以解密，加密的数据同时可以被openssl解密。

相关代码：https://github.com/buf1024/golib/tree/master/crypt 只简单导出`PrivateEncrypt`和`PublicDecrypt`两个函数。

最后，openssl的确存在一些非常混乱的命名方式，而其他语言/库，妥协这种混乱情况，那么混乱看起来即变为普遍。如不是非常熟悉，那么到一个不再妥协这种混乱时，那么及其容易使自己混乱啊。

