const AboutPage = () => {
  return (
    <main className="max-w-3xl mx-auto p-6 mt-6">
      <section className="bg-white shadow rounded-lg p-6">
        <h1 className="text-2xl font-bold text-zinc-800 mb-4">
          Sobre o Inviscan
        </h1>
        <p className="text-zinc-700 text-base leading-relaxed mb-4">
          O <strong>Inviscan</strong> é uma ferramenta de varredura e análise de
          segurança projetada para identificar subdomínios ativos, endpoints
          acessíveis e alvos sensíveis em aplicações web.
        </p>
        <p className="text-zinc-700 text-base leading-relaxed mb-4">
          Ele combina funcionalidades modernas com uma interface gráfica leve e
          intuitiva, integrando tecnologias como{" "}
          <code className="bg-zinc-100 px-1 rounded text-sm">Tauri</code>,
          <code className="bg-zinc-100 px-1 rounded text-sm">React</code> e{" "}
          <code className="bg-zinc-100 px-1 rounded text-sm">Rust</code>.
        </p>
        <p className="text-zinc-700 text-base leading-relaxed">
          Desenvolvido para facilitar o uso de ferramentas ofensivas em
          ambientes controlados e auditorias de segurança, o Inviscan oferece
          uma maneira prática de organizar e visualizar os resultados de scans.
        </p>
      </section>
    </main>
  );
};

export default AboutPage;
