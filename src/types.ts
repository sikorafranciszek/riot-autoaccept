export type ConnectionStatus =
  | "stopped"
  | "searching"
  | "connecting"
  | "connected"
  | "disconnected"
  | "error";

export type MatchPhase =
  | "idle"
  | "in-queue"
  | "ready-check"
  | "in-champ-select"
  | "in-game"
  | "end-of-game";

export type QueueType = "league-of-legends" | "tft" | "aram" | "unknown";

export interface AppState {
  running: boolean;
  autoAcceptEnabled: boolean;
  connectionStatus: ConnectionStatus;
  matchPhase: MatchPhase;
  queueType: QueueType;
  clientPort: number | null;
  clientSource: string | null;
  queueDurationSec: number;
  acceptedCount: number;
  lastError: string | null;
  rawPhase: string;
  updatedAt: number;
}

export interface Settings {
  autoAcceptEnabled: boolean;
  startWithWindows: boolean;
  minimizeToTray: boolean;
  showNotifications: boolean;
  alwaysOnTop: boolean;
  language: "auto" | "pl" | "en";
  acceptedCount: number;
}
