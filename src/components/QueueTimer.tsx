import gsap from "gsap";
import { useEffect, useRef } from "react";
import { cn } from "../lib/cn";

interface Props {
  seconds: number;
  active: boolean;
}

function format(sec: number) {
  const s = Math.max(0, Math.floor(sec));
  const m = Math.floor(s / 60).toString().padStart(2, "0");
  const r = (s % 60).toString().padStart(2, "0");
  return `${m}:${r}`;
}

export function QueueTimer({ seconds, active }: Props) {
  const wrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!wrapRef.current) return;
    gsap.fromTo(
      wrapRef.current,
      { opacity: 0.3, y: 4 },
      { opacity: active ? 1 : 0.3, y: 0, duration: 0.4, ease: "power2.out" },
    );
  }, [active]);

  return (
    <div
      ref={wrapRef}
      className={cn(
        "tabular text-center font-mono text-3xl font-light tracking-tight transition-colors",
        active ? "text-zinc-50" : "text-zinc-700",
      )}
    >
      {format(seconds)}
    </div>
  );
}
