# Kovi-plugin-news

这是一个为 [Kovi](https://github.com/0x727/kovi) 开发的插件，用于获取每日新闻。它通过 HTTP 请求从指定的 URL 获取新闻数据，并使用 `serde` 序列化库来解析 JSON 响应。

## 特性

- 从配置的 URL 获取每日新闻。
- 使用 `reqwest` 进行异步 HTTP 请求。
- 使用 `serde` 进行 JSON 数据解析。
- 支持通过 `/每日新闻` 命令触发新闻获取。

## 安装

确保你已经安装了 [Rust](https://www.rust-lang.org/) 和 [Cargo](https://doc.rust-lang.org/cargo/)。然后，克隆仓库并构建插件：

```bash
git clone https://gitee.com/yiranxiaohui/kovi-plugin-news
cd kovi-plugin-60s
cargo build --release
```

## 配置

在`data/插件名/config.toml` 文件中配置新闻 API 的 URL：

```toml
url = "http://your-news-api.com"
```

公益`60s`API接口
```toml
url = "https//60s.yunnet.top"
```


## 使用

启动 Kovi 并加载插件。发送 `/每日新闻` 命令以获取最新的新闻。

## 项目结构

- `src/lib.rs`：插件的主逻辑。
- `src/model/mod.rs`：数据模型定义，包括 `Response`, `News`, 和 `Config` 结构体。

## 依赖

- `kovi`：Kovi 插件框架。
- `reqwest`：用于异步 HTTP 请求。
- `serde`：用于 JSON 序列化和反序列化。

## 许可证

本项目使用 [MIT License](https://opensource.org/licenses/MIT)。