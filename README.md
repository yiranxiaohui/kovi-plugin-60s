# Kovi-plugin-60s

这是一个为 [Kovi](https://github.com/ThriceCola/Kovi) 开发的插件，基于 [60s](https://github.com/vikiboss/60s)用于获取每日新闻。它通过 HTTP 请求从指定的 URL 获取新闻数据，并使用 `serde` 序列化库来解析 JSON 响应。

## 特性

- 从配置的 URL 获取每日新闻。
- 使用 `reqwest` 进行异步 HTTP 请求。
- 使用 `serde` 进行 JSON 数据解析。
- 支持通过 `/60s` 命令触发60s新闻获取。

## 配置

在`data/kovi-plugin-60s/config.toml` 文件中配置新闻 API 的 URL：

```toml
url = "http://your-60s-api.com"
```

公益`60s`API接口
```toml
url = "https://60s.yunnet.top"
```

60s自建方案（Docker）
```bash
docker run -d \
  --restart always \
  --name 60s \
  -p 4399:4399 \
  vikiboss/60s:latest
```

## 使用

启动 Kovi 并加载插件。。

## 支持的命令
- `/60s` 机器人回复每日新闻的图片
- `/help` 机器人回复本插件所有支持的命令

## 项目结构

- `src/lib.rs`：插件的主逻辑。
- `src/model/mod.rs`：数据模型定义，包括 `Response`, `News`, 和 `Config` 结构体。

## 依赖

- `kovi`：Kovi 插件框架。
- `reqwest`：用于异步 HTTP 请求。
- `serde`：用于 JSON 序列化和反序列化。

## 许可证

本项目使用 [MIT License](https://opensource.org/licenses/MIT)。