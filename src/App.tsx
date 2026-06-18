import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "beercss";
import "material-dynamic-colors";
import "./App.scss";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const appWindow = getCurrentWindow();

  async function greet() {
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <>
      <header class="top">
        <nav data-tauri-drag-region>
          <button class="circle transparent">
            <i>menu</i>
          </button>
          <div class="max" />
          <button
            class="circle transparent"
            onClick={() => appWindow.minimize()}
          >
            <i>remove</i>
          </button>
          <button
            class="circle transparent"
            onClick={() => appWindow.toggleMaximize()}
          >
            <i>check_box_outline_blank</i>
          </button>
          <button class="circle transparent" onClick={() => appWindow.close()}>
            <i>close</i>
          </button>
        </nav>
      </header>
      <main class="responsive" style="display: flex; flex-direction: row">
        <div>
          <h3>drawer</h3>
        </div>
        <div>
          <h3>main content</h3>
        </div>
      </main>
    </>
  );
}

export default App;
