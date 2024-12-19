use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use viuer::{print_from_file, Config};

fn base64_to_image(base64_image: &str) -> Result<(), Box<dyn Error>> {
    #[allow(deprecated)]
    let decoded_image = base64::decode(base64_image.replace("data:image/jpeg;base64,", ""))?;

    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::new());
    file_path.push(".bridgedb_captcha.jpeg");

    let mut file = fs::File::create(file_path)?;

    file.write_all(&decoded_image)?;

    Ok(())
}

fn prompt_transport_mode(transport_modes: &[(String, String)]) -> String {
    for (index, (_key, value)) in transport_modes.iter().enumerate() {
        println!("[{}] {}", index, value);
    }

    let mut user_input = String::new();
    print!("Choose your transport: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).unwrap();

    match user_input.trim().parse::<usize>() {
        Ok(index) if index < transport_modes.len() => transport_modes[index].0.to_string(),
        _ => "obfs4".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Getting available transports from BridgeDB");

    let transport_modes = lib::get_transport_modes().await?;

    let transport = prompt_transport_mode(&transport_modes);

    // The captcha requirement seems to be removed but im keeping the codes commented just in case
    /*
    let captcha = lib::request_transport(&transport).await?;

    
    base64_to_image(&captcha.0)?;

    let viuer_config = Config {
        transparent: false,
        absolute_offset: false,
        x: 0,
        y: 0,
        restore_cursor: false,
        width: None,
        height: None,
        truecolor: true,
        use_kitty: true,
        use_iterm: true,
    };

    let mut file_path = dirs::cache_dir().unwrap_or(PathBuf::new());
    file_path.push(".bridgedb_captcha.jpeg");

    print_from_file(&file_path, &viuer_config)?;

    print!("Enter the captcha: ");
    io::stdout().flush().unwrap();

    let mut captcha_answer = String::new();
    io::stdin()
        .read_line(&mut captcha_answer)
        .expect("Failed to read line");

    let bridgedb_response =
        lib::submit_answer(&transport, captcha.1, captcha_answer.trim().to_string()).await?;

    println!("Your bridges:\n{}", bridgedb_response);

    fs::remove_file(file_path)?;
    */


    let bridgedb_response = lib::request_transport(&transport).await?;
    println!("Your bridges:\n{}", bridgedb_response);

    Ok(())
}
