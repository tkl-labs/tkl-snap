"use client";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
    const [imageSrc, setImageSrc] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  async function get_image() {
      // invoke returns the base64 image string
      const base64Img: string = await invoke("get_image");
      // prefix with proper data URI to display
      setImageSrc(`data:image/png;base64,${base64Img}`);
  }

  return (
    <main className="container">
      <h1>Welcome to TKL Snap</h1>

      <div className="row">
        <a href="https://tauri.app" target="_blank" rel="noreferrer">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
      </div>
      <p>Click on the logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <button onClick={get_image}>Get Image</button>

      {imageSrc && <img src={imageSrc} alt="Generated from Tauri" />}
    </main>
  );
}

export default App;
