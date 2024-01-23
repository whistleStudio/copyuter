#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{thread::sleep, time::{Duration, SystemTime}, fs, fs::{OpenOptions, File}, io::{Read, Write}};
use serde::{Serialize, Deserialize};
use tauri::{Manager, Window};
use rdev::{self, simulate, EventType};
use serde_json;



static mut IS_START: bool = false;
static mut EV_VEC: Vec<Ev> = vec![]; 
static mut LOG_FILE: Option<File> = None; // 记录文件





#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ev {
  time: SystemTime,
  event_type: EventType
}

/* 动作监听 */
async fn listen_event () {
  rdev::listen(|ev|{
    unsafe {
      if IS_START == true {
        EV_VEC.push(Ev {time: ev.time, event_type: ev.event_type});
      }
    }
  }).unwrap();
}

/* 开始录制 */
#[tauri::command]
fn start_record (window: Window) {
  window.minimize().expect("start mini err");
  unsafe {
    EV_VEC = vec![];
    IS_START = true; 
  }
}

/* 停止录制 */
#[tauri::command]
fn stop_record () {
  unsafe { 
    IS_START = false;
  }
}

/* 重复 */
#[tauri::command]
fn repeat (window: Window) {
  println!("repeat");
  window.minimize().expect("repeat mini err");
  sleep(Duration::from_millis(500));
  let mut pre_ev:Option<Ev> = None;
  unsafe {
    for ev in &EV_VEC {
      if let Some(_) = pre_ev {
        let inv = ev.time.duration_since(pre_ev.expect("1111").time).expect("2222");
        sleep(inv);
      }
      simulate(&ev.event_type).expect("repeat simulate err");
      pre_ev = Some(ev.clone());
    }
  }
}

/* 保存 */
#[tauri::command]
fn save () {
  unsafe {
    if EV_VEC.len() > 0 {
      LOG_FILE = OpenOptions::new().read(true).write(true).create(true).open("ev_logs/ev.log").ok();
      let serialized = serde_json::to_string(&EV_VEC).unwrap();
      LOG_FILE.as_ref().unwrap().write(serialized.as_bytes()).unwrap();
      // println!("{:?}", serialized);
      // // println!("{:?}", deserialized);
    }
  }
}

/* 获取文件名 */
#[tauri::command]
fn get_filenames () ->Vec<String> {
  let paths = fs::read_dir("ev_logs/").unwrap();
  let log_arr = paths.map(
    |f| f.unwrap().path().file_name().unwrap().to_str().unwrap().to_owned())
    .collect::<Vec<_>>();
  log_arr
}

/* 运行指定文件 */
#[tauri::command]
fn run_log (window: Window, f: String) -> i8{
  println!("{}", f);
  unsafe {
    LOG_FILE = OpenOptions::new().read(true).write(true).create(true).open(format!("{}{}", "ev_logs/", f)).ok();
    let mut buf = "".to_owned();
    LOG_FILE.as_ref().unwrap().read_to_string(&mut buf).unwrap();
    println!("{}", buf);
    serde_json::from_str::<Vec<Ev>>(&buf).map_or(-1, |v| {
      EV_VEC = v;
      repeat(window);
      0
    })
  }
}

fn init () {
  // let serialized = serde_json::to_string::<Vec<i32>>(&vec![]).unwrap();
  // let deserialized: Vec<i32> = serde_json::from_str(&serialized).unwrap();
}


#[tokio::main]
async fn main() {
    println!("start");

    init();
    tauri_run();

}

fn tauri_run () {
  tokio::spawn(listen_event());
  tauri::Builder::default()
      .setup(|app|{
          let main_window = app.get_window("main").unwrap();
          main_window.listen("event-name", |event| {
            println!("got window event-name with payload {:?}", event.payload());
          });
          main_window.minimize().expect("err");
          Ok(())
      })
      .invoke_handler(tauri::generate_handler![start_record, stop_record, repeat, save, get_filenames, run_log])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
