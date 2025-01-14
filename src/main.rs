use std::env;
use std::env::current_exe;
use std::fs::write;
use std::io;
use std::io::Error;
use std::io::Write;
use winreg::enums::*;
use winreg::RegKey;

mod has_admin;

fn get_yn_prompt(text: &str) -> bool {
    print!("{text} (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    return match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => false,
    };
}

fn check_is_intercepting_active(exe_path: &str) -> Result<bool, Error> {
    let classes_root = RegKey::predef(HKEY_CLASSES_ROOT);
    let seb: String = classes_root
        .open_subkey("seb\\shell\\open\\command")?
        .get_value("")?;

    let sebs: String = classes_root
        .open_subkey("sebs\\shell\\open\\command")?
        .get_value("")?;

    let pattern = exe_path.to_string() + "\" \"%1\"";
    if seb.ends_with(&pattern) && sebs.ends_with(&pattern) {
        return Ok(true);
    }
    Ok(false)
}

fn set_seb_keys(value: &str) -> Result<(), Error> {
    let classes_root = RegKey::predef(HKEY_CLASSES_ROOT);
    // restore seb
    classes_root
        .open_subkey_with_flags("seb\\shell\\open\\command", KEY_WRITE)?
        .set_value("", &value)?;

    // restore sebs
    classes_root
        .open_subkey_with_flags("sebs\\shell\\open\\command", KEY_WRITE)?
        .set_value("", &value)?;
    Ok(())
}

fn deactivate_intercepting() -> Result<(), Error> {
    let standard_handler =
        "\"C:\\Program Files\\SafeExamBrowser\\Application\\SafeExamBrowser.exe\" \"%1\"";

    set_seb_keys(standard_handler)?;
    Ok(())
}
fn activate_intercepting(exe_path: &str) -> Result<(), Error> {
    let handler = "\"".to_string() + exe_path + "\" \"%1\"";

    set_seb_keys(&handler)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let exe_path = current_exe().unwrap();
    let exe_path = exe_path.to_str().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let is_intercepting = check_is_intercepting_active(exe_path)
            .expect("seb doesn't appear to even be installed!");
        println!("hello! welcome to sebbocept!");
        if !has_admin::is_elevated() {
            println!("this requires admin to enable/disable intercepting");
            return;
        }
        if is_intercepting {
            println!("intercepting is currently activated");
            if get_yn_prompt("disable it?") {
                deactivate_intercepting().expect("couldn't deactivate :<");
            }
        } else {
            println!("intercepting is currently not activated");
            if get_yn_prompt("enable it?") {
                activate_intercepting(exe_path).expect("couldn't activate :<");
            }
        }
        return;
    }
    let mut output_path = home::home_dir().unwrap();
    output_path.push("intercepted.seb");

    let data = &args[1];
    let mut stripped_text = "";
    let mut has_prefix = false;

    if let Some(stripped) = data.strip_prefix("sebs://") {
        has_prefix = true;
        stripped_text = stripped;
    }
    if let Some(stripped) = data.strip_prefix("seb://") {
        has_prefix = true;
        stripped_text = stripped;
    }

    if has_prefix {
        let url = "https://".to_string() + stripped_text;
        println!("hello! sending get request to {url} :>");
        let request = reqwest::get(url).await.unwrap();
        let body = request.bytes().await.unwrap();
        println!("got response yippie");
        write(&output_path, body).unwrap();
        println!("wrote to {}", output_path.to_str().unwrap());
    } else {
        println!("expected sebs:// or seb://, not... whatever that is");
        return;
    }
}
