use crate::utils::{log_error, log_info, log_warning};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize)]
struct FFUFResult {
    results: Vec<FFUFEntry>,
}

#[derive(Deserialize)]
struct FFUFEntry {
    input: std::collections::HashMap<String, String>,
    host: Option<String>,
}

#[tauri::command]
pub fn run_ffuf(domain: &str, output_dir: &Path) -> Result<Vec<String>, String> {
    log_info("Enumerando subdomínios com ffuf...");

    let wordlist = "/snap/seclists/900/Discovery/DNS/subdomains-top1million-5000.txt";
    if !std::path::Path::new(wordlist).exists() {
        log_error(&format!("Wordlist não encontrada: {}", wordlist));
        return Ok(Vec::new());
    }

    let mut output_file = PathBuf::from(output_dir);
    output_file.push("ffuf.json");

    let base_domain = domain.trim_start_matches("www.").to_string();
    log_info(&format!(
        "Executando comando ffuf: ffuf -u http://FUZZ.{} -w {} -mc 200 -t 40 -of json -o {}",
        base_domain,
        wordlist,
        output_file.display()
    ));

    let ffuf_cmd = Command::new("ffuf")
        .args(&[
            "-u",
            &format!("http://FUZZ.{}", base_domain),
            "-w",
            wordlist,
            "-mc",
            "200",
            "-t",
            "40",
            "-of",
            "json",
            "-o",
            output_file.to_str().unwrap(),
        ])
        .output();

    match ffuf_cmd {
        Ok(output) => {
            if !output.status.success() {
                log_warning(&format!(
                    "ffuf falhou: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
                return Ok(Vec::new());
            }
        }
        Err(e) => {
            log_warning(&format!("Erro ao executar ffuf: {:?}", e));
            return Ok(Vec::new());
        }
    }

    let data = match fs::read_to_string(&output_file) {
        Ok(content) => content,
        Err(e) => {
            log_warning(&format!("Erro ao ler {}: {:?}", output_file.display(), e));
            return Ok(Vec::new());
        }
    };

    if data.trim().is_empty() {
        log_warning("ffuf não retornou resultados.");
        return Ok(Vec::new());
    }

    let parsed: FFUFResult = match serde_json::from_str(&data) {
        Ok(json) => json,
        Err(e) => {
            log_warning(&format!("Erro ao interpretar JSON do ffuf: {:?}", e));
            return Ok(Vec::new());
        }
    };

    let mut found = Vec::new();
    for r in parsed.results {
        if let Some(host) = r.host {
            found.push(host);
        } else if let Some(sub) = r.input.get("FUZZ") {
            found.push(format!("{}.{}", sub, base_domain));
        }
    }

    log_info(&format!("ffuf encontrou {} subdomínios", found.len()));
    Ok(found)
}
