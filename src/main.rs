use std::{
    env::set_current_dir,
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use clap::{command, Parser};
use dirs::home_dir;

#[derive(Parser)] // requires `derive` feature
#[command(name = "whisper-cli")]
#[command(bin_name = "whisper-cli")]
#[command(about = "A friendly helper for anyone running whisper 🐍", long_about = None)]
enum CargoCli {
    /// Setup whisper-cli to convert audio files to text
    Setup {
        /// Install ffmpeg
        #[arg(short = 'f', long = "ffmpeg")]
        ffmpeg: bool,
    },
    /// Run whisper-cli on a file
    #[command(arg_required_else_help = true)]
    Run {
        /// Path to the audio file
        #[arg(required = true)]
        path: PathBuf,
    },
}

fn setup(ffmpeg: bool) {
    println!("Downloading whisper.cpp 💽");
    Command::new("curl")
        .args([
            "-L",
            "-o",
            "whisper.cpp.zip",
            "https://github.com/ggerganov/whisper.cpp/archive/master.zip",
        ])
        .output()
        .expect("failed to execute curl");

    println!("Unzipping whisper.cpp 📦");
    Command::new("unzip")
        .args(["whisper.cpp.zip"])
        .output()
        .expect("failed to execute unzip");

    println!("Removing whisper.cpp.zip 🗑");
    Command::new("rm")
        .args(["whisper.cpp.zip"])
        .output()
        .expect("failed to execute rm");

    let home_dir = home_dir().unwrap().to_str().unwrap().to_string();
    let whisper_dir = format!("{home_dir}/.whisper");

    println!("Clearing old .whisper directory 📂");
    Command::new("rm")
        .args(["-rf", &whisper_dir])
        .output()
        .expect("failed to execute rm");

    println!("Moving contents of whisper.cpp-master to $HOME 📂");
    Command::new("mv")
        .args(["whisper.cpp-master", &whisper_dir])
        .status()
        .expect("failed to execute mv");

    match ffmpeg {
        true => {
            println!("Installing ffmpeg 🎞");
            Command::new("brew")
                .args(["install", "ffmpeg"])
                .stderr(Stdio::piped())
                .status()
                .expect("failed to install ffmpeg 🚧");
        }
        false => println!("Skipping ffmpeg installation 🚫"),
    }

    set_current_dir(whisper_dir).expect("failed to enter whisper folder");

    println!("Downloading base.en 📥");
    Command::new("bash")
        .args(["./models/download-ggml-model.sh", "base.en"])
        .output()
        .expect("failed to execute bash ./models/download-ggml-model.sh base.en");

    println!("Compiling whisper.cpp 🛠");
    Command::new("make")
        .output()
        .expect("failed to execute make");

    println!("Whisper setup complete ✅");
}

fn run(path: PathBuf) {
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

    println!("Converting {} to wav 🎞", base_file_name);
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
        .output()
        .expect("failed to execute ffmpeg");

    sleep(Duration::from_millis(1000));

    println!("Running whisper 🤫");
    let home_dir = home_dir().unwrap().to_str().unwrap().to_string();
    let whisper_dir = format!("{}/.whisper", home_dir);
    Command::new(format!("{}/main", whisper_dir))
        .args([
            "-m",
            &format!("{}/models/ggml-base.en.bin", whisper_dir),
            "-f",
            format!("{}.wav", file_path_name).as_str(),
            "-otxt",
            "-of",
            &file_path_name,
        ])
        .status()
        .expect("failed to execute whisper");

    println!("Deleting wav file 🗑");
    Command::new("rm")
        .args([format!("{}.wav", file_path_name).as_str()])
        .output()
        .expect("failed to execute rm");
}

fn main() {
    let args = CargoCli::parse();

    match args {
        CargoCli::Setup { ffmpeg } => setup(ffmpeg),
        CargoCli::Run { path } => run(path),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_setup() {
        setup(false);
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
    }

    #[test]
    fn test_run() {
        run(PathBuf::from(
            "test/test folder.with\\weird^~..symbols/test.mp3",
        ));
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
