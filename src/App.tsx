import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.scss";
import "@material/web/all";
import { getCurrentWindow } from "@tauri-apps/api/window";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const appWindow = getCurrentWindow();

  async function greet() {
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <main class="app-container">
      <header data-tauri-drag-region class="desktop-app-bar">
        <span>My Notes</span>
        <div>
          <md-icon-button onClick={() => appWindow.minimize()}>
            <md-icon>remove</md-icon>
          </md-icon-button>
          <md-icon-button onClick={() => appWindow.toggleMaximize()}>
            <md-icon>check_box_outline_blank</md-icon>
          </md-icon-button>
          <md-icon-button onClick={() => appWindow.close()}>
            <md-icon>close</md-icon>
          </md-icon-button>
        </div>
      </header>
      <div class="app-drawer">
        <div></div>
        <div class="app-drawer-actions">
          <md-divider></md-divider>
          <md-filled-tonal-button style="width: 100%;">
            Create new note
            <md-icon slot="icon">create</md-icon>
          </md-filled-tonal-button>
          <div class="app-drawer-actions-row">
            <div class="app-drawer-actions-btn-group">
              <md-filled-tonal-button>
                Select project
                <md-icon slot="icon">folder_open</md-icon>
              </md-filled-tonal-button>
              <md-filled-tonal-icon-button>
                <md-icon>settings</md-icon>
              </md-filled-tonal-icon-button>
            </div>
            <md-outlined-icon-button>
              <md-icon>sync</md-icon>
            </md-outlined-icon-button>
          </div>
        </div>
      </div>
    </main>
  );
}

export default App;
