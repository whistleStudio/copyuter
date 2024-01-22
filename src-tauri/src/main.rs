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
    println!("----- EV_VEC ------");
    // println!("{:?}", EV_VEC); 
    println!("-------------------");
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
    // println!("{:?}", EV_VEC);
    for ev in &EV_VEC {
      if let Some(_) = pre_ev {
        // let cur_t = serde_json::from_str::<SystemTime>(&ev.time).unwrap();
        // let pre_t = serde_json::from_str::<SystemTime>(&pre_ev.unwrap().time).unwrap();
        let inv = ev.time.duration_since(pre_ev.expect("1111").time).expect("2222");
        sleep(inv);
      }
      simulate(&ev.event_type).expect("repeat simulate err");
      pre_ev = Some(ev.clone());
    }
  }
}

#[tauri::command]
fn save () {

  // let t = SystemTime::now();
  // let serialized = serde_json::to_string(&t).unwrap();
  // let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();

  // println!("{:?}", serialized);
  // println!("{:?}", deserialized);

  unsafe {
    if EV_VEC.len() > 0 {
      // let e = format!("{:?}", EV_VEC);
      // let mut f = LOG_FILE.as_mut().unwrap();
      // f.write(e.as_bytes()).unwrap();
      let serialized = serde_json::to_string(&EV_VEC[0]).unwrap();
      let deserialized: Ev = serde_json::from_str(&serialized).unwrap();
      println!("{:?}", serialized);
      println!("{:?}", deserialized);
    }
    
    // let serialized = serde_json::to_string(&e).unwrap();

  }
}

fn init () {
  unsafe {
    let paths = fs::read_dir("ev_logs").unwrap();
    // println!("{:?}", paths.cloned());
    for path in paths {
      println!("{}", path.unwrap().path().display())
    }

    // LOG_FILE = OpenOptions::new().read(true).write(true).create(true).open("ev_logs/events.log").ok();
    // let mut buf = "".to_owned();
    // let _ = LOG_FILE.as_ref().unwrap().read_to_string(&mut buf);
  }
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
      .invoke_handler(tauri::generate_handler![start_record, stop_record, repeat, save])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
