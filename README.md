# Inviscan

**Inviscan** é uma aplicação de varredura e análise de alvos, focada em subdomínios, endpoints ativos e possíveis pontos sensíveis. A interface é construída com React, Tailwind e Tauri, enquanto a lógica de backend é desenvolvida em Rust.

## 🚀 Requisitos

- [Node.js](https://nodejs.org/) **v22**
- [Bun](https://bun.sh) instalado globalmente
- Ambiente compatível com [Tauri](https://tauri.app) (verifique as dependências conforme seu sistema operacional)

## 📦 Instalação

Clone o repositório e instale as dependências com:

```bash
bun install
```

## 🧪 Executando em modo de desenvolvimento

Para rodar a aplicação com a interface gráfica:

```bash
bun run tauri dev
```

## 📁 Estrutura de dados

Todos os resultados de varreduras são salvos no diretório:

```bash
~/inviscan/resultados
```

Cada subpasta representa um scan, contendo arquivos como:

    - active_sites.txt: lista de sites ativos detectados

    - juicytargets.txt: possíveis alvos sensíveis encontrados

## 🛠 Tecnologias

    - ⚙️ Rust (backend)

    - ⚡ Bun (gerenciador de pacotes e runtime)

    - 🖥 React + TypeScript (frontend)

    - 🎨 Tailwind CSS (estilização)

    - 🪟 Tauri (empacotamento)
