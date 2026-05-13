import gsap from "gsap";
import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { cn } from "../lib/cn";

interface Props {
  enabled: boolean;
  onChange: (next: boolean) => void;
}

export function AutoToggle({ enabled, onChange }: Props) {
  const { t } = useTranslation();
  const knobRef = useRef<HTMLDivElement>(null);
  const trackRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!knobRef.current || !trackRef.current) return;
    gsap.to(knobRef.current, {
      x: enabled ? 20 : 0,
      duration: 0.4,
      ease: "elastic.out(1, 0.6)",
    });
  }, [enabled]);

  return (
    <button
      ref={trackRef}
      type="button"
      onClick={() => onChange(!enabled)}
      className={cn(
        "card card-hover group flex w-full items-center justify-between px-4 py-3",
        enabled && "border-emerald-500/30 bg-emerald-500/[0.04]",
      )}
      aria-pressed={enabled}
    >
      <div className="flex items-center gap-3">
        <span
          className={cn(
            "text-[10px] font-medium uppercase tracking-[0.22em]",
            enabled ? "text-emerald-300" : "text-zinc-500",
          )}
        >
          {t("auto.label")}
        </span>
        <span
          className={cn(
            "rounded-md px-1.5 py-0.5 text-[10px] font-semibold tracking-wider",
            enabled
              ? "bg-emerald-500/15 text-emerald-300"
              : "bg-zinc-800 text-zinc-500",
          )}
        >
          {enabled ? t("auto.on") : t("auto.off")}
        </span>
      </div>

      <div
        className={cn(
          "relative h-6 w-11 rounded-full border transition-colors",
          enabled
            ? "border-emerald-500/40 bg-emerald-500/20"
            : "border-zinc-700/80 bg-zinc-900",
        )}
      >
        <div
          ref={knobRef}
          className={cn(
            "absolute left-0.5 top-0.5 h-5 w-5 rounded-full transition-shadow",
            enabled
              ? "bg-emerald-300 shadow-[0_0_12px_rgba(16,185,129,0.6)]"
              : "bg-zinc-500",
          )}
        />
      </div>
    </button>
  );
}
