use crate::compare::compare_with_previous;
use crate::ffuf::run_ffuf;
use crate::juicy::filter_juicy_targets;
use crate::probing::probe_active_sites;
use crate::screenshot::capture_screenshots;
use crate::subdomains::enumerate_subdomains;
use crate::utils::{log_info, log_success, remove_duplicates};
use chrono::Local;
use dirs_next as dirs;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ScanResult {
    status: String,
    data: Vec<String>,
}

#[tauri::command]
pub async fn run_full_scan_command(domain: String) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        run_full_scan(domain.clone())
            .map(|_| format!("Scan concluído para: {}", domain))
            .map_err(|e| format!("Erro: {:?}", e))
    })
    .await
    .map_err(|e| format!("Erro ao executar scan: {:?}", e))?;

    result
}

pub fn run_full_scan(domain: String) -> Result<String, String> {
    log_info(&format!("Iniciando scan para: {}", domain));

    let base_domain = domain.trim_start_matches("www.").to_string();
    log_info(&format!("Domínio base utilizado: {}", base_domain));

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let home_dir = dirs::home_dir().ok_or("Não foi possível localizar o diretório HOME")?;
    let output_dir: PathBuf = home_dir
        .join("inviscan")
        .join("resultados")
        .join(format!("{}_{}", domain, timestamp));
    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    // 1. Enumerar subdomínios
    let mut all_subdomains = enumerate_subdomains(&base_domain)?;
    log_success(&format!(
        "Total de subdomínios encontrados: {}",
        all_subdomains.len()
    ));

    // 2. Executar ffuf
    if let Ok(ffuf_out) = run_ffuf(&base_domain, &output_dir) {
        all_subdomains.extend(ffuf_out);
        all_subdomains = remove_duplicates(all_subdomains);
        log_success(&format!("Subdomínios após ffuf: {}", all_subdomains.len()));
    }

    // 3. Verificar sites ativos
    let active = probe_active_sites(&all_subdomains, &output_dir)?;
    log_success(&format!("Sites ativos encontrados: {}", active.len()));

    // 4. Filtrar juicy targets
    let juicy = filter_juicy_targets(&active, &output_dir)?;
    log_success(&format!("Juicy targets encontrados: {}", juicy.len()));

    // 5. Capturar screenshots
    capture_screenshots(&active, &output_dir)?;

    // 6. Comparar com scans anteriores
    compare_with_previous(&domain, &all_subdomains, &output_dir)?;

    // 7. Finalizar
    log_success(&format!(
        "Scan concluído: {} subdomínios | {} ativos | {} juicy",
        all_subdomains.len(),
        active.len(),
        juicy.len()
    ));
    log_success(&format!("Scan concluído para: {}", domain));

    Ok("Ok".to_string())
}
