use crate::utils::{log_error, log_info, log_warning};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[tauri::command]
pub fn probe_active_sites(
    subdomains: &Vec<String>,
    output_dir: &Path,
) -> Result<Vec<String>, String> {
    log_info("Verificando sites ativos com httprobe...");

    let filtered_subdomains = filter_valid_domains(&subdomains);
    let mut active = Vec::new();

    for sub in filtered_subdomains {
        let mut cmd = match Command::new("httprobe")
            .arg("-t")
            .arg("5000")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(process) => process,
            Err(e) => {
                log_warning(&format!("Falha ao iniciar httprobe para {}: {:?}", sub, e));
                continue;
            }
        };

        {
            let stdin = cmd.stdin.as_mut().unwrap();
            stdin.write_all(format!("{}\n", sub).as_bytes()).unwrap();
        }

        let output = match cmd.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                log_warning(&format!("httprobe falhou para {}: {:?}", sub, e));
                continue;
            }
        };

        let reader = BufReader::new(&output.stdout[..]);
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim().to_string();
                if !trimmed.is_empty() {
                    active.push(trimmed);
                }
            }
        }
    }

    let mut active_file = PathBuf::from(output_dir);
    active_file.push("active_sites.txt");

    if let Err(e) = fs::write(&active_file, active.join("\n")) {
        log_error(&format!("Erro ao salvar active_sites.txt: {:?}", e));
    }

    Ok(active)
}

fn filter_valid_domains(domains: &Vec<String>) -> Vec<String> {
    domains
        .iter()
        .filter(|d| !d.is_empty() && !d.starts_with("http") && !d.contains('/') && !d.contains(' '))
        .cloned()
        .collect()
}
