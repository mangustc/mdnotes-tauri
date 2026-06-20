import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "beercss";
import "material-dynamic-colors";
import "./App.scss";
import { AppError } from "./bindings/AppError";
import { Settings } from "./bindings/Settings";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const appWindow = getCurrentWindow();

  async function greet() {
    try {
      const msg = await invoke<Settings>("read");
      setGreetMsg(msg.syncProvider);
    } catch (e) {
      const error = e as AppError;
      console.log(error.type);
    }
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
      <main style="display: flex; flex-direction: row; width: 100vw; height: 100%; padding: 0;">
        <div style="display: flex; flex-direction: column; justify-content: space-between; width: 360px; height: 100%;">
          <h3>{greetMsg()}</h3>
          <div>
            <hr />
            <div style="padding: 16px">
              <button class="tetriary responsive" onClick={greet}>
                <i>create</i>
                Create new note
              </button>
            </div>
            <div style="display: flex; flex-direction: row; justify-content: space-between; padding: 0px 16px 16px 16px">
              <nav class="group split">
                <button class="left-round">
                  <i class="small">folder_open</i>
                  <span>project_name</span>
                </button>
                <button class="right-round square">
                  <i class="small fill">settings</i>
                </button>
              </nav>
              <button class="border square round extra">
                <i>sync</i>
              </button>
            </div>
          </div>
        </div>
        <div>
          <h3>main content</h3>
        </div>
      </main>
    </>
  );
}

export default App;
