import gsap from "gsap";
import {
  Crown,
  Gamepad2,
  LucideIcon,
  Moon,
  Swords,
  Trophy,
  Users,
} from "lucide-react";
import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { cn } from "../lib/cn";
import type { MatchPhase, QueueType } from "../types";

interface Props {
  phase: MatchPhase;
  queueType: QueueType;
}

const phaseIcon: Record<MatchPhase, LucideIcon> = {
  idle: Moon,
  "in-queue": Swords,
  "ready-check": Crown,
  "in-champ-select": Users,
  "in-game": Gamepad2,
  "end-of-game": Trophy,
};

export function PhaseIndicator({ phase, queueType }: Props) {
  const { t } = useTranslation();
  const wrapRef = useRef<HTMLDivElement>(null);
  const prevPhase = useRef<MatchPhase>(phase);

  useEffect(() => {
    if (prevPhase.current === phase) return;
    prevPhase.current = phase;
    if (!wrapRef.current) return;
    gsap.fromTo(
      wrapRef.current,
      { rotationY: -90, opacity: 0 },
      { rotationY: 0, opacity: 1, duration: 0.5, ease: "power3.out" },
    );
  }, [phase]);

  const Icon = phaseIcon[phase] ?? Moon;
  const queueLabel = queueType !== "unknown" ? t(`queue.${queueType}`) : null;
  const phaseLabel = t(`phase.${phase}`);

  const accent =
    phase === "ready-check"
      ? "text-accent-emerald"
      : phase === "in-queue"
        ? "text-accent-cyan"
        : phase === "in-game" || phase === "in-champ-select"
          ? "text-accent-violet"
          : phase === "end-of-game"
            ? "text-accent-amber"
            : "text-zinc-600";

  return (
    <div
      ref={wrapRef}
      className="flex items-center justify-center gap-2 text-[11px] uppercase tracking-[0.18em] text-zinc-400"
      style={{ perspective: "400px" }}
    >
      <Icon size={14} className={cn(accent)} strokeWidth={2.2} />
      <span>{phaseLabel}</span>
      {queueLabel ? (
        <>
          <span className="text-zinc-700">·</span>
          <span className="text-zinc-300">{queueLabel}</span>
        </>
      ) : null}
    </div>
  );
}
