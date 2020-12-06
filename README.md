# RUST-Newcomer-Development-Program
(RUST新人培养计划) RUST Newcomer Development Program

### 基础
- RUST基础 阅读 https://rustwiki.org/zh-CN//rust-by-example/index.html  [s01]
    - 考核 参与基础答辩
- TCP UDP 基础库学习  [s02]
    - CLI TODO LIST  [单线程]
    - CLI TODO LIST  [多线程]
    - 基于TCP 分布式TODO LIST
    - 考核 基于UDP 分布式TODO LIST
- Async_std 库学习 [s03]
    - 考核 将上面基于TCP 的TODO LIST 改为异步
- MySQL [s04]
    - 考核将上面的TODO LIST 存储层修改为MySQL  (现在市面上的ORM太不人性化了  打算写一个RUST的GORM)
- MongoDB [s05]
- Redis [s06]
    - 考核将上面的TODO LIST 存储层修改为Redis
  
  
### base middleware
``` 
docker run --restart=always -p 3306:3306 --name mysql -e MYSQL_ROOT_PASSWORD=root -d mysql:8
docker run --restart=always -p 6379:6379 --name redis  -d redis:6-alpine
docker run --restart=always -p 5432:5432 --name postgre  -e POSTGRES_PASSWORD=root -d postgres:13-alpine
```