import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Swords, X } from "lucide-react";
import { api } from "../lib/tauri";

export function TitleBar() {
  const handleMinimize = () => void getCurrentWindow().minimize();
  const handleClose = () => void api.hideWindow();

  return (
    <div
      data-tauri-drag-region
      className="flex h-9 select-none items-center justify-between border-b border-white/[0.04] px-3"
    >
      <div
        data-tauri-drag-region
        className="flex items-center gap-2 text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-400"
      >
        <Swords size={14} className="text-accent-cyan" strokeWidth={2.4} />
        <span data-tauri-drag-region>RIOT ACCEPT</span>
      </div>
      <div className="flex items-center gap-1">
        <button
          type="button"
          aria-label="Minimize"
          className="titlebar-button"
          onClick={handleMinimize}
        >
          <Minus size={14} strokeWidth={2.4} />
        </button>
        <button
          type="button"
          aria-label="Close"
          className="titlebar-button titlebar-button-close"
          onClick={handleClose}
        >
          <X size={14} strokeWidth={2.4} />
        </button>
      </div>
    </div>
  );
}
