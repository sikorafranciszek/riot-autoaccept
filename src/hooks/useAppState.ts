import { useEffect, useState } from "react";
import { api, onState } from "../lib/tauri";
import type { AppState } from "../types";

const initial: AppState = {
  running: false,
  autoAcceptEnabled: true,
  connectionStatus: "stopped",
  matchPhase: "idle",
  queueType: "unknown",
  clientPort: null,
  clientSource: null,
  queueDurationSec: 0,
  acceptedCount: 0,
  lastError: null,
  rawPhase: "None",
  updatedAt: Date.now(),
};

export function useAppState() {
  const [state, setState] = useState<AppState>(initial);

  useEffect(() => {
    let unlisten: (() => void) | null = null;
    let cancelled = false;

    api.getState().then((s) => {
      if (!cancelled) setState(s);
    });

    onState((s) => {
      if (!cancelled) setState(s);
    }).then((un) => {
      if (cancelled) un();
      else unlisten = un;
    });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  return state;
}
