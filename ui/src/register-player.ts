import { LitElement, html } from "lit";
import {customElement, property} from 'lit/decorators.js';
import { AppWebsocket } from '@holochain/conductor-api';

@customElement('register-player')
export class RegisterPlayer extends LitElement{
  @property()
  name = "";

  render() {
    return html `
      <span>
        <input placeholder="Enter your alias" value="${this.name}" @input=${this.updateName} >
        <button @click="${this.enterPool}">Join</button>
      </span>
    `;
  }

  updateName(e: { target: HTMLInputElement}) {
    this.name = e.target.value;
    console.log(this.name)
  }

  async enterPool() {
    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

    const appInfo = await appWebsocket.appInfo({installed_app_id: 'co-learning'});

    const cellData = appInfo.cell_data[0];
    
    await appWebsocket.callZome({
      cap: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'my_zome',
      fn_name: 'create_and_hash_entry_player_profile',
      payload: this.name,
      provenance: cellData.cell_id[1],
    });
    this.joined = true;
    console.log("Joined");
  }
}