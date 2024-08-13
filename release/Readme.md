# Simx Extension Development Guide

> 插件允许通过多种编程语言进行开发，此文档仅用于插件开发者对simx插件进行开放时使用

## 插件原理

Simx 插件是运行时动态加载的插件，仅在启动时动态加载extension.json，在运行时才会根据配置文件加载对应的插件。

由于插件本身都是运行时插件，因此必须在运行时加载后才能调用，引擎是无法直接从扩展文件中读取具体有哪些方法，
所以需要在extension中对插件进行描述。

## 扩展文件

> extension.json

允许调用的方法必须在extension中定义，格式如下：

```json
{
  "name": "插件名称",
  "version": "插件版本",
  "engine": "支持的引擎版本",
  "description": "插件描述",
  "license": "开源许可证，可以为None，不影响",
  "author": "插件作者",
  "keywords": [],
  "dependencies": [],
  "function": [
      "test1": [
        "core.dll",
        "测试方法1",
        "String",
        "String a",
        "String b"
      ],
      "test2": [
        "core.dll",
        "测试方法2",
        "String",
        "String a",
        "String b"
      ],
      "test3": [
        "support.dll",
        "测试方法3",
        "String",
        "String a",
        "String b"
      ]
  ]
}
```

其中，方法定义必须为一个数组，数组中包含方法名，库名，返回值类型，参数类型和参数名/类型，即：

方法名: ["库名","方法描述","返回值类型","参数1","参数2"...]

> **注意：即使字段值为空，也不可忽略，比如方法描述并没有值，或者方法没有返回值，这种情况下需要使用空字符串，但不要直接忽略该值，引擎会根据数组下标寻找需要的值
**

## 数据类型

目前支持的类型如下：

| 类型         | 说明          | 描述               |
|------------|-------------|------------------|
| none       | 空类型         | 仅用于返回值，也可以写作空字符串 |
| bytes      | 字节数组        | 如图片数据            |
| int        | 整数          | 整数               |
| float      | 浮点数         | 一般浮点             |
| double     | 高精度浮点       | 高精度浮点            |
| boolean    | 布尔值         | 布尔值              |
| String     | 字符串         | 一般字符串            |
| JsonObject | Json对象（字符串） | 返回Json对象的字符串即可   |
| JsonArray  | Json数组（字符串） | 返回Json数组的字符串即可   |

