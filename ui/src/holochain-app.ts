import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { AppSignal, AppWebsocket, InstalledCell } from '@holochain/client';
import "@holochain-open-dev/profiles/profile-prompt";
import "@holochain-open-dev/profiles/list-profiles";
import "@holochain-open-dev/context/context-provider";
import {
  ProfilesStore,
  profilesStoreContext,
} from "@holochain-open-dev/profiles";

import { StoreSubscriber } from "lit-svelte-stores";
import { HolochainClient } from "@holochain-open-dev/cell-client";
// import './register-player';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @property()
  store!: ProfilesStore;

  myProfile = new StoreSubscriber(this, () => this.store?.myProfile);

  async firstUpdated() {
    await this.setupProfiles();

    const signalCb = (signal: AppSignal) => {
      console.log(JSON.stringify(signal, null, 2));
      alert(`${signal.data.payload} sent a ping!`);
    };

    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`, 12000, signalCb);
    const appInfo = await appWebsocket.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];
  }

  async setupProfiles() {
    const client = await HolochainClient.connect(
      `ws://localhost:${process.env.HC_PORT}`,
      "co-learning"
    );
    const cellClient = client.forCell(client.cellDataByRoleId("my-dna") as InstalledCell);
  
    this.store = new ProfilesStore(cellClient, {
      avatarMode: "identicon",
    });
  }

  async sendPing() {
    const apws = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);
    const appInfo = await apws.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];
    
    await apws.callZome({
      cap_secret: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'profiles',
      fn_name: 'ui_send_ping',
      payload: this.myProfile.value?.nickname,
      provenance: cellData.cell_id[1],
    });
  }

  async sendDirectPing(event: CustomEvent) {
    const apws = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);
    const appInfo = await apws.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];
    
    await apws.callZome({
      cap_secret: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'ping',
      fn_name: 'ui_send_direct_ping',
      payload: event.detail.agentPubKey,
      provenance: cellData.cell_id[1],
    });
  }

  render() {
    if(!this.store) {
      return html`<p>Loading...</p>`;
    }
    return html`
      <main>
        <h1>${"Signals and Cap Grants"}</h1>
        <context-provider .value=${this.store} .context=${profilesStoreContext} id="profiles-context">
          <profile-prompt>
            <h2>Welcome ${this.myProfile.value?.nickname}</h2>
            <button @click="${this.sendPing}">PING ALL!</button>
            <list-profiles @agent-selected="${this.sendDirectPing}"></list-profiles>
          </profile-prompt>
        </context-provider>
      </main>

      <p class="app-footer">
      🚽 Rearranging Electrons
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
  `;
}
