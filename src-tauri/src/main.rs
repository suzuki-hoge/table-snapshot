mod command;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::tmp,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
