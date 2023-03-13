## eatnp
快速释放前端项目node_modules存储空间。
### 为什么要写这个工具
```
每个现代化的前端项目，都有一个远大于代码本身体积的的node_modules用于本地开发及构建部署。
当电脑里这样的前端项目非常多的情况下，会导致储存空间不够。
为了开发便捷，我们一般不会连带项目代码直接删除。
当我们聚焦于某个或某几个项目时，可以将其余非当前迭代中的node_modules吃掉，从而释放储存空间。
```


### 如何安装
```
cargo install eatnp
```

### 使用示例

```
eatnp /Users/foo/base
```

例如

```
deleted: "/Users/foo/base/app1/node_modules                                              | size 402 MB"
deleted: "/Users/foo/base/app2/node_modules                                              | size 502 MB"
deleted: "/Users/foo/base/app3/node_modules                                              | size 42 MB"
deleted: "/Users/foo/base/app4/node_modules                                              | size 98 MB"
deleted: "/Users/foo/base/app5/node_modules                                              | size 232 MB"
deleted: "/Users/foo/base/app6/node_modules                                              | size 451 MB"
deleted: "/Users/foo/base/app7/node_modules                                              | size 423 MB"
deleted: "/Users/foo/base/app8/node_modules                                              | size 984 MB"
deleted: "/Users/foo/base/app9/node_modules                                              | size 322 MB"
deleted: "/Users/foo/base/app10/node_modules                                             | size 242 MB"
deleted: "/Users/foo/base/app11/node_modules                                             | size 121 MB"
```

### 开发指南
需要预先安装rust开发环境
#### 开发时调试

```
cargo run -- /path/dir/
```

#### 打包出bin运行
```shell
cargo build --bin eatnp
```

```
./target/debug/eatnp /path/dir/
```


#### 发布到crates.io

```
// 登录到crates.io
cargo login

// 发布测试
cargo publish --dry-run

// 正式发布
cargo publish
```

### FAQ
#### 没有英文版README？
懒得翻译了，看不懂中文的话，试试“翻译成英文”。

#### node_modules目录是 真实文件大小 还是 文件在磁盘上的大小？
真实文件大小。[真实文件大小与文件在磁盘上的大小区别](https://web.archive.org/web/20140712235443/https://stackoverflow.com/questions/15470787/please-help-me-understand-size-vs-size-on-disk)

### 声明
```
node_modules删除后不可恢复，一切由于使用本工具造成的node_modules误删行为，由使用者本人负责！！！
node_modules删除后不可恢复，一切由于使用本工具造成的node_modules误删行为，由使用者本人负责！！！
node_modules删除后不可恢复，一切由于使用本工具造成的node_modules误删行为，由使用者本人负责！！！
```