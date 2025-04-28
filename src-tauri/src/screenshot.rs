use crate::utils::{log_error, log_info};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[tauri::command]
pub fn capture_screenshots(sites: &Vec<String>, output_dir: &Path) -> Result<String, String> {
    if sites.is_empty() {
        log_info("Nenhum site ativo para capturar com gowitness.");
        return Ok("Nenhum site ativo para capturar.".to_string());
    }

    let gowitness_dir = PathBuf::from(&output_dir).join("gowitness");

    if let Err(e) = fs::create_dir_all(&gowitness_dir) {
        log_error(&format!("Erro ao criar diretório gowitness: {:?}", e));
        return Err(format!("Erro ao criar diretório gowitness: {:?}", e));
    }

    log_info("Preparando targets.txt para gowitness...");
    let targets_path = gowitness_dir.join("targets.txt");
    if let Err(e) = fs::write(&targets_path, sites.join("\n")) {
        log_error(&format!("Erro ao escrever targets.txt: {:?}", e));
        return Err(format!("Erro ao escrever targets.txt: {:?}", e));
    }

    log_info("Executando gowitness scan...");
    let cmd = Command::new("gowitness")
        .args(&[
            "scan",
            "file",
            "-f",
            "targets.txt",
            "--threads",
            "4",
            "--skip-html",
        ])
        .current_dir(&gowitness_dir)
        .output();

    match cmd {
        Ok(output) => {
            if !output.status.success() {
                log_error(&format!(
                    "gowitness retornou erro: {}",
                    String::from_utf8_lossy(&output.stdout)
                ));
                return Err(format!(
                    "falha ao executar gowitness scan: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Err(e) => {
            log_error(&format!("Erro ao executar gowitness: {:?}", e));
            return Err(format!("Erro ao executar gowitness: {:?}", e));
        }
    }

    let result_message = format!("Capturas salvas no diretório: {}", gowitness_dir.display());
    log_info(&result_message);
    Ok(result_message)
}
