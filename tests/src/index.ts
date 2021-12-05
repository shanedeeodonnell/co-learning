
import { Orchestrator } from "@holochain/tryorama";

import my_zome from './my-dna/my_zome';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
my_zome(orchestrator);
orchestrator.run();



