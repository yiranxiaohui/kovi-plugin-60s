use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub data: T
}

#[derive(Deserialize, Debug)]
pub struct News {
    pub image: String
}

#[derive(Deserialize)]
pub struct Config {
    pub url: String
}