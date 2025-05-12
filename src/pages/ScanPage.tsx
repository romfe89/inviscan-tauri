import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
const ScanPage = () => {
  const [url, setUrl] = useState("");
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const handleScan = async () => {
    if (!url.trim()) {
      setMessage("Digite uma URL válida.");
      return;
    }

    setLoading(true);
    setMessage("Varredura em andamento...");

    try {
      const result = await invoke<string>("run_full_scan_command", {
        domain: url.trim(),
      });
      setMessage(result);
      // navigate(`/resultados/${url.trim()}`); // Ative se quiser redirecionar
    } catch (error) {
      console.error(error);
      setMessage("Erro ao iniciar a varredura: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleScan();
    }
  };

  return (
    <>
      <section className="bg-white shadow rounded-lg p-4 sm:p-6 max-w-2xl mx-auto mt-4">
        <h3 className="text-lg sm:text-xl font-semibold mb-4 text-center sm:text-left">
          Nova Varredura
        </h3>
        <div className="flex flex-col sm:flex-row gap-4 items-stretch sm:items-center">
          <input
            type="text"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Digite a URL do alvo..."
            disabled={loading}
            className="flex-1 px-4 py-2 border border-zinc-300 rounded-md text-sm sm:text-base"
          />
          <button
            onClick={handleScan}
            disabled={loading}
            className={`px-4 py-2 rounded-md shadow text-white text-sm sm:text-base ${
              loading
                ? "bg-gray-400 cursor-not-allowed"
                : "bg-blue-600 hover:bg-blue-700"
            }`}
          >
            {loading ? "Escaneando..." : "Escanear"}
          </button>
          {loading && (
            <div className="mx-auto sm:ml-2 animate-spin border-2 border-t-transparent border-zinc-400 rounded-full h-5 w-5"></div>
          )}
        </div>
        {message && (
          <p
            className={`mt-4 text-sm sm:text-base ${
              loading ? "text-blue-600" : "text-zinc-600"
            } text-center sm:text-left`}
          >
            {message}
          </p>
        )}
      </section>
    </>
  );
};

export default ScanPage;
