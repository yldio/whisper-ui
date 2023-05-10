import {
  Component,
  For,
  createEffect,
  createResource,
  createSignal,
} from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import "./App.css";
import { createFileUploader } from "@solid-primitives/upload";

const SingleFileUpload: Component = () => {
  const { files, selectFiles } = createFileUploader();
  const { files: filesAsync, selectFiles: selectFilesAsync } =
    createFileUploader();

  return (
    <div>
      <div>
        <h5>Select a single file</h5>
        <button
          onClick={() => {
            selectFiles(([files]) => {
              console.log(files);
            });
          }}
        >
          Select
        </button>
        <For each={files()}>{(file) => <p>{file.name}</p>}</For>
      </div>

      <div>
        <h5>Select a single file with async callback</h5>
        <button
          onClick={() => {
            selectFilesAsync(async ([{ source, name, size, file }]) => {
              console.log({ source, name, size, file });
            });
          }}
        >
          Select
        </button>
        <For each={filesAsync()}>{(file) => <p>{file.name}</p>}</For>
      </div>
    </div>
  );
};

function App() {
  const [greetMsg, setGreetMsg] = createSignal<string[]>([]);
  const [setupRunning, setSetupRunning] = createSignal(false);
  const [count, setCount] = createSignal(0);

  createResource(
    async () =>
      await appWindow.listen<{ message: string }>("setup", (ev) => {
        setGreetMsg((prev) => [...prev, ev.payload.message]);
      })
  );

  const setup = async () => {
    setSetupRunning(true);
    const completed = await invoke("test", { appWindow });
    setCount(count() + 1);
  };

  createEffect(async () => {
    if (greetMsg().findLast((msg) => msg !== "Whisper setup complete ✅")) {
      console.log("olé" + count());
    }
  });

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <div class="row">
        <button type="button" onClick={() => !setupRunning() && setup()}>
          Setup
        </button>
      </div>

      <p>
        Count:
        {count()}
      </p>

      <SingleFileUpload />

      <For each={greetMsg()}>
        {(msg) => (
          <div class="row">
            <p>{msg}</p>
          </div>
        )}
      </For>
    </div>
  );
}

export default App;
