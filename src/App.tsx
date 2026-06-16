import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import "@material/web/textfield/outlined-text-field";
import "@material/web/iconbutton/filled-icon-button";
import "@material/web/icon/icon";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <main>
      <h1>Welcome to Tauri + Solid</h1>

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
