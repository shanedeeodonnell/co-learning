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
      alert(`${signal.data.payload} sent a ping!`);
    };

    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`, 12000, signalCb);
    const appInfo = await appWebsocket.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];
  }

  render() {
    return html`
      <main>
        <h1>${"Signals and Cap Grants"}</h1>
        <register-player></register-player>
      </main>

      <p class="app-footer">
        Rearanging elecrons.
      </p>
    `;
  }

  static styles = css`
    main {
      flex-grow: 1;
    }

    .app-footer {
      font-size: calc(12px + 0.5vmin);
      align-items: center;
    }
  `;
}
