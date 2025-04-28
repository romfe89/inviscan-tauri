use crate::utils::{log_error, log_info};
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

#[tauri::command]
pub fn compare_with_previous(
    domain: &String,
    current: &Vec<String>,
    output_dir: &Path,
) -> Result<String, String> {
    let dir = PathBuf::from("resultados");
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            log_error(&format!("Erro ao ler diretório de resultados: {:?}", e));
            return Err(format!("Erro ao ler diretório de resultados: {:?}", e));
        }
    };

    let mut last_scan: Option<String> = None;

    for entry in entries.flatten() {
        let name = entry.file_name().into_string().unwrap_or_default();
        let path = entry.path();

        if path.is_dir()
            && name.starts_with(&(domain.clone() + "_"))
            && path != PathBuf::from(&output_dir)
        {
            if last_scan.is_none() || name > last_scan.clone().unwrap() {
                last_scan = Some(name);
            }
        }
    }

    let last_scan_name = match last_scan {
        Some(name) => name,
        None => {
            log_info("Nenhum scan anterior encontrado para comparação.");
            return Ok("Nenhum scan anterior encontrado para comparação.".to_string());
        }
    };

    let last_sub_file = dir.join(&last_scan_name).join("subdomains.txt");
    let last_subs = match read_lines(&last_sub_file) {
        Ok(lines) => lines,
        Err(e) => {
            log_error(&format!("Erro ao ler último subdomains.txt: {:?}", e));
            return Ok(format!("Erro ao ler último subdomains.txt: {:?}", e));
        }
    };

    let new_subs = diff_sorted(&last_subs, &current);

    log_info(&format!(
        "Novos subdomínios desde {}: {}",
        last_scan_name,
        new_subs.len(),
    ));
    for s in &new_subs {
        log_info(&format!("  + {}", s));
    }

    Ok(format!(
        "Novos subdomínios desde {}: {}",
        last_scan_name,
        new_subs.len()
    ))
}

fn read_lines(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if !line.is_empty() {
            lines.push(line);
        }
    }
    Ok(lines)
}

fn diff_sorted(old: &Vec<String>, current: &Vec<String>) -> Vec<String> {
    let old_set: HashSet<_> = old.iter().cloned().collect();
    current
        .iter()
        .filter(|s| !old_set.contains(*s))
        .cloned()
        .collect()
}
