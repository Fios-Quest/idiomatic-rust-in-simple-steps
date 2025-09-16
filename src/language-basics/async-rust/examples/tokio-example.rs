use std::error::Error;

const FIOS_QUEST_URL: &str = "https://fios-quest.com";
const IRISS_URL: &str = "https://fios-quest.com/idiomatic-rust-in-simple-steps/";

async fn get_url(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url).await?.text().await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (fios_quest, iriss) = tokio::join!(get_url(FIOS_QUEST_URL), get_url(IRISS_URL));

    let fq_chars_chars = fios_quest?.chars().count();
    let iriss_chars = iriss?.chars().count();

    println!("Fio's Quest's body contains {fq_chars_chars} characters");
    println!("IRISS's body contains {iriss_chars} characters");

    Ok(())
}
