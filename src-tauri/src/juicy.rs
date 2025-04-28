use crate::utils::{log_error, log_success};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[tauri::command]
pub fn filter_juicy_targets(sites: &Vec<String>, output_dir: &Path) -> Result<Vec<String>, String> {
    let terms = vec![
        "dev",
        "dev1",
        "dev2",
        "dev3",
        "development",
        "test",
        "testing",
        "qa",
        "staging",
        "hml",
        "sandbox",
        "demo",
        "preview",
        "beta",
        "alpha",
        "preprod",
        "uat",
        "jenkins",
        "git",
        "gitlab",
        "bitbucket",
        "ci",
        "cicd",
        "pipeline",
        "artifactory",
        "nexus",
        "registry",
        "docker",
        "harbor",
        "login",
        "signin",
        "auth",
        "authentication",
        "sso",
        "saml",
        "oauth",
        "register",
        "signup",
        "password",
        "reset",
        "forgot",
        "token",
        "vpn",
        "remote",
        "access",
        "gateway",
        "firewall",
        "admin",
        "adminpanel",
        "manage",
        "dashboard",
        "console",
        "cms",
        "intranet",
        "internal",
        "private",
        "secure",
        "portal",
        "support",
        "help",
        "helpdesk",
        "it",
        "ticket",
        "jira",
        "confluence",
        "servicenow",
        "db",
        "database",
        "mysql",
        "postgres",
        "mongo",
        "sql",
        "redis",
        "api",
        "backend",
        "tools",
        "monitoring",
        "status",
        "uptime",
        "metrics",
        "grafana",
        "prometheus",
        "logs",
        "log",
        "kibana",
        "elastic",
        "public",
        "static",
        "files",
        "uploads",
        "content",
        "assets",
        "media",
        "old",
        "backup",
        "bak",
        "temp",
        "tmp",
        "archive",
    ];

    let mut juicy = Vec::new();

    for site in sites {
        for term in &terms {
            if site.contains(term) {
                juicy.push(site.clone());
                break;
            }
        }
    }

    let mut juicy_file = PathBuf::from(output_dir);
    juicy_file.push("juicytargets.txt");

    if let Err(e) = fs::write(&juicy_file, juicy.join("\n")) {
        log_error(&format!("Erro ao salvar juicytargets.txt: {:?}", e));
        return Err(format!("Erro ao salvar juicytargets.txt: {:?}", e));
    }

    log_success(&format!("Juicy targets encontrados: {}", juicy.len()));
    Ok(juicy)
}
