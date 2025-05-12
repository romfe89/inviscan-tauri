use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use chrono::{DateTime, Local};
use std::env;
use crate::utils::log_debug;

#[derive(Serialize)]
pub struct ScanResult {
    pub name: String,
    pub date: String,
    pub endpoints: usize,
    pub juicy_targets: usize,
}

#[tauri::command]
pub fn get_previous_scans() -> Result<Vec<ScanResult>, String> {
    let dir_path = get_resultados_path();
    let mut scans = Vec::new();

    log_debug(&format!("Lendo resultados em: {:?}", dir_path));

    if !dir_path.exists() {
        log_debug(&format!("Diretório não encontrado: {:?}", dir_path));
        return Ok(scans);
    }

    for entry in fs::read_dir(&dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
            let modified: DateTime<Local> = metadata.modified().map_err(|e| e.to_string())?.into();
            let date_str = modified.format("%Y-%m-%d %H:%M:%S").to_string();

            let endpoints = count_lines(&path.join("active_sites.txt"));
            let juicy_targets = count_lines(&path.join("juicytargets.txt"));

            log_debug(&format!(
                "Subscan '{}': {} endpoints, {} juicy targets",
                path.display(),
                endpoints,
                juicy_targets
            ));

            scans.push(ScanResult {
                name: path.file_name().unwrap_or_default().to_string_lossy().into(),
                date: date_str,
                endpoints,
                juicy_targets,
            });
        }
    }

    scans.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(scans)
}

fn get_resultados_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    Path::new(&home).join("inviscan").join("resultados")
}

fn count_lines(path: &PathBuf) -> usize {
    fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .count()
        })
        .unwrap_or(0)
}
