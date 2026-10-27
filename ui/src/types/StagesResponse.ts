import { StageSpace } from "./StageSpace.js";


export interface StagesResponse {
    stages: StageSpace[];
    inlet: StageSpace | null;
    outlet: StageSpace | null;
    staging: StageSpace | null;
}
