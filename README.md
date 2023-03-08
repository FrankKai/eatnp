## eatnodemodules

### 为什么要写这个工具
每个现代化的前端项目，都有一个远大于代码本身体积的的node_modules用于本地开发及构建部署。
当电脑里这样的前端项目非常多的情况下，会导致储存空间不够。
为了开发便捷，我们一般不会连带项目代码直接删除。
所以当我们聚焦于某个或某几个项目时，可以将其余非当前迭代中的node_modules吃掉，从而释放储存空间。


### 如何使用
在第一个命令行参数输入你存放前端项目的目录名称即可。

#### clone项目到本地运行
```
cargo run -- /path/dir/
```

#### 打包出bin运行
```
cargo build --bin eatnodemodules
```

```
./target/debug/eatnodemodules /path/dir/
```
### 使用示例

```
eatnodemodules /Users/foo/base
```

例如

```
deleted: "/Users/foo/base/app1/node_modules"
deleted: "/Users/foo/base/app2/node_modules"
deleted: "/Users/foo/base/app3/node_modules"
deleted: "/Users/foo/base/app4/node_modules"
deleted: "/Users/foo/base/app5/node_modules"
```