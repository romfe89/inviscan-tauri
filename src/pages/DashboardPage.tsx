import StatusCard from "../components/StatusCard";

const DashboardPage = () => (
  <>
    <main className="p-4 sm:p-6 max-w-3xl mx-auto">
      <h2 className="text-xl sm:text-2xl font-bold mb-4 text-center sm:text-left">
        Dashboard
      </h2>
      <div className="bg-white shadow rounded-lg p-4 sm:p-6 mb-2">
        <p className="text-sm sm:text-base text-zinc-700">
          Bem-vindo ao Inviscan! Use o menu para iniciar uma nova varredura ou
          ver resultados anteriores.
        </p>
      </div>
      <StatusCard />
    </main>
  </>
);

export default DashboardPage;
