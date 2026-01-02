/*
C:\Solutions\Personal\AddToPathWindow\target\debug\deps;C:\Solutions\Personal\AddToPathWindow\target\debug;C:\Users\austi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Users\austi\bin;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\local\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\austi\bin;C:\Program Files\Common Files\Oracle\Java\javapath;C:\Program Files (x86)\Common Files\Oracle\Java\java8path;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\bin;C:\Program Files\dotnet;C:\Program Files\Docker\Docker\resources\bin;C:\Users\austi\.cargo\bin;C:\Users\austi\AppData\Local\Programs\Python\Python311\Scripts;C:\Users\austi\AppData\Local\Programs\Python\Python311;C:\Users\austi\AppData\Local\Microsoft\WindowsApps;C:\Users\austi\AppData\Local\Programs\Microsoft VS Code\bin;C:\Users\austi\AppData\Roaming\npm;C:\Solutions\Personal\Bash Scripts;C:\MinGW\bin;C:\FlutterSDK\flutter\bin;C:\Users\austi\AppData\Local\Pub\Cache\bin;C:\Users\austi\AppData\Local\Google\Cloud SDK\google-cloud-sdk\bin;C:\Users\austi\AppData\Local\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files\Java\jdk-22\bin;C:\Users\austi\AppData\Local\Programs\MiKTeX\miktex\bin\x64;C:\Program Files\Git\usr\bin\vendor_perl;C:\Program Files\Git\usr\bin\core_perl
*/

use std::{error::Error, ffi::OsStr, path::Path};

use winreg::{RegKey, enums::HKEY_CURRENT_USER};

use crate::edit_context_lib::types::NormalResult;

pub fn valid_path<T: AsRef<OsStr>>(dir_ref: &T) -> bool {
    let path = Path::new(dir_ref);

    (*path).is_dir()
}

pub fn add_to_path<T: Into<String>>(dir_into: T) -> NormalResult {
    let dir: String = dir_into.into();

    if !valid_path(&dir) {
        return Err("Directory does not exist".into());
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let (file, _) = hkcu.create_subkey("Environment")?;

    let cur_path: String = file.get_value("PATH")?;

    let new_path = format!("{};{}", cur_path, dir);

    file.set_value("PATH", &new_path)?;

    println!("Added Directory \"{}\" to User Path", dir);

    Ok(())
}

pub fn check_in_path<'a>(exe: &'a str) -> Result<Box<str>, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("WHERE")
        .arg(format!("$path:{}", exe))
        .output()?;

    let path = str::from_utf8(&output.stdout)?.trim();

    Ok(path.into())
}
