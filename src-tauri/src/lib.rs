use tauri::{Manager, PhysicalPosition};
use tauri_plugin_updater::UpdaterExt as _;

fn checar_atualizacao(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        if let Ok(updater) = app.updater() {
            if let Ok(Some(update)) = updater.check().await {
                let _ = update.download_and_install(|_, _| {}, || {}).await;
                app.restart();
            }
        }
    });
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Posiciona a janela no canto superior direito da tela.
            if let Some(win) = app.get_webview_window("main") {
                if let (Ok(Some(monitor)), Ok(tam)) = (win.primary_monitor(), win.outer_size()) {
                    let tela = monitor.size();
                    let margem = 24i32;
                    let x = (tela.width as i32 - tam.width as i32 - margem).max(0);
                    let y = margem;
                    let _ = win.set_position(PhysicalPosition::new(x, y));
                }
            }
            checar_atualizacao(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o painel de convocacao");
}
