# 阶段 8 项目二：待办事项 Web API

这是 `docs/08-project.md` 的方向 B，使用 Tokio、Axum 和 Serde 实现内存版 CRUD API。

```bash
cargo run -p stage08-todo-api
cargo test -p stage08-todo-api
```

服务默认监听 `127.0.0.1:3000`：

```bash
curl -X POST http://127.0.0.1:3000/tasks \
  -H 'content-type: application/json' \
  -d '{"title":"学习 Axum"}'
curl http://127.0.0.1:3000/tasks
curl -X PATCH http://127.0.0.1:3000/tasks/1/complete
curl -X DELETE http://127.0.0.1:3000/tasks/1
```

项目把路由、业务状态和启动入口分开，并覆盖正常 CRUD、无效输入和不存在资源。
