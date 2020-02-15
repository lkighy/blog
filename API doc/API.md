# API

## 响应状态

| 状态码 | 说明         |
| ------ | ------------ |
| 200    | OK           |
| 201    |
| 401    |
| 403    |
| 404    | Not Found    |
| 500    | Server Error |

## 前台

### 前缀

> /api/blog/

### 统计

页头获取 包括 logo, 页面大标题

> info

作者信息

> author/info

标签列表

> tags/list

首页文章分页查看

> article/list

标签文章查询 不过存在一个问题, 如果, 这个标签存在多个, 则如何查询

> tags/article/list

书籍目录查询

> books/catalogs

获取文章评论

> article/comments





### 首页文章分页查看 api

url:

> /api/article/list

请求参数:

```json
{
    "pageSize": 0, // 页面显示数量
    "pageCurrent": 0, // 当前页
}
```

响应参数:

```json
{
    "code": 200,
    "data": {
        "list": []
    },
    "msg": "",
}
```

### 文章查看 api

url:

> /api/article/view

请求参数:

```json
{
    "id": 0,
}
```

响应参数:

```json
{
    "code": 200,
    "data": {},
    "msg": "",
}
```

### 标签

url

> /api/tag/list

请求参数:

无

响应参数:

```json
{
    "code": 200,
    "data": {
        "list": []
    },
    "msg": "",
}
```

### 作者信息

### 最近更新

### 同标签内容分页查看

### 同书籍内容查看

## 后台

### 前缀

> /api/data/

