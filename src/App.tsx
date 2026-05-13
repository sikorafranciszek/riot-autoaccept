import gsap from "gsap";
import { useEffect, useRef, useState } from "react";
import { AcceptCounter } from "./components/AcceptCounter";
import { ActionRow } from "./components/ActionRow";
import { AutoToggle } from "./components/AutoToggle";
import { PhaseIndicator } from "./components/PhaseIndicator";
import { QueueTimer } from "./components/QueueTimer";
import { SettingsPanel } from "./components/SettingsPanel";
import { StatusOrb } from "./components/StatusOrb";
import { TitleBar } from "./components/TitleBar";
import { useAppState } from "./hooks/useAppState";
import { useSettings } from "./hooks/useSettings";
import { api } from "./lib/tauri";

const APP_VERSION = "1.0.0";

export default function App() {
  const state = useAppState();
  const { settings, update, loaded } = useSettings();
  const [settingsOpen, setSettingsOpen] = useState(false);
  const stageRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!loaded || !stageRef.current) return;
    const cards = stageRef.current.querySelectorAll<HTMLElement>("[data-stage]");
    gsap.fromTo(
      cards,
      { opacity: 0, y: 8 },
      { opacity: 1, y: 0, duration: 0.4, stagger: 0.05, ease: "power2.out" },
    );
  }, [loaded]);

  const handleAutoToggle = (next: boolean) => {
    void api.setAutoAccept(next);
  };

  const inQueue = state.matchPhase === "in-queue" || state.matchPhase === "ready-check";

  return (
    <div className="relative flex h-screen flex-col overflow-hidden">
      <TitleBar />

      <main
        ref={stageRef}
        className="flex flex-1 flex-col items-stretch gap-3 px-4 pb-4 pt-3"
      >
        <div data-stage className="flex justify-center pt-2">
          <StatusOrb state={state} />
        </div>

        <div data-stage className="flex flex-col gap-1.5">
          <QueueTimer seconds={state.queueDurationSec} active={inQueue} />
          <PhaseIndicator phase={state.matchPhase} queueType={state.queueType} />
        </div>

        <div data-stage className="mt-1">
          <AutoToggle
            enabled={state.autoAcceptEnabled}
            onChange={handleAutoToggle}
          />
        </div>

        <div data-stage>
          <AcceptCounter count={state.acceptedCount} />
        </div>

        <div data-stage className="mt-auto">
          <ActionRow
            settings={settings}
            onUpdate={(next) => void update(next)}
            onOpenSettings={() => setSettingsOpen(true)}
          />
        </div>
      </main>

      <SettingsPanel
        open={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        settings={settings}
        onUpdate={(next) => void update(next)}
        acceptedCount={state.acceptedCount}
        version={APP_VERSION}
      />
    </div>
  );
}
