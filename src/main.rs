use std::fs::{File};
use std::io::Read;
use reqwest::{Client};
use serde_json::Value;
use std::{thread::sleep};
use std::time::Duration;
use enigo::*;



#[tokio::main]
async fn main(){
    let mut buf = Vec::new();
    File::open("rclient.der").unwrap().read_to_end(&mut buf).unwrap();
    let id = reqwest::Certificate::from_der(&buf).unwrap();
    let client = reqwest::Client::builder()
        .add_root_certificate(id).build().unwrap();
    loop {
        match req(client.clone()).await {
            Ok(_) => {
                println!("Game is live");
                let chosen_health :f64;
                chosen_health = 500f64;
                println!("Trigger activated");

                loop {
                    let mut enigo = Enigo::new();
                    let client = client.clone();
                    let response = req(client.clone()).await.unwrap();
                    let data: Value = serde_json::from_str(&response).unwrap();
                    let health = get_health(data.clone());
                    let slot = parse_item_slot(data);


                    if health != 0.0 && slot != -1{
                        if health_bool(health, chosen_health) {
                            if slot == 0{
                                enigo.key_click(Key::Layout('&'));
                                println!("Saved from death");
                                sleep(Duration::from_secs(30));
                                break;
                            }
                            if slot == 1{
                                enigo.key_click(Key::Layout('é'));
                                println!("Saved from death");
                                sleep(Duration::from_secs(30));
                                break;
                            }
                            if slot == 2{
                                enigo.key_click(Key::Layout('"'));
                                println!("Saved from death");
                                sleep(Duration::from_secs(30));
                                break;
                            }
                            if slot == 3{
                                enigo.key_click(Key::Layout('('));
                                println!("Saved from death");
                                sleep(Duration::from_secs(30));
                                break;
                            }
                            if slot == 4{
                                enigo.key_click(Key::Layout('-'));
                                println!("Saved from death");
                                sleep(Duration::from_secs(30));
                                break;
                            }
                            if slot == 5{
                                enigo.key_click(Key::Layout('è'));
                                println!("Saved from death");
                                sleep(Duration::from_secs(30));
                                break;

                            }
                        }
                    }
                }
            }
            Err(_) => {
                println!("Game is not live");
                sleep(Duration::from_secs(5));
            }
        };
    }

}


async fn req(client: Client) -> Result<String, reqwest::Error> {
    let response = client
        .get("https://127.0.0.1:2999/liveclientdata/allgamedata")
        .send()
        .await?;
    Ok(response.text().await?)
}



fn parse_item_slot(data:Value) -> i64{
    if let Some(items) = data["allPlayers"][0]["items"].as_array() {
        if let Some(zhonyas) = items.iter().find(|&item| item["displayName"] == "Zhonya's Hourglass" || item["displayName"] == "Stopwatch") {
            let slot = zhonyas["slot"].as_i64().unwrap();
            return slot
            }
        }
    -1
    }

fn get_health(data:Value) -> f64{
    return data["activePlayer"]["championStats"]["currentHealth"].as_f64().unwrap_or_default();
}

fn health_bool(currenthealth:f64,healthchosen:f64) -> bool{
    if currenthealth <= healthchosen{
        return true
    }
    false
}
