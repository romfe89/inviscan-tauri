import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface ScanResult {
  name: string;
  date: string;
  endpoints: number;
  juicy_targets: number;
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  const pad = (n: number) => n.toString().padStart(2, "0");
  return `${pad(date.getDate())}/${pad(
    date.getMonth() + 1
  )}/${date.getFullYear()} ${pad(date.getHours())}:${pad(
    date.getMinutes()
  )}:${pad(date.getSeconds())}`;
}

export default function TargetsTable() {
  const [scans, setScans] = useState<ScanResult[]>([]);

  useEffect(() => {
    invoke<ScanResult[]>("get_previous_scans")
      .then((res) => {
        console.log("Scans:", res);
        setScans(res);
      })
      .catch(console.error);
  }, []);

  return (
    <div className="p-4 w-full max-w-6xl mx-auto">
      <div className="bg-white shadow rounded-lg overflow-auto max-h-[70vh]">
        <table className="min-w-full text-sm text-left table-auto border-collapse">
          <thead className="bg-zinc-100 text-zinc-700 sticky top-0 z-10">
            <tr>
              <th className="px-4 py-3 border-b whitespace-nowrap">URL Alvo</th>
              <th className="px-4 py-3 border-b whitespace-nowrap">Data</th>
              <th className="px-4 py-3 border-b whitespace-nowrap">
                Resultados
              </th>
            </tr>
          </thead>
          <tbody>
            {scans.map((scan, i) => {
              return (
                <tr key={i} className="hover:bg-zinc-50">
                  <td className="px-4 py-2 border-b align-top">{scan.name}</td>
                  <td className="px-4 py-2 border-b align-top">
                    {formatDate(scan.date)}
                  </td>
                  <td className="px-4 py-2 border-b align-top whitespace-nowrap">
                    {scan.endpoints} endpoints / {scan.juicy_targets} juicy
                    targets
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
