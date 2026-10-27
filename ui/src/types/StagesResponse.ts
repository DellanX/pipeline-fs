import type { StageSpace } from './StageSpace';

export interface StagesResponse {
    stages: StageSpace[];
    ioDirectory: StageSpace | null;
    ioError: string | null;
    totalCapacity: {
        filesystemCount: number;
        totalBytes: number;
        usedBytes: number;
        availableBytes: number;
    };
    directoryUsage: {
        configuredDirectoryCount: number;
        scannedDirectoryCount: number;
        fileCount: number;
        totalBytes: number;
    };
}
