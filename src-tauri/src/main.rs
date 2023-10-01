// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod nodes;
mod config;

use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::nodes::Response;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![process_answer])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

lazy_static! {
     static ref CURRENT: Mutex<nodes::Node> = Mutex::new(config::get_decision_tree());
}

#[tauri::command]
fn process_answer(answer: &str) -> nodes::Response {
  let mut current = CURRENT.lock().unwrap();

  match answer {
    "yes" => {
      if current.yes_node.is_some() {
        current.go_yes()
      } else {
        current.return_response()
      }
    }
    "no" => {
      if current.no_node.is_some() {
        current.go_no()
      } else {
        current.return_response()
      }
    }
    "start" => {
      current.return_response()
    }
    _ => Response::default(),
  }
}
