use std::{
    env::set_current_dir,
    process::{Command, Stdio},
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

fn main() {
    let args = CargoCli::parse();

    match args {
        CargoCli::Setup { ffmpeg } => setup(ffmpeg),
    }
}

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
