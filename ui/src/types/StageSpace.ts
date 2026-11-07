export interface StageSpace {
    index: number | null;
    path: string;
    filesystemId: number | null;
    totalBytes: number | null;
    usedBytes: number | null;
    availableBytes: number | null;
    directoryBytes: number | null;
    fileCount: number | null;
    effectiveCapacityBytes: number | null;
    error: string | null;
    directoryError: string | null;
}
