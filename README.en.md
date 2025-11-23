# Kovi-plugin-news

This is a plugin developed for [Kovi](https://github.com/0x727/kovi) to fetch daily news. It retrieves news data via HTTP requests from a specified URL and parses the JSON response using the `serde` serialization library.

## Features

- Fetches daily news from a configured URL.
- Uses `reqwest` for asynchronous HTTP requests.
- Uses `serde` for JSON data parsing.
- Supports triggering news retrieval via the `/dailynews` command.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed. Then clone the repository and build the plugin:

```bash
git clone https://gitee.com/yiranxiaohui/kovi-plugin-news
cd kovi-plugin-60s
cargo build --release
```

## Configuration

Configure the news API URL in the `config.toml` file:

```toml
url = "http://your-news-api.com"
```

## Usage

Start Kovi and load the plugin. Send the `/dailynews` command to retrieve the latest news.

## Project Structure

- `src/lib.rs`: Main logic of the plugin.
- `src/model/mod.rs`: Data model definitions, including the `Response`, `News`, and `Config` structs.

## Dependencies

- `kovi`: Kovi plugin framework.
- `reqwest`: For asynchronous HTTP requests.
- `serde`: For JSON serialization and deserialization.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).