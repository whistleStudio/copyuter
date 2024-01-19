#![allow(dead_code)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{collections::VecDeque, thread::sleep, time::Duration, fs::OpenOptions};

use tauri::{Manager, Window};
use rdev::{self, Event, simulate};


static mut IS_START: bool = false;
static mut EV_DEQ: VecDeque<Event> = VecDeque::new(); // 记录单个动作点
static mut ACT_VEC: Vec<VecDeque<Event>> = vec![]; // 记录完整动作队列
static mut PRE_EV: Option<Event> = None; // 记录前一个动作事件
static mut EV_VEC: Vec<rdev::Event> = vec![]; 

/* 动作监听 */
async fn listen_event () {
  rdev::listen(|ev|{
    unsafe {
      if IS_START == true {
        EV_VEC.push(ev);
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
  println!("22233");
}

/* 重复 */
#[tauri::command]
fn repeat (window: Window) {
  println!("repeat");
  window.minimize().expect("repeat mini err");
  sleep(Duration::from_millis(500));
  let mut pre_ev:Option<Event> = None;
  unsafe {
    // println!("{:?}", EV_VEC);
    for ev in &EV_VEC {
      if pre_ev != None {
        let inv = ev.time.duration_since(pre_ev.unwrap().time).unwrap();
        sleep(inv);
      }
      simulate(&ev.event_type).expect("repeat simulate err");
      pre_ev = Some(ev.clone());
    }
  }
}

#[tauri::command]
fn save () {
  let file = OpenOptions::new().read(true).write(true).append(true).create(true).open("events.log");
}

fn my_test () {

}


#[tokio::main]
async fn main() {
    my_test();
    println!("start");
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
