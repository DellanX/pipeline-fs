import { CapacityInfo } from "./CapacityInfo.js";
import { UsageInfo } from "./UsageInfo.js";


export interface StageSpace {
    path: string;
    filesystemId: number | null;
    capacity: CapacityInfo;
    usage: UsageInfo;
    fileCount: number | null;
    error: string | null;
}
