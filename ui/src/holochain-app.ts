import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { AppSignal, AppWebsocket } from '@holochain/conductor-api';
import './register-player';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @property()
  name?: string;

  @property()
  joined = false;

  async firstUpdated() {

    const signalCb = (signal: AppSignal) => {
      console.log(JSON.stringify(signal, null, 2));
      let colour: String = signal.data.payload
      alert(`A ${colour} Ping received!`);

    }

    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`, 12000, signalCb);
    const appInfo = await appWebsocket.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];

  }

  // this gets called on button click
  async ping() {
    //alert();
    const apws = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);
    const appInfo = await apws.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];
    await apws.callZome({
      cap: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'my_zome',
      fn_name: 'ui_send_ping',
      payload: '',
      provenance: cellData.cell_id[1],
    });
  }

  render() {
    return html`
      <main>
        <h1>${"Signals and Cap Grants"}</h1>
        <register-player></register-player>

      </main>

      <p class="app-footer">
        🚽 Made with love by
        <a
          target="_blank"
          rel="noopener noreferrer"
          href="https://github.com/open-wc"
          >open-wc</a>
      </p>
    `;
  }

  static styles = css`
    :host {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
      font-size: calc(10px + 2vmin);
      color: #1a2b42;
      max-width: 960px;
      margin: 0 auto;
      text-align: center;
      background-color: var(--lit-element-background-color);
    }

    main {
      flex-grow: 1;
    }

    .app-footer {
      font-size: calc(12px + 0.5vmin);
      align-items: center;
    }

    .app-footer a {
      margin-left: 5px;
    }
  `;
}
