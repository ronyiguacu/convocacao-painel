use tauri::{LogicalPosition, LogicalSize, Manager};
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
            // Janela estreita fixa, encostada na lateral direita, altura quase total.
            if let Some(win) = app.get_webview_window("main") {
                if let Ok(Some(monitor)) = win.primary_monitor() {
                    let escala = monitor.scale_factor();
                    let mw = monitor.size().width as f64 / escala;
                    let mh = monitor.size().height as f64 / escala;
                    let largura = 400.0;
                    let barra = 48.0; // espaco da barra de tarefas
                    let altura = (mh - barra).max(420.0);
                    let _ = win.set_resizable(true);
                    let _ = win.set_size(LogicalSize::new(largura, altura));
                    let _ = win.set_position(LogicalPosition::new((mw - largura).max(0.0), 0.0));
                    let _ = win.set_resizable(false);
                }
            }
            checar_atualizacao(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o painel de convocacao");
}
