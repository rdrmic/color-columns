use std::{env, fs::File, io::Write};

fn main() {
    set_windows_exe_icon();
    write_build_time_to_file();
}

fn set_windows_exe_icon() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}

fn write_build_time_to_file() {
    let file_path = format!("{}/build-time", env::var("OUT_DIR").unwrap());
    let mut file = File::create(&file_path).unwrap();

    let build_time = chrono::Local::now()
        .format("Built on %Y-%m-%d at %H:%M:%S")
        .to_string();

    write!(file, r#""{}""#, build_time).ok();
}
