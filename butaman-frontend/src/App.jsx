import { useEffect, useState } from 'react';

function App() {
  const [states, setStates] = useState({});

  useEffect(() => {
    const fetchStates = async () => {
      const res = await fetch("http://localhost:8080/api/state");
      const data = await res.json();
      setStates(data);
    };

    fetchStates();
    const interval = setInterval(fetchStates, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Butaman Dashboard</h1>
      <table className="table-auto w-full border border-collapse border-gray-300">
        <thead>
          <tr>
            <th className="border px-2">IP</th>
            <th className="border px-2">Name</th>
            <th className="border px-2">RTT</th>
          </tr>
        </thead>
        <tbody>
          {Object.entries(states).map(([ip, state]) => (
            <tr key={ip}>
              <td className="border px-2">{ip}</td>
              <td className="border px-2">{state.name}</td>
              <td className="border px-2">
                {state.history.at(-1) === -1 ? "×" : `${state.history.at(-1)}ms`}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
