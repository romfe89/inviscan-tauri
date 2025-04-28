use crate::utils::{log_info, log_warning};
use std::collections::HashSet;
use std::process::Command;

#[tauri::command]
pub fn enumerate_subdomains(domain: &String) -> Result<Vec<String>, String> {
    let base_domain = domain.trim_start_matches("www.").to_string();
    let mut results: Vec<String> = Vec::new();

    log_info("Enumerando subdomínios com subfinder...");
    match run_tool("subfinder", &["-d", &base_domain]) {
        Ok(subfinder_out) => results.extend(subfinder_out),
        Err(e) => log_warning(&format!("Falha no subfinder: {:?}", e)),
    }

    log_info("Enumerando subdomínios com assetfinder...");
    match run_tool("assetfinder", &["--subs-only", &base_domain]) {
        Ok(assetfinder_out) => results.extend(assetfinder_out),
        Err(e) => log_warning(&format!("Falha no assetfinder: {:?}", e)),
    }

    log_info("Buscando subdomínios via crt.sh...");
    match query_crtsh(&base_domain) {
        Ok(crtsh_out) => results.extend(crtsh_out),
        Err(e) => log_warning(&format!("Falha no crt.sh: {:?}", e)),
    }

    let unique: HashSet<String> = results.into_iter().collect();
    Ok(unique.into_iter().collect())
}

fn run_tool(tool: &str, args: &[&str]) -> Result<Vec<String>, std::io::Error> {
    log_info(&format!("Executando: {} {}", tool, args.join(" ")));
    let output = Command::new(tool).args(args).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.contains('@'))
        .collect();

    Ok(lines)
}

fn query_crtsh(domain: &str) -> Result<Vec<String>, String> {
    let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
    log_info(&format!("Requisitando: {}", url));

    let output = Command::new("curl")
        .args(&["--compressed", "-s", &url])
        .output()
        .map_err(|e| format!("Erro ao executar curl: {:?}", e))?;

    let response_body = String::from_utf8_lossy(&output.stdout);

    if response_body.trim().is_empty() {
        log_warning("Resposta do crt.sh vazia.");
        return Ok(Vec::new());
    }

    // log_info(&format!("Resposta do crt.sh: {}", response_body));

    let json: serde_json::Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Erro ao interpretar JSON do crt.sh: {:?}", e))?;

    let mut results = Vec::new();

    if let Some(array) = json.as_array() {
        for entry in array {
            if let Some(name_value) = entry.get("name_value") {
                if let Some(name_str) = name_value.as_str() {
                    for name in name_str.split('\n') {
                        let clean_name = name.replace("*.", "").trim().to_string();
                        if !clean_name.is_empty() && !clean_name.contains('@') {
                            results.push(clean_name);
                        }
                    }
                }
            }
        }
    }

    Ok(results)
}
