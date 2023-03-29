use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE},
    ClientBuilder, Url,
};
use std::error::Error;
use std::path::Path;

const AOC_START: u16 = 2015;
const AOC_LAST: u16 = 2022;

pub struct Config {
    url: String,
    fout_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        match args {
            [_, year, day] => {
                let year = validate_year(year.clone())?;
                let parsed_day = day.clone().parse::<u8>()?;
                let zero_pad_day = if parsed_day < 10 {
                    format!("0{}", day)
                } else {
                    day.clone()
                };

                Ok(Config {
                    url: format!("https://adventofcode.com/{year}/day/{day}/input"),
                    fout_path: format!("../../{year}/day{zero_pad_day}/input.txt"),
                })
            }
            _ => Err("Invalid number of arguments (2)".into()),
        }
    }
}

#[tokio::main]
pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = config.url.as_str().parse::<Url>().unwrap();
    let file_path = config.fout_path;

    let cookie = format!("session={};", std::env::var("SESSION_COOKIE").unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&cookie).expect("Invalid cookie header"),
    );

    let client = ClientBuilder::new().default_headers(headers).build()?;

    let results = client.get(url).send().await?.text().await?;

    std::fs::write(Path::new(&file_path), &results)
        .map_err(|err| format!("{err}, tried \"{file_path}\""))?;
    println!("Wrote {} to \"{}\"", &results, &file_path);

    Ok(())
}

fn validate_year(year: String) -> Result<String, Box<dyn Error>> {
    let parsed_year = year.clone().parse()?;
    let lower = AOC_START;
    let upper = AOC_LAST;

    if !(lower..=upper).contains(&parsed_year) {
        return Err(format!("invalid year input (first arg, {lower}..{upper})").into());
    }

    Ok(year)
}

#[cfg(test)]
mod tests {
    use super::*;
}
