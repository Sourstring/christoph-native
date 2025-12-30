import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
    Folder,
    File,
    ArrowLeft,
    HardDrive,
    Calendar,
    Shield,
} from "lucide-react";
import { List, AutoSizer } from "react-virtualized";
import "react-virtualized/styles.css";
import { FileUploader } from "./FileUploader";
import { FileDownloader } from "./FileDownloader";

interface FileInfo {
    name: string;
    path: string;
    size: number;
    is_dir: boolean;
    modified: number;
    permissions: string;
}

export function FileBrowser({ connectionId }: { connectionId: string }) {
    const [files, setFiles] = useState<FileInfo[]>([]);
    const [currentPath, setCurrentPath] = useState("/");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState("");

    const loadDirectory = async (path: string) => {
        setLoading(true);
        setError("");
        try {
            const result = await invoke<FileInfo[]>("list_directory", {
                connectionId,
                path,
            });
            setFiles(result);
            setCurrentPath(path);
        } catch (err) {
            setError(String(err));
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        loadDirectory("/");
    }, [connectionId]);

    const handleItemClick = (file: FileInfo) => {
        if (file.is_dir) {
            loadDirectory(file.path);
        }
    };

    const formatSize = (bytes: number) => {
        if (bytes === 0) return "-";
        const k = 1024;
        const sizes = ["B", "KB", "MB", "GB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
    };

    const formatDate = (timestamp: number) => {
        return new Date(timestamp * 1000).toLocaleString();
    };

    const rowRenderer = ({ index, key, style }: any) => {
        const file = files[index];
        const isParent = file.name === "..";

        return (
            <div
                key={key}
                style={{
                    ...style,
                    display: "flex",
                    alignItems: "center",
                    gap: "12px",
                    padding: "8px 16px",
                    borderBottom: "1px solid #333",
                    background: index % 2 === 0 ? "#1a1a1a" : "#0f0f0f",
                }}
                onMouseEnter={(e) => {
                    e.currentTarget.style.background = "#2a2a2a";
                }}
                onMouseLeave={(e) => {
                    e.currentTarget.style.background =
                        index % 2 === 0 ? "#1a1a1a" : "#0f0f0f";
                }}
            >
                <div
                    style={{
                        width: "24px",
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "center",
                    }}
                >
                    {isParent ? (
                        <ArrowLeft size={18} color="#888" />
                    ) : file.is_dir ? (
                        <Folder size={18} color="#4a9eff" />
                    ) : (
                        <File size={18} color="#888" />
                    )}
                </div>

                <div
                    style={{ flex: 1, minWidth: 0, cursor: "pointer" }}
                    onClick={() => handleItemClick(file)}
                >
                    <div
                        style={{
                            fontWeight: file.is_dir ? "bold" : "normal",
                            color: file.is_dir ? "#4a9eff" : "#fff",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                            whiteSpace: "nowrap",
                        }}
                    >
                        {file.name}
                    </div>
                </div>

                {!isParent && (
                    <>
                        <div
                            style={{
                                width: "100px",
                                textAlign: "right",
                                color: "#888",
                                fontSize: "12px",
                            }}
                        >
                            {file.is_dir ? "-" : formatSize(file.size)}
                        </div>

                        <div
                            style={{
                                width: "80px",
                                display: "flex",
                                alignItems: "center",
                                gap: "4px",
                                color: "#888",
                                fontSize: "11px",
                            }}
                        >
                            <Shield size={12} />
                            {file.permissions}
                        </div>

                        <div
                            style={{
                                width: "180px",
                                display: "flex",
                                alignItems: "center",
                                gap: "4px",
                                color: "#888",
                                fontSize: "11px",
                            }}
                        >
                            <Calendar size={12} />
                            {formatDate(file.modified)}
                        </div>

                        {!file.is_dir && (
                            <div
                                style={{
                                    width: "100px",
                                    display: "flex",
                                    justifyContent: "flex-end",
                                }}
                                onClick={(e) => e.stopPropagation()}
                            >
                                <FileDownloader
                                    connectionId={connectionId}
                                    remotePath={file.path}
                                    fileName={file.name}
                                />
                            </div>
                        )}
                    </>
                )}
            </div>
        );
    };

    const renderPathBreadcrumbs = () => {
        // Split path and filter out empty strings
        const segments = currentPath.split("/").filter(Boolean);

        return (
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                {/* Root */}
                <span
                    onClick={() => loadDirectory("/")}
                    style={{
                        cursor: "pointer",
                        fontWeight: currentPath === "/" ? "bold" : "normal",
                    }}
                >
                    /
                </span>

                {/* Other segments */}
                {segments.map((segment, index) => {
                    // Build the full path up to this segment
                    const pathUpToHere = "/" + segments.slice(0, index + 1).join("/");

                    return (
                        <div
                            key={index}
                            style={{ display: "flex", alignItems: "center", gap: "8px" }}
                        >
                            {index > 0 && <span style={{ color: "#666" }}>/</span>}
                            <span
                                onClick={() => loadDirectory(pathUpToHere)}
                                style={{
                                    cursor: "pointer",
                                    fontWeight: index === segments.length - 1 ? "bold" : "normal",
                                }}
                            >
                                {segment}
                            </span>
                        </div>
                    );
                })}
            </div>
        );
    };

    return (
        <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
            <div
                style={{
                    padding: "16px",
                    borderBottom: "2px solid #333",
                    display: "flex",
                    alignItems: "center",
                    gap: "16px",
                    background: "#0a0a0a",
                }}
            >
                <div style={{ display: "flex", alignItems: "center", gap: "8px", flex: 1 }}>
                    <HardDrive size={20} color="#4a9eff" />
                    <span style={{ fontFamily: "monospace", color: "#4a9eff" }}>
                        {renderPathBreadcrumbs()}
                    </span>
                </div>
                <FileUploader
                    connectionId={connectionId}
                    currentPath={currentPath}
                    onUploadComplete={() => loadDirectory(currentPath)}
                />
            </div>

            {error && (
                <div
                    style={{
                        padding: "12px",
                        background: "#ff000020",
                        border: "1px solid #ff0000",
                        color: "#ff6666",
                        margin: "16px",
                        borderRadius: "4px",
                    }}
                >
                    {error}
                </div>
            )}

            {loading ? (
                <div style={{ padding: "32px", textAlign: "center", color: "#888" }}>
                    Loading...
                </div>
            ) : (
                <div style={{ flex: 1 }}>
                    <AutoSizer>
                        {({ height, width }) => (
                            <List
                                width={width}
                                height={height}
                                rowCount={files.length}
                                rowHeight={50}
                                rowRenderer={rowRenderer}
                            />
                        )}
                    </AutoSizer>
                </div>
            )}
        </div>
    );
}
