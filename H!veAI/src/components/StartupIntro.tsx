import { useEffect, useRef, useState } from "react";
import openingVideo from "../assets/opening-video.mp4";
import { isTauriDesktop } from "../projectRegistry";

const INTRO_SESSION_KEY = "hiveai.startup-intro.played";
const INTRO_FAILSAFE_MS = 15_000;
const INTRO_FADE_MS = 280;

function hasPlayedInThisWindow() {
  try {
    return window.sessionStorage.getItem(INTRO_SESSION_KEY) === "1";
  } catch {
    return false;
  }
}

function markPlayedInThisWindow() {
  try {
    window.sessionStorage.setItem(INTRO_SESSION_KEY, "1");
  } catch {
    // A memory-backed session still guarantees route-stable play-once behavior.
  }
}

export function StartupIntro() {
  const [visible, setVisible] = useState(() => {
    if (!isTauriDesktop() || hasPlayedInThisWindow()) return false;
    markPlayedInThisWindow();
    return true;
  });
  const [closing, setClosing] = useState(false);
  const videoRef = useRef<HTMLVideoElement>(null);
  const closeTimer = useRef<number | undefined>(undefined);

  const dismiss = () => {
    setClosing(true);
    if (closeTimer.current === undefined) {
      closeTimer.current = window.setTimeout(() => setVisible(false), INTRO_FADE_MS);
    }
  };

  useEffect(() => {
    if (!visible) return undefined;
    const failsafe = window.setTimeout(dismiss, INTRO_FAILSAFE_MS);
    const video = videoRef.current;
    if (video) {
      const playback = video.play();
      playback?.catch(dismiss);
    }
    return () => {
      window.clearTimeout(failsafe);
      if (closeTimer.current !== undefined) {
        window.clearTimeout(closeTimer.current);
        closeTimer.current = undefined;
      }
    };
  }, [visible]);

  if (!visible) return null;

  return (
    <div
      className={`startup-intro${closing ? " startup-intro-closing" : ""}`}
      role="dialog"
      aria-label="H!veAI startup"
      aria-live="polite"
    >
      <video
        ref={videoRef}
        src={openingVideo}
        autoPlay
        muted
        playsInline
        preload="auto"
        controls={false}
        onEnded={dismiss}
        onError={dismiss}
      />
    </div>
  );
}
