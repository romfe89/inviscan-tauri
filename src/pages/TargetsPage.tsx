import TargetsTable from "../components/TargetsTable";

const TargetsPage = () => (
  <>
    <main className="p-4 sm:p-6 w-full">
      <h2 className="text-xl sm:text-2xl font-bold mb-4 text-center sm:text-left">
        Alvos
      </h2>
      <div className="bg-white shadow rounded-lg p-4 sm:p-6 w-full">
        <TargetsTable />
      </div>
    </main>
  </>
);

export default TargetsPage;
