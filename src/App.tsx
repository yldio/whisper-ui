import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import {
  createEffect,
  createResource,
  createSignal,
  For,
  onMount,
  Show,
} from 'solid-js';
import Upload from './assets/upload.svg?component-solid';
import Footer from './components/Footer';
import Layout from './components/Layout';
import Loading from './components/Loading';

function App() {
  const [messages, setMessages] = createSignal<string[]>([]);
  const [running, setRunning] = createSignal(false);
  const [hovering, setHovering] = createSignal(false);
  const [disabledSetup, setDisabledSetup] = createSignal(false);
  let outputRef!: HTMLDivElement;

  const setup = async () => {
    setRunning(true);
    void invoke('setup', { appWindow });
  };

  createResource(
    async () =>
      await appWindow.listen<{ message: string }>('setup', (ev) => {
        setMessages((prev) => [...prev, ev.payload.message]);
      }),
  );
  createResource(
    async () =>
      await appWindow.listen<{ message: string }>('run', (ev) => {
        setMessages((prev) => [...prev, ev.payload.message]);
      }),
  );

  createResource(
    async () =>
      await appWindow.onFileDropEvent(async (event) => {
        if (event.payload.type === 'hover') {
          setHovering(true);
        } else if (event.payload.type === 'drop' && !running()) {
          setHovering(false);
          setMessages([]);
          setRunning(true);
          event.payload.paths.map(
            async (path) => void invoke('run', { appWindow, path }),
          );
        } else {
          setHovering(false);
        }
      }),
  );

  onMount(() => {
    setInterval(
      () => running() && (outputRef.scrollTop = outputRef.scrollHeight),
      1000,
    );
  });

  createEffect(async () => {
    if (messages().findLast((msg) => msg === 'Whisper setup complete ✅')) {
      setMessages([]);
      setRunning(false);
      setDisabledSetup(true);
    } else if (messages().findLast((msg) => msg === 'Success! 🎉')) {
      setMessages([]);
      setRunning(false);
    }
  });

  return (
    <Layout hovering={hovering()}>
      <Show
        when={!running()}
        fallback={<h1 class="pt-4 text-xl">Running...</h1>}
      >
        <h1 class="pt-4 text-xl">Drop a file anywhere to run</h1>
      </Show>

      <Show when={!disabledSetup()}>
        <div class="absolute right-5 top-5">
          <button
            class="rounded bg-aquamarine-500 px-3 py-2 font-semibold text-blackrock-950"
            type="button"
            onClick={() => {
              if (!running()) setup();
            }}
          >
            Setup
          </button>
        </div>
      </Show>

      <div class="mx-auto mb-1 mt-5 h-24 w-max">
        <Show when={!running()} fallback={<Loading />}>
          <Upload
            fill="white"
            class="absolute bottom-0 left-0 right-0 top-0 m-auto h-20"
          />
        </Show>
      </div>

      <Show when={running()}>
        <div
          class="no-scrollbar mx-6 h-36 overflow-y-auto bg-dovegray font-mono"
          ref={outputRef}
        >
          <For each={messages()}>{(msg) => <p>{msg}</p>}</For>
        </div>
      </Show>

      <Footer />
    </Layout>
  );
}

export default App;
