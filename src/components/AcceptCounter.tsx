import gsap from "gsap";
import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";

interface Props {
  count: number;
}

export function AcceptCounter({ count }: Props) {
  const { t } = useTranslation();
  const numRef = useRef<HTMLSpanElement>(null);
  const cardRef = useRef<HTMLDivElement>(null);
  const displayed = useRef(0);

  useEffect(() => {
    if (!numRef.current) return;
    const from = displayed.current;
    const obj = { v: from };
    gsap.to(obj, {
      v: count,
      duration: 0.8,
      ease: "power2.out",
      onUpdate: () => {
        if (numRef.current) numRef.current.textContent = Math.round(obj.v).toString();
      },
      onComplete: () => {
        displayed.current = count;
      },
    });
    if (count > from && cardRef.current) {
      gsap.fromTo(
        cardRef.current,
        { boxShadow: "0 0 0 rgba(16,185,129,0)" },
        {
          boxShadow: "0 0 28px rgba(16,185,129,0.45)",
          duration: 0.3,
          yoyo: true,
          repeat: 1,
          ease: "power2.out",
        },
      );
    }
  }, [count]);

  return (
    <div
      ref={cardRef}
      className="card flex items-center justify-center gap-3 px-4 py-3"
    >
      <span
        ref={numRef}
        className="tabular font-mono text-2xl font-light tracking-tight text-zinc-100"
      >
        0
      </span>
      <span className="text-[10px] uppercase tracking-[0.22em] text-zinc-500">
        {t("stats.accepted")}
      </span>
    </div>
  );
}
