import { useNavigate } from "react-router-dom";
import Header from "../components/Header";

interface Result {
  domain: string;
  timestamp: string;
  subdomains: number;
  activeSites: number;
  juicyTargets: number;
  path: string;
}

const ResultsPage = ({
  results,
  error,
}: {
  results: any[];
  error: string | null;
}) => {
  const navigate = useNavigate();

  return (
    <div className="flex-1">
      <Header />
      <main className="p-4 sm:p-6 max-w-4xl mx-auto mt-4">
        <section className="bg-white shadow rounded-lg p-4 sm:p-6">
          <h2 className="text-lg sm:text-xl font-bold mb-4 text-center sm:text-left">
            Resultados Anteriores
          </h2>
          {error && (
            <p className="text-red-600 text-sm sm:text-base">{error}</p>
          )}
          {results.length === 0 && !error && (
            <p className="text-zinc-600 text-sm sm:text-base">
              Nenhum resultado encontrado.
            </p>
          )}
          <ul className="divide-y divide-zinc-200">
            {results.map((r, i) => (
              <li
                key={i}
                onClick={() => navigate(`/resultados/${r.path}`)}
                className="p-3 sm:p-4 cursor-pointer hover:bg-zinc-100 rounded-md transition-colors"
              >
                <div className="font-semibold text-sm sm:text-base">
                  {r.domain}
                </div>
                <div className="text-xs sm:text-sm text-zinc-600 mt-1">
                  Data: {r.timestamp} | Subdomínios: {r.subdomains} | Ativos:{" "}
                  {r.activeSites} | Juicy: {r.juicyTargets}
                </div>
              </li>
            ))}
          </ul>
        </section>
      </main>
    </div>
  );
};

export default ResultsPage;
