// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::{remove_file, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use arboard::Clipboard;
use tauri::ipc::Channel;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
struct ClipboardHistory {
    items: Vec<String>,
}
const PATH: &str = "/Users/mac/Desktop/clipboard-manager/clippy/clipboad_history.json";

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

#[tauri::command]
fn init(on_event: Channel<String>){
    std::thread::spawn(move || {
        let mut clipboard = Clipboard::new().unwrap();
        loop {
            if let Ok(text) = clipboard.get_text() {
                    let mut history = load_history().unwrap_or_else(|_| ClipboardHistory{ items: vec![] });
                    if history.items.last().map(|last| last != &text).unwrap_or(true){
                        history.items.push(text.clone());
                        save_history(&history).unwrap();
                        on_event.send(text).unwrap();
                    }
                
            }
            std::thread::sleep(std::time::Duration::from_millis(1000)); // equivalent to 2 seconds
        }
    });
}

fn load_history() -> Result<ClipboardHistory, std::io::Error>{
    let file = File::open(PATH)?;
    let reader = BufReader::new(file);
    let history = serde_json::from_reader(reader)?;
    Ok(history)
}

fn save_history(history: &ClipboardHistory) -> Result<(), std::io::Error>{
    let file = OpenOptions::new()
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
        .invoke_handler(tauri::generate_handler![wipe_all, copy, load_last_n_entries, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
