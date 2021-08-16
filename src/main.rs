extern crate chrono;
use std::process::Command;
use std::time::Duration;
use std::env
use chrono::{Local, DateTime};

fn main() {
      let video_duration_secs: u64 = 3 * 60;

      loop {
          let now: DateTime<Local> = Local::now();

          let date = now.format("%d%m%Y_%T").to_string();

          let formated_date = String::from(date).replace(":", "_");

          let user = env::var("user");
          let password = env::var("password");
          let address = env::var("address");
          let port = env::var("port");;
          let files_folder = env::var("cam_files");

          println!("{}", formated_date);

          let file_name = format!("front_cam_01_{}.mp4", formated_date);

          let _command = Command::new("ffmpeg")
              .arg("-y")
              .arg("-t")
              .arg(format!("{}", video_duration_secs))
              .arg("-r")
              .arg("20")
              .arg("-i")
              .arg(format!("rtsp://{}:{}@{}:{}/onvif1", user, password, address, port))
              .arg("-vcodec")
              .arg("copy")
              .arg("-map")
              .arg("0")
              .arg("-r")
              .arg("15")
              .arg("-f")
              .arg("mp4")
              .arg(format!("{}/{}", files_folder, file_name)).spawn();

          std::thread::sleep(Duration::from_secs(video_duration_secs));
      }
}
