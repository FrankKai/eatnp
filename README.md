## eatnodemodules
当我们聚焦于某个或某几个项目时，可以将其余非当前迭代中的node_modules吃掉，从而释放储存空间。
### 为什么要写这个工具
```
每个现代化的前端项目，都有一个远大于代码本身体积的的node_modules用于本地开发及构建部署。
当电脑里这样的前端项目非常多的情况下，会导致储存空间不够。
为了开发便捷，我们一般不会连带项目代码直接删除。
```


### 如何安装
```
cargo install eatnodemodules
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
deleted: "/Users/foo/base/app6/node_modules"
deleted: "/Users/foo/base/app7/node_modules"
deleted: "/Users/foo/base/app8/node_modules"
deleted: "/Users/foo/base/app9/node_modules"
deleted: "/Users/foo/base/app10/node_modules"
deleted: "/Users/foo/base/app11/node_modules"
```

### 开发指南
需要预先暗转rust开发环境
#### 开发时调试

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


#### 发布到crates.io

```
// 登录到crates.io
cargo login

// 发布测试
cargo publish --dry-run

// 正式发布
cargo publish
```


### 没有英文版README？
懒得翻译了，看不懂中文自己“翻译成英文”去。