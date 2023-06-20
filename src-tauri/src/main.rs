// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use async_process::Command;
use dirs::home_dir;
use std::path::PathBuf;
use tauri::Window;

use crate::utils::send_message;

#[tauri::command]
async fn setup(window: Window) {
    let home_dir = home_dir().unwrap().to_str().unwrap().to_string();
    send_message(window.to_owned(), "setup", "Starting whisper setup 🚀");

    let whisper_zip_path = format!("{home_dir}/whisper.cpp.zip");
    send_message(window.to_owned(), "setup", "Downloading whisper.cpp 💽");
    Command::new("curl")
        .args([
            "-L",
            "-o",
            &whisper_zip_path,
            "https://github.com/ggerganov/whisper.cpp/archive/master.zip",
        ])
        .status()
        .await
        .expect("failed to execute curl");

    let whisper_dir_path = format!("{home_dir}/.whisper");
    send_message(window.to_owned(), "setup", "Unzipping whisper.cpp 📦");
    Command::new("unzip")
        .args([whisper_zip_path.as_str(), "-d", home_dir.as_str()])
        .output()
        .await
        .expect("failed to execute unzip");

    send_message(window.to_owned(), "setup", "Removing whisper.cpp.zip 🗑");
    Command::new("rm")
        .args([whisper_zip_path.as_str()])
        .output()
        .await
        .expect("failed to execute rm");

    send_message(
        window.to_owned(),
        "setup",
        "Preparing .whisper directory 📂",
    );
    Command::new("rm")
        .args(["-rf", whisper_dir_path.as_str()])
        .output()
        .await
        .expect("failed to execute rm");

    let whisper_unzipped_path = format!("{home_dir}/whisper.cpp-master");
    send_message(
        window.to_owned(),
        "setup",
        "Moving whisper.cpp-master to $HOME 📂",
    );
    Command::new("mv")
        .args([whisper_unzipped_path.as_str(), whisper_dir_path.as_str()])
        .status()
        .await
        .expect("failed to execute mv");

    send_message(window.to_owned(), "setup", "Downloading base.en 📥");
    Command::new("bash")
        .args([
            format!("{whisper_dir_path}/models/download-ggml-model.sh").as_str(),
            "base.en",
        ])
        .output()
        .await
        .expect("failed to execute bash ./models/download-ggml-model.sh base.en");

    send_message(window.to_owned(), "setup", "Compiling whisper.cpp 🛠");
    Command::new("make")
        .arg(format!("--directory={whisper_dir_path}"))
        .status()
        .await
        .expect("failed to execute make");

    send_message(window.to_owned(), "setup", "Whisper setup complete ✅");
}

#[tauri::command]
async fn run(window: Window, path: PathBuf) {
    let base_file_name = path.file_name().unwrap().to_str().unwrap();
    let path_name = path.parent().unwrap().to_str().unwrap();
    let file_name = base_file_name
        .split(".")
        .collect::<Vec<&str>>()
        .into_iter()
        .take(base_file_name.split(".").collect::<Vec<&str>>().len() - 1)
        .collect::<Vec<&str>>()
        .join(".");
    let file_path_name = format!("{}/{}", path_name, file_name);

    send_message(
        window.to_owned(),
        "run",
        format!("Converting {} to wav 🎞", base_file_name).as_str(),
    );
    Command::new("ffmpeg")
        .args([
            "-i",
            path.to_str().unwrap(),
            "-ar",
            "16000",
            "-f",
            "wav",
            format!("{}.wav", file_path_name).as_str(),
        ])
        .status()
        .await
        .expect("failed to execute ffmpeg");

    send_message(window.to_owned(), "run", "Running whisper 🤫");
    let home_dir = home_dir().unwrap().to_str().unwrap().to_string();
    let whisper_dir = format!("{}/.whisper", home_dir);
    Command::new(format!("{}/main", whisper_dir))
        .args([
            "-m",
            format!("{whisper_dir}/models/ggml-base.en.bin").as_str(),
            "-f",
            format!("{file_path_name}.wav").as_str(),
            "-otxt",
            "-of",
            file_path_name.as_str(),
        ])
        .output()
        .await
        .expect("failed to execute whisper");

    send_message(window.to_owned(), "run", "Deleting wav file 🗑");
    Command::new("rm")
        .args([format!("{file_path_name}.wav").as_str()])
        .output()
        .await
        .expect("failed to execute rm");

    send_message(window.to_owned(), "run", "Success! 🎉")
}

fn main() {
    fix_path_env::fix().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![setup, run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* #[cfg(test)]
Enable when tests are stable.
mod tests {
    use super::*;
    use tauri::Manager;
    #[test]
    fn test_workflow() {
        let app = super::create_app(tauri::test::mock_builder());
        let window = app.get_window("main").unwrap();
        tauri::test::assert_ipc_response(
            &window,
            tauri::InvokePayload {
                cmd: "setup".into(),
                tauri_module: None,
                callback: tauri::api::ipc::CallbackFn(0),
                error: tauri::api::ipc::CallbackFn(1),
                inner: serde_json::Value::Null,
            },
            Ok(()),
        );

        let home_dir = home_dir().unwrap().to_str().unwrap().to_string();
        let whisper_dir = &format!("{}/.whisper", home_dir);
        assert_eq!(PathBuf::from(whisper_dir).exists(), true);
        assert_eq!(PathBuf::from(format!("{whisper_dir}/main")).exists(), true);
        assert_eq!(
            PathBuf::from(format!("{}/models/ggml-base.en.bin", whisper_dir)).exists(),
            true
        );
        assert_eq!(PathBuf::from("whisper.cpp.zip").exists(), false);
        assert_eq!(PathBuf::from("whisper.cpp-master").exists(), false);

        tauri::test::assert_ipc_response(
            &window,
            tauri::InvokePayload {
                cmd: "run".into(),
                tauri_module: None,
                callback: tauri::api::ipc::CallbackFn(0),
                error: tauri::api::ipc::CallbackFn(1),
                inner: serde_json::Value::Null,
            },
            Ok(()),
        );

        assert_eq!(
            PathBuf::from("test/test folder.with\\weird^~..symbols/test.wav").exists(),
            false
        );
        assert_eq!(
            PathBuf::from("test/test folder.with\\weird^~..symbols/test.txt").exists(),
            true
        );
    }
}
 */
