import { useState } from "react";
import { Link } from "react-router-dom";

const Header = () => {
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <header className="bg-blue-600 text-white px-4 py-3 sm:px-6 flex justify-between items-center shadow relative">
      <h1 className="text-lg sm:text-xl font-semibold">Inviscan</h1>

      <button
        onClick={() => setMenuOpen(!menuOpen)}
        className="sm:hidden focus:outline-none"
      >
        <svg
          className="w-6 h-6"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
        >
          {menuOpen ? (
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          ) : (
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M4 6h16M4 12h16M4 18h16"
            />
          )}
        </svg>
      </button>

      <nav className="hidden sm:flex gap-4">
        <Link to="/" className="hover:underline">
          Dashboard
        </Link>
        <Link to="/alvos" className="hover:underline">
          Alvos
        </Link>
        <Link to="/varreduras" className="hover:underline">
          Varreduras
        </Link>
        <Link to="/resultados" className="hover:underline">
          Resultados
        </Link>
      </nav>

      {menuOpen && (
        <div className="absolute top-full left-0 w-full bg-blue-700 sm:hidden z-10">
          <nav className="flex flex-col p-4 space-y-2">
            <Link
              to="/"
              className="hover:underline"
              onClick={() => setMenuOpen(false)}
            >
              Dashboard
            </Link>
            <Link
              to="/alvos"
              className="hover:underline"
              onClick={() => setMenuOpen(false)}
            >
              Alvos
            </Link>
            <Link
              to="/varreduras"
              className="hover:underline"
              onClick={() => setMenuOpen(false)}
            >
              Varreduras
            </Link>
            <Link
              to="/resultados"
              className="hover:underline"
              onClick={() => setMenuOpen(false)}
            >
              Resultados
            </Link>
          </nav>
        </div>
      )}
    </header>
  );
};

export default Header;
