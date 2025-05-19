import React, { useState, useEffect } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";
import "./App.css";

const App: React.FC = () => {
  const [clippboardItems, setClipboardItems] = useState<string[]>([]);
  const [filter, setFilter] = useState<number>(5);
  const [status, setStatus] = useState<string>("");

  const fetchClipboardHistory = async (n: number) => {
    try {
      const items: string[] = await invoke("load_last_n_entries", { n });
      setClipboardItems(items);
      setStatus(items.length > 0 ? "" : "No clipboard history found");
    } catch (error) {
      console.error("Error fetching clipboard history: ", error);
      setStatus("Error fetching clipboard history");
    }
  };

  const copyItemsToClipboard = async (data: string) => {
    try {
      await invoke("copy", { data });
    } catch (error) {
      console.error("Error copying to clipboard: ", error);
      setStatus("Error copying to clipboard");
    }
  };

  const wipeAllClipboardHistory = async () => {
    try {
      await invoke("wipe_all");
      setClipboardItems([]);
      setStatus("All clipboard history wiped");
    } catch (error) {
      console.error("Error wiping clipboard history: ", error);
      setStatus("Error wiping clipboard history");
    }
  };

  useEffect(() => {
    const initializeClipboard = async () => {
      try {
        const onEvent = new Channel<string>();
        onEvent.onmessage = (message: string) => {
          console.log("Clipboard updated");
          setClipboardItems((prevItems) => [message, ...prevItems]);
        };
        await invoke("init", { onEvent });
      } catch (error) {
        console.error("Error initializing clipboard: ", error);
        setStatus("Error initializing clipboard");
      }
    };
    initializeClipboard();
    fetchClipboardHistory(filter);

    return () => {
      console.log("Cleaning up on unmount");
    };
  }, [filter]);

  return (
    <div className="app">
      <header className="app-header">
        <h1>Clippy</h1>
        <p>Manage your clipboard history easy!!</p>
      </header>
      <main className="app-main">
        <div className="controls">
          <div className="filter-container">
            <label htmlFor="filter">Show last: </label>
            <select
              id="filter"
              value={filter}
              onChange={(e) => setFilter(Number.parseInt(e.target.value))}
            >
              <option value={5}>5</option>
              <option value={10}>10</option>
              <option value={20}>20</option>
              <option value={50}>50</option>
            </select>
          </div>
          <button onClick={wipeAllClipboardHistory} className="wipe-button">
            Wipe all
          </button>
        </div>
        {status && <p className="status">{status}</p>}
        <ul className="clipboard-list">
          {clippboardItems.map((item, idx) => (
            <li className="clipboard-item" key={idx}>
              <span className="item-text">{item}</span>
              <button
                onClick={() => copyItemsToClipboard(item)}
                className="copy-button"
              >
                Copy
              </button>
            </li>
          ))}
        </ul>
      </main>
    </div>
  );
};

export default App;
