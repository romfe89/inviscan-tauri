import Header from "../components/Header";
import TargetsTable from "../components/TargetsTable";

const TargetsPage = () => (
  <>
    <Header />
    <main className="p-4 sm:p-6 max-w-3xl mx-auto">
      <h2 className="text-xl sm:text-2xl font-bold mb-4 text-center sm:text-left">
        Alvos
      </h2>
      <div className="bg-white shadow rounded-lg p-4 sm:p-6">
        <p className="text-sm sm:text-base text-zinc-700">
          <TargetsTable />
        </p>
      </div>
    </main>
  </>
);

export default TargetsPage;
