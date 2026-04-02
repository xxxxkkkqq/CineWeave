export const STYLE_LIBRARY = Object.freeze({
  cinematic_grade: [
    "eq=contrast=1.08:saturation=1.12:brightness=0.01",
    "colorbalance=rs=0.02:bs=-0.01",
    "vignette=PI/6",
  ],
  retro_film: [
    "curves=vintage",
    "noise=alls=10:allf=t",
    "eq=saturation=0.92:contrast=1.03",
  ],
  clean_bright: [
    "eq=brightness=0.03:saturation=1.04",
    "unsharp=3:3:0.35",
  ],
  teal_orange: [
    "colorbalance=rs=0.05:gs=0.01:bs=-0.04",
    "eq=saturation=1.07:contrast=1.04",
  ],
});

export const EFFECT_LIBRARY = Object.freeze({
  zoom_punch: {
    ffmpeg: "zoompan=z='min(zoom+0.0015,1.2)':d=1:s=1080x1920",
    experimental: false,
  },
  speed_ramp: {
    ffmpeg: "setpts=0.92*PTS",
    experimental: false,
  },
  soft_flash: {
    ffmpeg: "eq=brightness=0.06:contrast=1.03",
    experimental: false,
  },
  subtitle_pop: {
    ffmpeg: null,
    experimental: true,
  },
  glitch: {
    ffmpeg: null,
    experimental: true,
  },
});
