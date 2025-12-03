mod model;

use std::sync::Arc;
use kovi::log::{debug};
use kovi::{serde_json, toml, Message, PluginBuilder as plugin, PluginBuilder, RuntimeBot};
use kovi::toml::{toml};
use kovi::utils::load_toml_data;
use crate::model::{Config, News, Response};

#[kovi::plugin]
async fn main() {
    let bot = PluginBuilder::get_runtime_bot();
    read_config(bot.clone());
    plugin::on_msg(move |event| {
        let bot = bot.clone();
        async move {
            let text = event.borrow_text().unwrap_or("");
            if text.starts_with("/60s") {
                if let Ok(news) = get_60s(bot).await {
                    let msg = Message::new()
                        .add_image(news.image.as_str());
                    event.reply(msg);
                } else {
                    let msg = Message::new()
                        .add_text("系统异常，请检查系统日志");
                    event.reply(msg);
                }
            } else if text.starts_with("/help") {
                let msg = Message::new()
                    .add_text("- /60s")
                    .add_text("- /help");
                event.reply(msg);
            }
        }
    });
}

fn read_config(bot: Arc<RuntimeBot>) -> Config {
    let data_path = bot.get_data_path();
    let config_toml_path = data_path.join("config.toml");
    let default_config = toml! {
        url = "http://127.0.0.1:4399"
    };
    let config = load_toml_data(default_config, config_toml_path).unwrap();
    debug!("{}", config.to_string());
    let config: Config = toml::from_str(&config.to_string()).unwrap();
    config
}

async fn get_60s(bot: Arc<RuntimeBot>) -> Result<News, String> {
    let client = reqwest::Client::new();
    let config = read_config(bot);
    match client.get(format!("{}/v2/60s", config.url)).send().await {
        Ok(res) => {
            let json = res.text().await.unwrap();
            debug!("get_60s_response = {:?}", json);
            let response: Response<News> = serde_json::from_str(json.as_str()).unwrap();
            Ok(response.data)
        }
        Err(err) => {
            debug!("get_60s_response = {:?}", err);
            Err(err.to_string())
        }
    }

}