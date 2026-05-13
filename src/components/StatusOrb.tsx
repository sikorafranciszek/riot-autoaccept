import gsap from "gsap";
import { useEffect, useMemo, useRef } from "react";
import { useTranslation } from "react-i18next";
import { cn } from "../lib/cn";
import type { AppState } from "../types";

interface Props {
  state: AppState;
}

type Mode = "off" | "searching" | "connected" | "error";

function getMode(s: AppState): Mode {
  if (!s.autoAcceptEnabled) return "off";
  switch (s.connectionStatus) {
    case "connected":
      return "connected";
    case "error":
    case "disconnected":
      return "error";
    case "connecting":
    case "searching":
    case "stopped":
    default:
      return "searching";
  }
}

const modeMeta: Record<
  Mode,
  { color: string; glow: string; ring: string; label: keyof IntlMap }
> = {
  off: {
    color: "bg-zinc-500",
    glow: "shadow-zinc-500/0",
    ring: "ring-zinc-700",
    label: "stopped",
  },
  searching: {
    color: "bg-accent-amber",
    glow: "shadow-amber-500/40",
    ring: "ring-amber-500/40",
    label: "searching",
  },
  connected: {
    color: "bg-accent-emerald",
    glow: "shadow-emerald-500/40",
    ring: "ring-emerald-500/40",
    label: "connected",
  },
  error: {
    color: "bg-accent-rose",
    glow: "shadow-rose-500/40",
    ring: "ring-rose-500/40",
    label: "error",
  },
};

type IntlMap = { stopped: string; searching: string; connected: string; error: string };

export function StatusOrb({ state }: Props) {
  const mode = useMemo(() => getMode(state), [state]);
  const { t } = useTranslation();
  const dotRef = useRef<HTMLDivElement>(null);
  const pulseRef = useRef<HTMLDivElement>(null);
  const tl = useRef<gsap.core.Timeline | null>(null);

  useEffect(() => {
    tl.current?.kill();
    if (!dotRef.current || !pulseRef.current) return;
    gsap.set(pulseRef.current, { scale: 1, opacity: 0.6 });

    if (mode === "connected" || mode === "searching") {
      tl.current = gsap
        .timeline({ repeat: -1 })
        .to(pulseRef.current, {
          scale: 2.2,
          opacity: 0,
          duration: 1.4,
          ease: "power2.out",
        });
    } else if (mode === "error") {
      tl.current = gsap.timeline({ repeat: -1, repeatDelay: 1.4 }).to(dotRef.current, {
        x: -2,
        duration: 0.05,
        yoyo: true,
        repeat: 5,
        ease: "none",
      });
    }
    return () => {
      tl.current?.kill();
    };
  }, [mode]);

  const meta = modeMeta[mode];
  const labelKey = (() => {
    if (mode === "off") return "status.stopped";
    if (mode === "connected") {
      if (state.matchPhase === "ready-check") return "phase.ready-check";
      if (state.matchPhase === "in-queue") return "phase.in-queue";
      if (state.matchPhase === "in-champ-select") return "phase.in-champ-select";
      if (state.matchPhase === "in-game") return "phase.in-game";
      return "status.connected";
    }
    if (mode === "error") return "status.disconnected";
    return "status.searching";
  })();

  return (
    <div className="flex flex-col items-center gap-3">
      <div className="relative h-16 w-16">
        <div
          ref={pulseRef}
          className={cn(
            "absolute inset-0 rounded-full ring-1 ring-inset",
            meta.ring,
          )}
        />
        <div
          className={cn(
            "absolute inset-3 rounded-full ring-1 ring-inset",
            meta.ring,
          )}
        />
        <div className="absolute inset-0 flex items-center justify-center">
          <div
            ref={dotRef}
            className={cn(
              "h-2.5 w-2.5 rounded-full shadow-[0_0_20px_var(--tw-shadow-color)]",
              meta.color,
              meta.glow,
            )}
          />
        </div>
      </div>
      <div className="text-center">
        <div className="text-[10px] uppercase tracking-[0.22em] text-zinc-500">
          {t("auto.label")}
        </div>
        <div className="mt-0.5 text-sm font-medium tracking-tight text-zinc-100">
          {t(labelKey)}
        </div>
      </div>
    </div>
  );
}
