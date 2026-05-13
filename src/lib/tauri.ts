import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { AppState, Settings } from "../types";

export const STATE_EVENT = "app://state";
export const ACCEPTED_EVENT = "app://accepted";

export const api = {
  getState: () => invoke<AppState>("get_state"),
  getSettings: () => invoke<Settings>("get_settings"),
  setAutoAccept: (enabled: boolean) =>
    invoke<AppState>("set_auto_accept", { enabled }),
  updateSettings: (next: Settings) =>
    invoke<Settings>("update_settings", { next }),
  resetStats: () => invoke<AppState>("reset_stats"),
  showWindow: () => invoke<void>("show_window"),
  hideWindow: () => invoke<void>("hide_window"),
  quitApp: () => invoke<void>("quit_app"),
  systemLocale: () => invoke<string | null>("system_locale"),
};

export function onState(cb: (s: AppState) => void): Promise<UnlistenFn> {
  return listen<AppState>(STATE_EVENT, (event) => cb(event.payload));
}

export function onAccepted(cb: (count: number) => void): Promise<UnlistenFn> {
  return listen<number>(ACCEPTED_EVENT, (event) => cb(event.payload));
}
