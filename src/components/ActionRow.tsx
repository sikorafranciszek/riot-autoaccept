import { Bell, BellOff, Pin, PinOff, Settings as SettingsIcon } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cn } from "../lib/cn";
import type { Settings } from "../types";

interface Props {
  settings: Settings;
  onUpdate: (next: Partial<Settings>) => void;
  onOpenSettings: () => void;
}

export function ActionRow({ settings, onUpdate, onOpenSettings }: Props) {
  const { i18n, t } = useTranslation();
  const currentLang = i18n.resolvedLanguage ?? "en";

  const toggleLang = () => {
    const next = currentLang === "pl" ? "en" : "pl";
    onUpdate({ language: next });
  };

  return (
    <div className="flex items-center justify-center gap-2 pt-1">
      <IconButton
        active={false}
        ariaLabel={t("actions.settings")}
        onClick={onOpenSettings}
      >
        <SettingsIcon size={15} strokeWidth={2.2} />
      </IconButton>

      <IconButton
        active={settings.showNotifications}
        ariaLabel={t("actions.notifications")}
        onClick={() => onUpdate({ showNotifications: !settings.showNotifications })}
      >
        {settings.showNotifications ? (
          <Bell size={15} strokeWidth={2.2} />
        ) : (
          <BellOff size={15} strokeWidth={2.2} />
        )}
      </IconButton>

      <IconButton
        active={settings.alwaysOnTop}
        ariaLabel={t("actions.alwaysOnTop")}
        onClick={() => onUpdate({ alwaysOnTop: !settings.alwaysOnTop })}
      >
        {settings.alwaysOnTop ? (
          <Pin size={15} strokeWidth={2.2} />
        ) : (
          <PinOff size={15} strokeWidth={2.2} />
        )}
      </IconButton>

      <button
        type="button"
        onClick={toggleLang}
        className="flex h-8 items-center rounded-md border border-white/[0.06] bg-white/[0.02] px-2 text-[10px] font-semibold uppercase tracking-wider text-zinc-400 transition-colors hover:border-accent-cyan/40 hover:bg-white/[0.06] hover:text-accent-cyan"
        aria-label={t("actions.language")}
      >
        {currentLang.toUpperCase()}
      </button>
    </div>
  );
}

interface IconButtonProps {
  active: boolean;
  ariaLabel: string;
  onClick: () => void;
  children: React.ReactNode;
}

function IconButton({ active, ariaLabel, onClick, children }: IconButtonProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      aria-label={ariaLabel}
      aria-pressed={active}
      className={cn(
        "flex h-8 w-8 items-center justify-center rounded-md border transition-all",
        active
          ? "border-accent-cyan/40 bg-accent-cyan/[0.08] text-accent-cyan shadow-[0_0_12px_rgba(34,211,238,0.25)]"
          : "border-white/[0.06] bg-white/[0.02] text-zinc-500 hover:border-white/[0.12] hover:text-zinc-200",
      )}
    >
      {children}
    </button>
  );
}
