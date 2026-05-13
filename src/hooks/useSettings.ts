import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { api } from "../lib/tauri";
import type { Settings } from "../types";

const initial: Settings = {
  autoAcceptEnabled: true,
  startWithWindows: false,
  minimizeToTray: true,
  showNotifications: true,
  alwaysOnTop: false,
  language: "auto",
  acceptedCount: 0,
};

export function useSettings() {
  const [settings, setSettings] = useState<Settings>(initial);
  const [loaded, setLoaded] = useState(false);
  const { i18n } = useTranslation();

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const s = await api.getSettings();
      if (cancelled) return;
      setSettings(s);

      // Resolve language at startup.
      let lang = s.language;
      if (lang === "auto") {
        const locale = await api.systemLocale().catch(() => null);
        lang = locale && locale.toLowerCase().startsWith("pl") ? "pl" : "en";
      }
      void i18n.changeLanguage(lang);
      setLoaded(true);
    })();
    return () => {
      cancelled = true;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const update = async (next: Partial<Settings>) => {
    const merged = { ...settings, ...next };
    setSettings(merged);
    const saved = await api.updateSettings(merged);
    setSettings(saved);

    if (next.language) {
      let lang = next.language;
      if (lang === "auto") {
        const locale = await api.systemLocale().catch(() => null);
        lang = locale && locale.toLowerCase().startsWith("pl") ? "pl" : "en";
      }
      void i18n.changeLanguage(lang);
    }
    return saved;
  };

  return { settings, update, loaded };
}
