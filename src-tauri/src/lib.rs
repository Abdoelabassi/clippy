// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::{remove_file, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use arboard::Clipboard;
use tauri::ipc::Channel;
use serde::{Serialize, Deserialize}



#[derive(Serialize, Deserialize)]
struct ClipboardHistory {
    items: Vec<String>,
}
const PATH: &str = "/Users/mac/Desktop/clipboard-manager/clippy/clipboad_history.json"

#[tauri::command]
fn wipe_all(){
    let _ = remove_file(PATH);
}
#[tauri::command]
fn copy(data: String){
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(data).unwrap();
}
#[tauri::command]
fn load_last_n_entries(n: usize) -> Vec<String>{
    if let Ok(history) = load_history(){
        history.items.into_iter().rev().take(n).collect()
    }else{
        vec![]
    }
}

fn init(on_event: Channel<String>){
    spawn(move || {
        let mut clipboard = Clipboard::new()::unwrap();
        let mut last_text = String::new();
        loop {
            if let Ok(text) = clipboard.get_text() {
                if text != last_text {
                    last_text = text.clone();
                    on_event.send(text).unwrap();
                    let mut history = load_history().unwrap_or(ClipboardHistory { items: vec![] });
                    history.items.push(text);
                    save_history(&history).unwrap();
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    })
}

fn load_history() -> Result<ClipboardHistory, std::io::Error>{
    let file = File::open(PATH)?;
    let reader = Buffer::new(file);
    let history = serde_json::from_reader(reader)?;
    Ok(history)
}

fn save_history(history: &ClipboardHistory) -> Result<(), std::io::Error>{
    let file = OpenOptions::nre()
        .create(true)
        .write(true)
        .truncate(true)
        .open(PATH)?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, history)?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![wipe_all, copy, load_last_n_entries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
