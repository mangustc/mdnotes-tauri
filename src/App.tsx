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
        <div
          style={{
            "background-color": "var(--md-sys-color-primary-container, red)",
            "font-size": "var(--md-sys-typescale-body-medium-size, 400px)",
          }}
        >
          My Notes
        </div>
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
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <md-outlined-text-field
          onChange={(e) => setName(e.currentTarget.value)}
          label="Name"
        />
        <md-filled-icon-button type="submit">
          <md-icon>send</md-icon>
        </md-filled-icon-button>
      </form>
      <p>{greetMsg()}</p>
    </main>
  );
}

export default App;
