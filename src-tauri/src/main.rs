// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod nodes;
mod config;

use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::nodes::{Node, Response};

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![process_answer, reset_node])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

lazy_static! {
    static ref CURRENT: Mutex<Option<Node>> = Mutex::new(Some(config::get_decision_tree()));
}

#[tauri::command]
fn reset_node() {
  let mut current = CURRENT.lock().unwrap();
  *current = Some(config::get_decision_tree());
}

#[tauri::command]
fn process_answer(answer: &str) -> Response {
  let mut current = CURRENT.lock().unwrap();

  if let Some(ref mut current_node) = *current {
    match answer {
      "Да" => current_node.go_yes(),
      "Нет" => current_node.go_no(),
      "Скорее да" => current_node.go_likely_yes(),
      "Скорее нет" => current_node.go_likely_no(),
      "start" =>  current_node.return_response(),
      _ => Response::default(),
    }
  } else {
    Response::default()
  }
}
