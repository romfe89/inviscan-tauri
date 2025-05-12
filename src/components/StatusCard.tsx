import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const StatusCard = () => {
  const [ping, setPing] = useState("...");

  useEffect(() => {
    invoke<string>("ping")
      .then((message) => setPing(message))
      .catch((err) => {
        console.error("Erro ao chamar backend:", err);
        setPing("Offline");
      });
  }, []);

  return (
    <section className="bg-white shadow rounded-lg p-6">
      <h3 className="text-lg font-semibold mb-4">Status do Backend</h3>
      <p>
        Resposta do Backend: <strong>{ping}</strong>
      </p>
    </section>
  );
};

export default StatusCard;
