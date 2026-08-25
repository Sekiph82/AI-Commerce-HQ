import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import openingVideo from "../assets/opening-video.mp4";
import { isTauriDesktop } from "../projectRegistry";

const INTRO_FAILSAFE_MS = 15_000;
const INTRO_FADE_MS = 280;

export function StartupIntro() {
  const native = isTauriDesktop();
  const [claim, setClaim] = useState<"pending" | "play" | "skip">(
    native ? "pending" : "skip",
  );
  const [visible, setVisible] = useState(native);
  const [closing, setClosing] = useState(false);
  const videoRef = useRef<HTMLVideoElement>(null);
  const closeTimer = useRef<number | undefined>(undefined);

  useEffect(() => {
    if (!native) return undefined;
    let active = true;
    void invoke<boolean>("hiveai_startup_intro_claim")
      .then((claimed) => {
        if (!active) return;
        setClaim(claimed ? "play" : "skip");
        if (!claimed) setVisible(false);
      })
      .catch(() => {
        if (!active) return;
        setClaim("skip");
        setVisible(false);
      });
    return () => {
      active = false;
    };
  }, [native]);

  const dismiss = () => {
    setClosing(true);
    if (closeTimer.current === undefined) {
      closeTimer.current = window.setTimeout(() => setVisible(false), INTRO_FADE_MS);
    }
  };

  useEffect(() => {
    if (!visible || claim !== "play") return undefined;
    const failsafe = window.setTimeout(dismiss, INTRO_FAILSAFE_MS);
    const video = videoRef.current;
    if (video) {
      video.muted = false;
      video.volume = 1;
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
  }, [claim, visible]);

  if (!visible) return null;

  return (
    <div
      className={`startup-intro${closing ? " startup-intro-closing" : ""}`}
      role="dialog"
      aria-label="H!veAI startup"
      aria-live="polite"
    >
      {claim === "play" ? (
        <video
          ref={videoRef}
          aria-label="H!veAI opening video"
          src={openingVideo}
          autoPlay
          playsInline
          preload="auto"
          controls={false}
          onEnded={dismiss}
          onError={dismiss}
        />
      ) : null}
    </div>
  );
}
