use std::error::Error;

use headless_chrome::Browser;
use rand::Rng;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    scraiping().await.expect("TODO: panic message");
}

async fn scraiping() -> Result<(), Box<dyn Error>> {
    // ブラウザとタブの初期設定
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(Duration::from_secs(200));

    // ページの読み込み
    let url = "https://forms.office.com/Pages/ResponsePage.aspx?id=********";
    tab.navigate_to(url)?;
    // ページの読み込みが完了するまで待機
    tab.wait_until_navigated()?;
    println!("page loaded");

    // grade
    tab.wait_for_element("div.__question__:nth-child(1) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > label:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    println!("grade clicked");

    // class
    tab.wait_for_element("div.__question__:nth-child(2) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > label:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    println!("class clicked");

    // number
    tab.wait_for_element("div.__question__:nth-child(3) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > label:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    println!("number clicked");

    // club
    tab.wait_for_element("div.__question__:nth-child(4) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > label:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    println!("club clicked");

    // name
    tab.wait_for_element("div.__question__:nth-child(5) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    tab.type_str("Your Name")?;
    println!("name typed");

    // temprerature
    let mut rng = rand::thread_rng();
    let temp = "36.".to_string() + rng.gen_range(1..8).to_string().as_str();
    tab.wait_for_element("div.__question__:nth-child(6) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    tab.type_str(&*temp)?;
    println!("temprerature typed");

    // condition
    tab.wait_for_element("div.__question__:nth-child(7) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > label:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    println!("condition clicked");

    // family condition
    tab.wait_for_element("div.__question__:nth-child(8) > div:nth-child(1) > div:nth-child(3) > div:nth-child(1) > div:nth-child(2) > div:nth-child(1) > label:nth-child(1) > div:nth-child(1) > input:nth-child(1)")?.click()?;
    println!("family condition clicked");

    // finish
    tab.wait_for_element(".office-form-bottom-button > div:nth-child(1)")?.click()?;

    println!("体温{temp}°Cでフォームを送信しました", temp = temp);

    sleep(Duration::from_millis(1000)).await;

    Ok(())
}
