import { LitElement, html } from "lit";
import {when} from 'lit/directives/when.js';
import {customElement, property} from 'lit/decorators.js';
import { AppWebsocket } from '@holochain/conductor-api';

@customElement('register-player')
export class RegisterPlayer extends LitElement{
  @property()
  name: string = "";
  @property()
  login: boolean = true;

  updateName(e: { target: HTMLInputElement}) {
    this.name = e.target.value;
  }

  render() {
    return html`
      ${when(this.login, () => html`      
        <span>
          <input placeholder="Enter your alias" value="${this.name}" @input=${this.updateName}>
          <button @click="${this.join}">Join</button>
        </span>`,
      () => html`
      <h3>Welcome ${this.name}!</h3>
        <span>
          <button @click="${this.sendPing}">PING!</button>
        </span>`)}
      `
  }

  async sendPing() {
    const apws = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);
    const appInfo = await apws.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];
    
    await apws.callZome({
      cap: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'ping',
      fn_name: 'ui_send_ping',
      payload: this.name,
      provenance: cellData.cell_id[1],
    });
  }

  async join() {
    console.log("Current name: ", this.name);
    const apws = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);
    const appInfo = await apws.appInfo({installed_app_id: 'co-learning'});
    const cellData = appInfo.cell_data[0];

    await apws.callZome({
      cap: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'ping',
      fn_name: 'join_with_code',
      payload: this.name,
      provenance: cellData.cell_id[1],
    });

    this.login = false;
  };
}