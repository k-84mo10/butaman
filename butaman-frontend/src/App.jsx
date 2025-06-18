import { useEffect, useState } from 'react';

function ColoredBar({ rtt }) {
  if (rtt === -1) return <span className="text-gray-400">×</span>;

  let color = '';
  if (rtt < 40) color = 'text-green-500';
  else if (rtt < 100) color = 'text-yellow-500';
  else color = 'text-red-500';

  let symbol = '▁';
  if (rtt < 20) symbol = '▁';
  else if (rtt < 40) symbol = '▂';
  else if (rtt < 60) symbol = '▃';
  else if (rtt < 80) symbol = '▄';
  else if (rtt < 100) symbol = '▅';
  else if (rtt < 120) symbol = '▆';
  else if (rtt < 140) symbol = '▇';
  else symbol = '█';

  return <span className={`${color} text-lg`}>{symbol}</span>;
}

function App() {
  const [states, setStates] = useState({});

  useEffect(() => {
    const fetchStates = async () => {
      try {
        const res = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/state`);
        const data = await res.json();
        setStates(data);
      } catch (error) {
        console.error("Failed to fetch:", error);
      }
    };

    fetchStates();
    const interval = setInterval(fetchStates, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="p-6 bg-gray-100 min-h-screen">
      <h1 className="text-3xl font-bold mb-6 text-center">Butaman Dashboard</h1>
      <table className="table-auto w-full border border-collapse border-gray-300 bg-white shadow">
        <thead>
          <tr className="bg-gray-200">
            <th className="border px-3 py-2">Name</th>
            <th className="border px-3 py-2">IP</th>
            <th className="border px-3 py-2">Latest Success</th>
            <th className="border px-3 py-2">Latest RTT</th>
            <th className="border px-3 py-2">History</th>
          </tr>
        </thead>
        <tbody>
          {Object.entries(states).map(([ip, state]) => (
            <tr key={ip}>
              <td className="border px-3 py-1">{state.name}</td>
              <td className="border px-3 py-1">{ip}</td>
              <td className="border px-3 py-1">{state.last_success}</td>
              <td className="border px-3 py-1">
                {state.history.at(-1) === -1 ? "×" : `${state.history.at(-1)}ms`}
              </td>
              <td className="border px-3 py-1 whitespace-nowrap">
                {state.history.slice().reverse().map((rtt, i) => (
                  <ColoredBar key={i} rtt={rtt} />
                ))}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
