import gsap from "gsap";
import { RotateCcw, X } from "lucide-react";
import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { api } from "../lib/tauri";
import { cn } from "../lib/cn";
import type { Settings } from "../types";

interface Props {
  open: boolean;
  onClose: () => void;
  settings: Settings;
  onUpdate: (next: Partial<Settings>) => void;
  acceptedCount: number;
  version: string;
}

export function SettingsPanel({
  open,
  onClose,
  settings,
  onUpdate,
  acceptedCount,
  version,
}: Props) {
  const panelRef = useRef<HTMLDivElement>(null);
  const { t } = useTranslation();

  useEffect(() => {
    if (!panelRef.current) return;
    if (open) {
      gsap.fromTo(
        panelRef.current,
        { y: "100%", opacity: 0 },
        { y: "0%", opacity: 1, duration: 0.35, ease: "power3.out" },
      );
    }
  }, [open]);

  if (!open) return null;

  return (
    <div className="absolute inset-0 z-40">
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onClick={onClose}
      />
      <div
        ref={panelRef}
        className="absolute inset-x-2 bottom-2 top-12 overflow-hidden rounded-2xl border border-white/[0.08] bg-bg-surface/95 shadow-2xl backdrop-blur-xl"
      >
        <div className="flex items-center justify-between border-b border-white/[0.04] px-4 py-3">
          <span className="text-[11px] font-medium uppercase tracking-[0.22em] text-zinc-300">
            {t("settings.title")}
          </span>
          <button
            type="button"
            onClick={onClose}
            className="titlebar-button"
            aria-label={t("settings.close")}
          >
            <X size={14} strokeWidth={2.4} />
          </button>
        </div>

        <div className="space-y-1 px-2 py-3">
          <Row
            label={t("settings.autostart")}
            checked={settings.startWithWindows}
            onChange={(v) => onUpdate({ startWithWindows: v })}
          />
          <Row
            label={t("settings.minimizeToTray")}
            checked={settings.minimizeToTray}
            onChange={(v) => onUpdate({ minimizeToTray: v })}
          />
          <Row
            label={t("settings.notifications")}
            checked={settings.showNotifications}
            onChange={(v) => onUpdate({ showNotifications: v })}
          />
          <Row
            label={t("settings.alwaysOnTop")}
            checked={settings.alwaysOnTop}
            onChange={(v) => onUpdate({ alwaysOnTop: v })}
          />

          <div className="px-3 py-3">
            <div className="mb-2 text-[10px] uppercase tracking-[0.22em] text-zinc-500">
              {t("settings.language")}
            </div>
            <div className="flex gap-1.5">
              {(["auto", "en", "pl"] as const).map((lang) => (
                <button
                  key={lang}
                  type="button"
                  onClick={() => onUpdate({ language: lang })}
                  className={cn(
                    "flex-1 rounded-md border px-2 py-1.5 text-[10px] font-semibold uppercase tracking-wider transition-colors",
                    settings.language === lang
                      ? "border-accent-cyan/50 bg-accent-cyan/[0.08] text-accent-cyan"
                      : "border-white/[0.06] bg-white/[0.02] text-zinc-400 hover:border-white/[0.12]",
                  )}
                >
                  {lang === "auto" ? t("settings.languageAuto") : lang}
                </button>
              ))}
            </div>
          </div>

          <button
            type="button"
            onClick={() => void api.resetStats()}
            className="mt-2 flex w-full items-center justify-center gap-2 rounded-md border border-rose-500/20 bg-rose-500/[0.04] px-3 py-2 text-[11px] uppercase tracking-wider text-rose-300 transition-colors hover:bg-rose-500/[0.08]"
          >
            <RotateCcw size={13} strokeWidth={2.2} />
            {t("settings.resetStats")} · {acceptedCount}
          </button>
        </div>

        <div className="absolute inset-x-0 bottom-0 flex items-center justify-between border-t border-white/[0.04] px-4 py-2 text-[10px] uppercase tracking-[0.18em] text-zinc-600">
          <span>{t("settings.version")}</span>
          <span>v{version}</span>
        </div>
      </div>
    </div>
  );
}

interface RowProps {
  label: string;
  checked: boolean;
  onChange: (next: boolean) => void;
}

function Row({ label, checked, onChange }: RowProps) {
  return (
    <button
      type="button"
      onClick={() => onChange(!checked)}
      className="flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-xs text-zinc-300 transition-colors hover:bg-white/[0.03]"
    >
      <span>{label}</span>
      <div
        className={cn(
          "relative h-4.5 w-8 rounded-full border transition-colors",
          checked
            ? "border-accent-cyan/40 bg-accent-cyan/20"
            : "border-zinc-700/80 bg-zinc-900",
        )}
        style={{ height: "1.125rem" }}
      >
        <div
          className={cn(
            "absolute top-0.5 h-3 w-3 rounded-full transition-all",
            checked
              ? "left-[1.125rem] bg-accent-cyan shadow-[0_0_8px_rgba(34,211,238,0.6)]"
              : "left-0.5 bg-zinc-500",
          )}
        />
      </div>
    </button>
  );
}
