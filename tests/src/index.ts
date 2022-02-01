
import { Orchestrator } from "@holochain/tryorama";

import ping from './my-dna/ping';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
ping(orchestrator);
orchestrator.run();



