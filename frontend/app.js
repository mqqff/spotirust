// Authentic Spotify Encore 16x16 / 24x24 SVG icons
const ICONS = {
  spotify: (s = 18) => `<svg width="${s}" height="${s}" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm4.58 14.42a.75.75 0 0 1-1.03.25c-2.82-1.72-6.37-2.11-10.55-1.16a.75.75 0 1 1-.33-1.46c4.58-1.05 8.51-.6 11.66 1.33.35.22.46.68.25 1.04zm1.22-2.72a.94.94 0 0 1-1.29.31c-3.23-1.98-8.15-2.56-11.97-1.4a.94.94 0 0 1-.55-1.8c4.36-1.32 9.78-.68 13.5 1.6.44.27.58.85.31 1.29zm.11-2.83C14.04 8.57 7.64 8.36 3.94 9.48a1.12 1.12 0 1 1-.65-2.15c4.25-1.29 11.31-1.04 15.77 1.6a1.12 1.12 0 0 1-1.15 1.94z"/></svg>`,
  home: (s = 22) => `<svg width="${s}" height="${s}" viewBox="0 0 24 24" fill="currentColor"><path d="M13.5 1.515a3 3 0 0 0-3 0L3 5.845a2 2 0 0 0-1 1.732V21a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1v-6h4v6a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V7.577a2 2 0 0 0-1-1.732l-7.5-4.33z"/></svg>`,
  homeOutline: (s = 22) => `<svg width="${s}" height="${s}" viewBox="0 0 24 24" fill="currentColor"><path d="M12.5 3.247a1 1 0 0 0-1 0L4 7.577V20h4.5v-6a1 1 0 0 1 1-1h5a1 1 0 0 1 1 1v6H20V7.577l-7.5-4.33zm-2-1.732a3 3 0 0 1 3 0l7.5 4.33a2 2 0 0 1 1 1.732V21a1 1 0 0 1-1 1h-6.5a1 1 0 0 1-1-1v-6h-3v6a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V7.577a2 2 0 0 1 1-1.732l7.5-4.33z"/></svg>`,
  search: (s = 18) => `<svg width="${s}" height="${s}" viewBox="0 0 24 24" fill="currentColor"><path d="M10.533 1.279C5.352 1.279 1.126 5.419 1.126 10.558s4.226 9.279 9.407 9.279c2.234 0 4.29-.77 5.907-2.058l4.353 4.353a1 1 0 1 0 1.414-1.414l-4.344-4.344a9.157 9.157 0 0 0 2.077-5.816c0-5.14-4.226-9.28-9.407-9.28zm-7.407 9.279c0-4.006 3.302-7.28 7.407-7.28s7.407 3.274 7.407 7.28-3.302 7.279-7.407 7.279-7.407-3.273-7.407-7.28z"/></svg>`,
  browse: (s = 19) => `<svg width="${s}" height="${s}" viewBox="0 0 24 24" fill="currentColor"><path d="M15 15.5c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z"/><path d="M1.513 9.37A1 1 0 0 1 2.291 9h19.418a1 1 0 0 1 .979 1.208l-2.339 11a1 1 0 0 1-.978.792H4.63a1 1 0 0 1-.978-.792l-2.339-11a1 1 0 0 1 .201-.837zM3.525 11l1.913 9h13.123l1.913-9H3.525zM4 2a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v4h-2V3H6v3H4V2z"/></svg>`,
  bell: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1.5a4 4 0 0 0-4 4v3.27a.75.75 0 0 1-.1.373L2.255 12h11.49L12.1 9.142a.75.75 0 0 1-.1-.374V5.5a4 4 0 0 0-4-4zm-5.5 4a5.5 5.5 0 0 1 11 0v3.067l2.193 3.809a.75.75 0 0 1-.65 1.124H10.5a2.5 2.5 0 0 1-5 0H.957a.75.75 0 0 1-.65-1.124L2.5 8.569V5.5zm4.5 8a1 1 0 1 0 2 0H7z"/></svg>`,
  users: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M5.5 3.5a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM2 5.5a3.5 3.5 0 1 1 5.898 2.549 5.507 5.507 0 0 1 3.034 4.084.75.75 0 1 1-1.482.235 4.001 4.001 0 0 0-7.9 0 .75.75 0 0 1-1.482-.236A5.507 5.507 0 0 1 3.102 8.05 3.49 3.49 0 0 1 2 5.5zM11 4a.75.75 0 1 0 0 1.5 1.5 1.5 0 0 1 .666 2.844.75.75 0 0 0-.416.672v.352a.75.75 0 0 0 .574.73c1.2.289 2.162 1.2 2.522 2.372a.75.75 0 1 0 1.434-.44 5.01 5.01 0 0 0-2.56-3.012A3 3 0 0 0 11 4z"/></svg>`,
  chevronLeft: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M11.03.47a.75.75 0 0 1 0 1.06L4.56 8l6.47 6.47a.75.75 0 1 1-1.06 1.06L2.44 8 9.97.47a.75.75 0 0 1 1.06 0z"/></svg>`,
  chevronRight: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M4.97.47a.75.75 0 0 0 0 1.06L11.44 8l-6.47 6.47a.75.75 0 1 0 1.06 1.06L13.56 8 6.03.47a.75.75 0 0 0-1.06 0z"/></svg>`,
  plus: (s = 14) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M15.25 8a.75.75 0 0 1-.75.75H8.75v5.75a.75.75 0 0 1-1.5 0V8.75H1.5a.75.75 0 0 1 0-1.5h5.75V1.5a.75.75 0 0 1 1.5 0v5.75h5.75a.75.75 0 0 1 .75.75z"/></svg>`,
  addCircle: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13zM0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8z"/><path d="M11.75 8a.75.75 0 0 1-.75.75H8.75V11a.75.75 0 0 1-1.5 0V8.75H5a.75.75 0 0 1 0-1.5h2.25V5a.75.75 0 0 1 1.5 0v2.25H11a.75.75 0 0 1 .75.75z"/></svg>`,
  checkCircle: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm11.748-1.97a.75.75 0 0 0-1.06-1.06l-4.47 4.47-1.405-1.406a.75.75 0 1 0-1.061 1.06l2.466 2.467 5.53-5.53z"/></svg>`,
  heart: (s = 22) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M15.724 4.22A4.313 4.313 0 0 0 12.192.814a4.269 4.269 0 0 0-3.622 1.13.837.837 0 0 1-1.14 0 4.272 4.272 0 0 0-6.21 5.855l5.916 7.05a1.128 1.128 0 0 0 1.727 0l5.916-7.05a4.228 4.228 0 0 0 .945-3.577z"/></svg>`,
  play: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M3 1.713a.7.7 0 0 1 1.05-.607l10.89 6.288a.7.7 0 0 1 0 1.212L4.05 14.894A.7.7 0 0 1 3 14.288V1.713z"/></svg>`,
  pause: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M2.7 1a.7.7 0 0 0-.7.7v12.6a.7.7 0 0 0 .7.7h2.6a.7.7 0 0 0 .7-.7V1.7a.7.7 0 0 0-.7-.7H2.7zm8 0a.7.7 0 0 0-.7.7v12.6a.7.7 0 0 0 .7.7h2.6a.7.7 0 0 0 .7-.7V1.7a.7.7 0 0 0-.7-.7h-2.6z"/></svg>`,
  prev: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M3.3 1a.7.7 0 0 1 .7.7v5.15l9.95-5.744a.7.7 0 0 1 1.05.606v12.575a.7.7 0 0 1-1.05.607L4 9.149V14.3a.7.7 0 0 1-.7.7H1.7a.7.7 0 0 1-.7-.7V1.7a.7.7 0 0 1 .7-.7h1.6z"/></svg>`,
  next: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M12.7 1a.7.7 0 0 0-.7.7v5.15L2.05 1.107A.7.7 0 0 0 1 1.712v12.575a.7.7 0 0 0 1.05.607L12 9.149V14.3a.7.7 0 0 0 .7.7h1.6a.7.7 0 0 0 .7-.7V1.7a.7.7 0 0 0-.7-.7h-1.6z"/></svg>`,
  shuffle: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M13.151.922a.75.75 0 1 0-1.06 1.06L13.109 3H11.16a3.75 3.75 0 0 0-2.873 1.34l-6.173 7.356A2.25 2.25 0 0 1 .39 12.5H0V14h.391a3.75 3.75 0 0 0 2.873-1.34l6.173-7.356a2.25 2.25 0 0 1 1.724-.804h1.947l-1.017 1.018a.75.75 0 0 0 1.06 1.06L15.98 3.75 13.15.922zM.391 3.5H0V2h.391c1.109 0 2.16.49 2.873 1.34L4.89 5.277l-.979 1.167-1.796-2.14A2.25 2.25 0 0 0 .39 3.5z"/><path d="m7.5 10.723.98-1.167.957 1.14a2.25 2.25 0 0 0 1.724.804h1.947l-1.017-1.018a.75.75 0 1 1 1.06-1.06l2.829 2.828-2.829 2.828a.75.75 0 1 1-1.06-1.06L13.109 13H11.16a3.75 3.75 0 0 1-2.873-1.34l-.787-.938z"/></svg>`,
  repeat: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M0 4.75A3.75 3.75 0 0 1 3.75 1h8.5A3.75 3.75 0 0 1 16 4.75v5a3.75 3.75 0 0 1-3.75 3.75H9.81l1.018 1.018a.75.75 0 1 1-1.06 1.06L6.939 12.75l2.829-2.828a.75.75 0 1 1 1.06 1.06L9.811 12h2.439a2.25 2.25 0 0 0 2.25-2.25v-5a2.25 2.25 0 0 0-2.25-2.25h-8.5A2.25 2.25 0 0 0 1.5 4.75v5A2.25 2.25 0 0 0 3.75 12H5v1.5H3.75A3.75 3.75 0 0 1 0 9.75v-5z"/></svg>`,
  repeatOne: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M0 4.75A3.75 3.75 0 0 1 3.75 1h8.5A3.75 3.75 0 0 1 16 4.75v5a3.75 3.75 0 0 1-3.75 3.75H9.81l1.018 1.018a.75.75 0 1 1-1.06 1.06L6.939 12.75l2.829-2.828a.75.75 0 1 1 1.06 1.06L9.811 12h2.439a2.25 2.25 0 0 0 2.25-2.25v-5a2.25 2.25 0 0 0-2.25-2.25h-8.5A2.25 2.25 0 0 0 1.5 4.75v5A2.25 2.25 0 0 0 3.75 12H5v1.5H3.75A3.75 3.75 0 0 1 0 9.75v-5z"/><path d="M9.12 8V4.247h-.922c-.12.404-.415.72-1.153.788v.83h.962V8h1.113z"/></svg>`,
  mic: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M13.426 2.574a2.831 2.831 0 0 0-4.797 1.55l3.247 3.247a2.831 2.831 0 0 0 1.55-4.797zM10.5 8.118l-2.619-2.62L4.74 9.075 2.065 12.12a1.287 1.287 0 0 0 1.816 1.816l3.06-2.688 3.56-3.129zM7.12 4.094a4.331 4.331 0 1 1 4.786 4.786l-3.974 3.493-3.06 2.689a2.787 2.787 0 0 1-3.933-3.933l2.676-3.045 3.505-3.99z"/></svg>`,
  nowPlaying: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M11.196 8 6 5v6l5.196-3z"/><path d="M15.002 1.75A1.75 1.75 0 0 0 13.252 0h-10.5a1.75 1.75 0 0 0-1.75 1.75v12.5c0 .966.783 1.75 1.75 1.75h10.5a1.75 1.75 0 0 0 1.75-1.75V1.75zm-1.75-.25a.25.25 0 0 1 .25.25v12.5a.25.25 0 0 1-.25.25h-10.5a.25.25 0 0 1-.25-.25V1.75a.25.25 0 0 1 .25-.25h10.5z"/></svg>`,
  volHigh: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M9.741.85a.75.75 0 0 1 .375.65v13a.75.75 0 0 1-1.125.65l-6.925-4a3.642 3.642 0 0 1-1.33-4.967 3.639 3.639 0 0 1 1.33-1.332l6.925-4a.75.75 0 0 1 .75 0zm-6.924 5.3a2.139 2.139 0 0 0 0 3.7l5.8 3.35V2.8l-5.8 3.35zm8.683 4.29V5.56a2.75 2.75 0 0 1 0 4.88z"/><path d="M11.5 13.614a5.752 5.752 0 0 0 0-11.228v1.55a4.252 4.252 0 0 1 0 8.127v1.55z"/></svg>`,
  volLow: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M9.741.85a.75.75 0 0 1 .375.65v13a.75.75 0 0 1-1.125.65l-6.925-4a3.642 3.642 0 0 1-1.33-4.967 3.639 3.639 0 0 1 1.33-1.332l6.925-4a.75.75 0 0 1 .75 0zm-6.924 5.3a2.139 2.139 0 0 0 0 3.7l5.8 3.35V2.8l-5.8 3.35zm8.683 4.29V5.56a2.75 2.75 0 0 1 0 4.88z"/></svg>`,
  volMute: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M13.86 5.47a.75.75 0 0 0-1.061 0l-1.47 1.47-1.47-1.47A.75.75 0 0 0 8.8 6.53L10.269 8l-1.47 1.47a.75.75 0 1 0 1.06 1.06l1.47-1.47 1.47 1.47a.75.75 0 0 0 1.06-1.06L12.39 8l1.47-1.47a.75.75 0 0 0 0-1.06z"/><path d="M10.116 1.5A.75.75 0 0 0 8.991.85l-6.925 4a3.642 3.642 0 0 0-1.33 4.967 3.639 3.639 0 0 0 1.33 1.332l6.925 4a.75.75 0 0 0 1.125-.649v-1.906a4.73 4.73 0 0 1-1.5-.694v1.3L2.817 9.852a2.141 2.141 0 0 1-.781-2.92c.187-.324.456-.594.78-.782l5.8-3.35v1.3c.45-.313.956-.55 1.5-.694V1.5z"/></svg>`,
  clock: (s = 15) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13zM0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8z"/><path d="M8 3.25a.75.75 0 0 1 .75.75v3.25H11a.75.75 0 0 1 0 1.5H7.25V4A.75.75 0 0 1 8 3.25z"/></svg>`,
  download: (s = 22) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M4.995 8.745a.75.75 0 0 1 1.06 0L7.25 9.939V4a.75.75 0 0 1 1.5 0v5.94l1.195-1.195a.75.75 0 1 1 1.06 1.06L8 12.811l-.528-.528L4.995 9.805a.75.75 0 0 1 0-1.06z"/><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13z"/></svg>`,
  list: (s = 14) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M15 14.5H5V13h10v1.5zm0-5.75H5v-1.5h10v1.5zM15 3H5V1.5h10V3zM3 3H1V1.5h2V3zm0 11.5H1V13h2v1.5zm0-5.75H1v-1.5h2v1.5z"/></svg>`,
  library: (s = 20) => `<svg width="${s}" height="${s}" viewBox="0 0 24 24" fill="currentColor"><path d="M3 22a1 1 0 0 1-1-1V3a1 1 0 0 1 2 0v18a1 1 0 0 1-1 1zM15.5 2.134A1 1 0 0 0 14 3v18a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V6.464a1 1 0 0 0-.5-.866l-6-3.464zM9 2a1 1 0 0 0-1 1v18a1 1 0 1 0 2 0V3a1 1 0 0 0-1-1z"/></svg>`,
  queue: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M15 15H1v-1.5h14V15zm0-4.5H1V9h14v1.5zm-14-7A2.5 2.5 0 0 1 3.5 1h9a2.5 2.5 0 0 1 0 5h-9A2.5 2.5 0 0 1 1 3.5zm2.5-1a1 1 0 0 0 0 2h9a1 1 0 1 0 0-2h-9z"/></svg>`,
  device: (s = 16) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M6 2.75C6 1.784 6.784 1 7.75 1h6.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0 1 14.25 15h-6.5A1.75 1.75 0 0 1 6 13.25V2.75zm1.75-.25a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h6.5a.25.25 0 0 0 .25-.25V2.75a.25.25 0 0 0-.25-.25h-6.5zm-6 0a.25.25 0 0 0-.25.25v6.5c0 .138.112.25.25.25H4V11H1.75A1.75 1.75 0 0 1 0 9.25v-6.5C0 1.784.784 1 1.75 1H4v1.5H1.75zM4 15H2v-1.5h2V15z"/><path d="M13 10a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm-1-5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"/></svg>`,
  more: (s = 20) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M3 8a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zm6.5 0a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zM16 8a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0z"/></svg>`,
  close: (s = 15) => `<svg width="${s}" height="${s}" viewBox="0 0 16 16" fill="currentColor"><path d="M2.47 2.47a.75.75 0 0 1 1.06 0L8 6.94l4.47-4.47a.75.75 0 1 1 1.06 1.06L9.06 8l4.47 4.47a.75.75 0 1 1-1.06 1.06L8 9.06l-4.47 4.47a.75.75 0 0 1-1.06-1.06L6.94 8 2.47 3.53a.75.75 0 0 1 0-1.06z"/></svg>`,
};

const invoke = (cmd, args = {}) => window.__TAURI__.core.invoke(cmd, args);

// UI Navigation state (kept on frontend while playback/catalog lives in Rust)
const ui = {
  view: { kind: "home" }, // home | search | liked | playlist | lyrics
  back: [],
  forward: [],
  homeFilter: "all",
  libFilter: "all",
  sortMode: "custom", // custom | title | duration
  searchQuery: "",
  playlistSearchOpen: false,
  playlistQuery: "",
  libCollapsed: false,
  rightPanel: "now_playing", // now_playing | queue | null
  lastLyricKey: "",
  lastErrorShown: null,
  loginBusy: false,
  openPopover: null, // menu | bell | friends | avatar | create | sort | devices | ctx | null
};

let snap = null;

function esc(str) {
  return String(str)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function fmtTime(secs) {
  const s = Math.max(0, Math.floor(secs));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

function fmtTotal(ids) {
  const total = ids.reduce((acc, id) => acc + (snap.tracks[id]?.duration_secs || 0), 0);
  const m = Math.floor(total / 60);
  return m >= 60 ? `${Math.floor(m / 60)} hr ${m % 60} min` : `${m} min ${total % 60} sec`;
}

function currentTrack() {
  if (!snap || snap.queue.length === 0) return null;
  return snap.tracks[snap.queue[snap.queue_pos]] || null;
}

function activeLyricIndex(lyrics, pos) {
  let idx = -1;
  for (let i = 0; i < lyrics.length; i++) {
    if (lyrics[i].time_secs <= pos) idx = i;
  }
  return idx;
}

function coverGradient(pri, acc, isLiked = false) {
  if (isLiked) {
    return "linear-gradient(135deg, #4826e0 0%, #6a72e8 55%, #9ae2d6 100%)";
  }
  const [r1, g1, b1] = pri;
  const [r2, g2, b2] = acc;
  return `linear-gradient(145deg, rgb(${Math.min(255, r1 + 36)},${Math.min(255, g1 + 36)},${Math.min(255, b1 + 36)}) 0%, rgb(${r1},${g1},${b1}) 52%, rgb(${Math.floor(r2 * 0.65)},${Math.floor(g2 * 0.65)},${Math.floor(b2 * 0.65)}) 100%)`;
}

function coverHTML(pri, acc, size, opts = {}) {
  const { round = false, shadowed = false, isLiked = false, full = false, mark = false, imageUrl = null } = opts;
  const style = full
    ? `background:${coverGradient(pri, acc, isLiked)};`
    : `width:${size}px;height:${size}px;background:${coverGradient(pri, acc, isLiked)};`;
  const cls = `cover${round ? " round" : ""}${shadowed ? " shadowed" : ""}`;
  let inner = "";
  if (isLiked) inner = ICONS.heart(Math.max(16, Math.round(size * 0.4)));
  else if (imageUrl) inner = `<img src="${esc(imageUrl)}" alt="" loading="lazy" />`;
  else if (mark) inner = `<span class="cover-mark">${ICONS.spotify(16)}</span>`;
  return `<div class="${cls}" style="${style}">${inner}</div>`;
}

function resetScrollAndPlaylistFilter() {
  ui.playlistSearchOpen = false;
  ui.playlistQuery = "";
  const scroller = document.getElementById("main-scroll");
  if (scroller) scroller.scrollTop = 0;
}

function maybeLoadPlaylistTracks() {
  if (ui.view.kind === "playlist" && snap?.spotify?.connected) {
    const p = snap.playlists[ui.view.index];
    if (p && !p.loaded && p.spotify_id) {
      callBackend("spotify_load_playlist", { playlistIdx: ui.view.index });
    }
  }
}

function navigate(next) {
  if (JSON.stringify(ui.view) === JSON.stringify(next)) return;
  ui.back.push(ui.view);
  ui.forward = [];
  ui.view = next;
  resetScrollAndPlaylistFilter();
  renderAll();
  maybeLoadPlaylistTracks();
}

function navigateBack() {
  if (!ui.back.length) return;
  ui.forward.push(ui.view);
  ui.view = ui.back.pop();
  resetScrollAndPlaylistFilter();
  renderAll();
  maybeLoadPlaylistTracks();
}

function navigateForward() {
  if (!ui.forward.length) return;
  ui.back.push(ui.view);
  ui.view = ui.forward.pop();
  resetScrollAndPlaylistFilter();
  renderAll();
  maybeLoadPlaylistTracks();
}

function ambientRGB() {
  if (!snap) return [30, 110, 72];
  if (ui.view.kind === "playlist") return snap.playlists[ui.view.index]?.primary_rgb || [29, 185, 84];
  if (ui.view.kind === "liked") return [92, 58, 220];
  if (ui.view.kind === "search") return [58, 66, 84];
  return currentTrack()?.primary_rgb || [30, 110, 72];
}

function renderTopBar() {
  document.getElementById("btn-back").disabled = ui.back.length === 0;
  document.getElementById("btn-fwd").disabled = ui.forward.length === 0;

  const homeBtn = document.getElementById("btn-home");
  const isHome = ui.view.kind === "home";
  homeBtn.classList.toggle("active", isHome);
  homeBtn.innerHTML = isHome ? ICONS.home(22) : ICONS.homeOutline(22);

  const hasQuery = ui.searchQuery.length > 0;
  document.getElementById("search-keycaps").classList.toggle("hidden", hasQuery);
  document.getElementById("btn-clear-search").classList.toggle("hidden", !hasQuery);

  const connBtn = document.getElementById("btn-spotify-connect");
  const sp = snap.spotify;
  connBtn.classList.toggle("connected", sp.connected);
  if (sp.connected) {
    const label = sp.active_device || (sp.audio_running ? "SpotiRust" : "Spotify");
    connBtn.innerHTML = `<span class="status-dot"></span><span>${esc(label)}</span>`;
    connBtn.title = "Spotify Connected • Switch device / Built-in audio";
  } else {
    connBtn.innerHTML = `${ICONS.spotify(15)}<span>${ui.loginBusy ? "Connecting..." : "Log in"}</span>`;
    connBtn.title = "Login with Spotify";
  }

  const avBtn = document.getElementById("btn-avatar");
  const avCore = document.getElementById("avatar-core");
  avBtn.title = sp.user_name;
  if (sp.user_avatar) {
    avCore.innerHTML = `<img src="${esc(sp.user_avatar)}" alt="${esc(sp.user_name)}" />`;
  } else {
    avCore.textContent = (sp.user_name || "K").trim().charAt(0).toUpperCase();
  }
}

function renderLibrary() {
  document.querySelectorAll("[data-lib-filter]").forEach((el) => {
    el.classList.toggle("active", ui.libFilter === el.dataset.libFilter);
  });

  const container = document.getElementById("library-list");
  let html = "";

  if (ui.libFilter !== "artists") {
    const likedCount = snap.tracks.filter((t) => t.liked).length;
    const likedActive = ui.view.kind === "liked" ? " active" : "";
    html += `
      <button class="lib-row${likedActive}" data-nav="liked" title="Liked Songs • ${likedCount} songs">
        ${coverHTML([98, 62, 232], [45, 212, 191], 48, { isLiked: true })}
        <div class="lib-row-text">
          <div class="lib-row-title">Liked Songs</div>
          <div class="lib-row-sub">Playlist &bull; ${likedCount} songs</div>
        </div>
      </button>`;

    snap.playlists.forEach((p, idx) => {
      const act = ui.view.kind === "playlist" && ui.view.index === idx ? " active" : "";
      html += `
        <button class="lib-row${act}" data-nav="playlist" data-idx="${idx}" title="${esc(p.name)}">
          ${coverHTML(p.primary_rgb, p.accent_rgb, 48, { imageUrl: p.image_url })}
          <div class="lib-row-text">
            <div class="lib-row-title">${esc(p.name)}</div>
            <div class="lib-row-sub">Playlist &bull; ${esc(p.creator)}</div>
          </div>
        </button>`;
    });
  }

  if (ui.libFilter !== "playlists") {
    snap.artists.forEach((a) => {
      html += `
        <button class="lib-row" data-artist="${esc(a.name)}" title="${esc(a.name)}">
          ${coverHTML(a.primary_rgb, a.accent_rgb, 48, { round: true, imageUrl: a.image_url })}
          <div class="lib-row-text">
            <div class="lib-row-title">${esc(a.name)}</div>
            <div class="lib-row-sub">Artist</div>
          </div>
        </button>`;
    });
  }

  container.innerHTML = html;
}

function renderTrackTableHTML(ids, ctxName) {
  const curId = currentTrack()?.id;
  let rows = "";
  ids.forEach((tid, idx) => {
    const t = snap.tracks[tid];
    if (!t) return;
    const isAct = curId === t.id;
    const eq = `<span class="eq-bars${snap.is_playing ? "" : " paused"}"><span></span><span></span><span></span></span>`;
    const playGlyph = isAct && snap.is_playing ? ICONS.pause(14) : ICONS.play(14);
    rows += `
      <div class="track-row${isAct ? " active" : ""}" data-play-track="${t.id}" data-ctx="${esc(ctxName)}" data-ids="${ids.join(",")}">
        <div class="col-num">
          ${isAct ? eq : `<span class="track-idx">${idx + 1}</span>`}
          <span class="track-play-hover">${playGlyph}</span>
        </div>
        <div class="track-title-cell">
          ${coverHTML(t.primary_rgb, t.accent_rgb, 40, { imageUrl: t.image_url })}
          <div class="track-names">
            <div class="track-name">${esc(t.title)}</div>
            <div class="track-sub">
              ${t.explicit ? `<span class="explicit-badge">E</span>` : ""}
              <span>${esc(t.artist)}</span>
            </div>
          </div>
        </div>
        <div class="track-album">${esc(t.album)}</div>
        <div class="col-num">
          <button class="row-like-btn${t.liked ? " liked" : ""}" data-like="${t.id}" title="Save to Liked Songs">
            ${t.liked ? ICONS.checkCircle(16) : ICONS.addCircle(16)}
          </button>
        </div>
        <div class="col-dur" style="color:var(--text-secondary);font-size:13px;">${fmtTime(t.duration_secs)}</div>
      </div>`;
  });

  return `
    <div class="track-table-hdr">
      <div class="col-num">#</div>
      <div>Title</div>
      <div>Album</div>
      <div></div>
      <div class="col-dur">${ICONS.clock(15)}</div>
    </div>
    <div>${rows}</div>`;
}

function filterPlaylistIds(ids) {
  const q = ui.playlistQuery.trim().toLowerCase();
  if (!q) return ids;
  return ids.filter((tid) => {
    const t = snap.tracks[tid];
    return t && (t.title.toLowerCase().includes(q) || t.artist.toLowerCase().includes(q) || t.album.toLowerCase().includes(q));
  });
}

function updateStickyBar() {
  const bar = document.getElementById("sticky-bar");
  const scroller = document.getElementById("main-scroll");
  const show = (ui.view.kind === "playlist" || ui.view.kind === "liked") && scroller.scrollTop > 220;
  bar.classList.toggle("visible", show);
}

function renderMain() {
  if (ui.view.kind === "playlist" && !snap.playlists[ui.view.index]) {
    ui.view = { kind: "home" };
  }
  const mainCard = document.getElementById("main-card");
  const mainScroll = document.getElementById("main-scroll");
  const stickyBar = document.getElementById("sticky-bar");
  const [r, g, b] = ambientRGB();

  const isLyrics = ui.view.kind === "lyrics";
  mainCard.classList.toggle("lyrics-mode", isLyrics);

  if (isLyrics) {
    // Deep muted Spotify-style desaturated canvas
    const mr = Math.round(r * 0.28 + 18);
    const mg = Math.round(g * 0.28 + 18);
    const mb = Math.round(b * 0.28 + 18);
    mainCard.style.background = `linear-gradient(180deg, rgb(${mr},${mg},${mb}) 0%, rgb(${Math.round(mr * 0.78)},${Math.round(mg * 0.78)},${Math.round(mb * 0.78)}) 100%)`;
  } else {
    const alpha = ui.view.kind === "playlist" || ui.view.kind === "liked" ? 0.42 : 0.16;
    mainCard.style.background = `linear-gradient(180deg, rgba(${r},${g},${b},${alpha}) 0%, #121212 260px, #121212 100%)`;
  }

  if (ui.view.kind !== "playlist" && ui.view.kind !== "liked") {
    stickyBar.classList.remove("visible");
    stickyBar.innerHTML = "";
  }

  if (ui.view.kind === "home") {
    const likedIds = snap.tracks.filter((t) => t.liked).map((t) => t.id);
    let html = `
      <div class="home-chips">
        ${["all", "music", "playlists", "liked"]
          .map((k) => `<button class="chip${ui.homeFilter === k ? " active" : ""}" data-home-filter="${k}">${k[0].toUpperCase() + k.slice(1)}</button>`)
          .join("")}
      </div>`;

    if (ui.homeFilter !== "liked") {
      const quick = [
        { title: "Liked Songs", pri: [98, 62, 232], acc: [45, 212, 191], isLiked: true, nav: "liked", ids: likedIds, imageUrl: null },
        ...snap.playlists.slice(0, 7).map((p, idx) => ({
          title: p.name,
          pri: p.primary_rgb,
          acc: p.accent_rgb,
          isLiked: false,
          nav: "playlist",
          idx,
          ids: p.track_ids,
          imageUrl: p.image_url,
        })),
      ];

      html += `<div class="quick-grid">`;
      quick.forEach((q) => {
        html += `
          <div class="quick-tile" data-nav="${q.nav}" ${q.idx !== undefined ? `data-idx="${q.idx}"` : ""}>
            ${coverHTML(q.pri, q.acc, 48, { isLiked: q.isLiked, imageUrl: q.imageUrl })}
            <span class="quick-tile-title">${esc(q.title)}</span>
            <button class="float-play" data-quick-play="${esc(q.title)}" data-ids="${q.ids.join(",")}" title="Play">${ICONS.play(14)}</button>
          </div>`;
      });
      html += `</div>`;

      if (snap.playlists.length > 0) {
        html += `
          <div class="section-hdr">
            <div>
              <div class="section-eyebrow">Made For</div>
              <div class="section-title">${esc(snap.spotify.user_name)}</div>
            </div>
            <button class="section-link" data-home-filter="playlists">Show all</button>
          </div>
          <div class="shelf-grid">`;

        snap.playlists.slice(0, 5).forEach((p, idx) => {
          html += `
            <div class="shelf-card" data-nav="playlist" data-idx="${idx}">
              <div class="shelf-cover-wrap">
                ${coverHTML(p.primary_rgb, p.accent_rgb, 160, { full: true, shadowed: true, mark: true, imageUrl: p.image_url })}
                <button class="float-play" data-quick-play="${esc(p.name)}" data-ids="${p.track_ids.join(",")}" title="Play">${ICONS.play(18)}</button>
              </div>
              <div class="shelf-card-title">${esc(p.name)}</div>
              <div class="shelf-card-desc">${esc(p.description || `By ${p.creator}`)}</div>
            </div>`;
        });
        html += `</div>`;
      }
    }

    const isLikedFilter = ui.homeFilter === "liked";
    const trackIds = isLikedFilter ? likedIds : snap.tracks.slice(0, 8).map((t) => t.id);
    const title = isLikedFilter ? "Your Liked Songs" : "Jump Back In";
    const eyebrow = isLikedFilter ? "Collection" : "Recently Played";
    html += `
      <div class="section-hdr">
        <div>
          <div class="section-eyebrow">${eyebrow}</div>
          <div class="section-title">${title}</div>
        </div>
      </div>
      ${
        trackIds.length
          ? renderTrackTableHTML(trackIds, title)
          : `<div style="padding:20px 0;color:var(--text-secondary);">Belum ada lagu di daftar ini.</div>`
      }`;

    mainScroll.innerHTML = html;
  } else if (ui.view.kind === "search") {
    const q = ui.searchQuery.trim().toLowerCase();
    const hits =
      snap.spotify.connected && q
        ? snap.search_result_ids
        : snap.tracks
            .filter((t) => !q || t.title.toLowerCase().includes(q) || t.artist.toLowerCase().includes(q) || t.album.toLowerCase().includes(q))
            .map((t) => t.id);

    mainScroll.innerHTML = `
      <div class="section-hdr" style="margin-bottom:18px;">
        <div>
          <div class="section-title">${q ? `Results for "${esc(ui.searchQuery.trim())}"` : "Browse All"}</div>
          ${!q ? `<div class="section-eyebrow" style="margin-top:4px;">Press Ctrl+K anytime to search songs, artists, or albums${snap.spotify.connected ? " on Spotify" : ""}.</div>` : ""}
        </div>
      </div>
      ${
        hits.length
          ? renderTrackTableHTML(hits, "Search Results")
          : `<div style="padding:28px 0;color:var(--text-secondary);">No results found for "${esc(ui.searchQuery.trim())}".</div>`
      }`;
  } else if (ui.view.kind === "liked" || ui.view.kind === "playlist") {
    const isLiked = ui.view.kind === "liked";
    const p = isLiked ? null : snap.playlists[ui.view.index];
    const title = isLiked ? "Liked Songs" : p.name;
    const desc = isLiked ? "All your favorite tracks in one place." : p.description;
    const creator = isLiked ? snap.spotify.user_name : p.creator;
    const rawIds = isLiked ? snap.tracks.filter((t) => t.liked).map((t) => t.id) : p.track_ids;
    const ids = sortTrackIds(rawIds);
    const filteredIds = filterPlaylistIds(ids);
    const sortLabel = ui.sortMode === "custom" ? "Custom order" : ui.sortMode === "title" ? "Title" : "Duration";
    const pri = isLiked ? [98, 62, 232] : p.primary_rgb;
    const acc = isLiked ? [45, 212, 191] : p.accent_rgb;
    const heroImg = isLiked ? null : p.image_url;
    const isLoadingPlaylist = !isLiked && snap.spotify.connected && !p.loaded;
    const isCtxPlaying = snap.is_playing && JSON.stringify(snap.queue) === JSON.stringify(ids) && ids.length > 0;

    stickyBar.style.background = `rgb(${Math.round(pri[0] * 0.36 + 14)},${Math.round(pri[1] * 0.36 + 14)},${Math.round(pri[2] * 0.36 + 14)})`;
    stickyBar.innerHTML = `
      <button class="sticky-play-btn" data-hero-play="${esc(title)}" data-ids="${ids.join(",")}">
        ${isCtxPlaying ? ICONS.pause(18) : ICONS.play(18)}
      </button>
      <div class="sticky-title">${esc(title)}</div>`;

    const searchControl = ui.playlistSearchOpen
      ? `
        <div class="pl-search-box">
          ${ICONS.search(14)}
          <input id="pl-search-input" class="pl-search-input" type="text" placeholder="Search in playlist" value="${esc(ui.playlistQuery)}" autocomplete="off" spellcheck="false" />
          <button class="icon-btn sm" data-action="close-pl-search" title="Clear filter" style="width:20px;height:20px;">${ICONS.close(12)}</button>
        </div>`
      : `<button class="icon-btn sm" data-action="open-pl-search" title="Search in playlist">${ICONS.search(16)}</button>`;

    mainScroll.innerHTML = `
      <div class="hero">
        ${coverHTML(pri, acc, 192, { isLiked, shadowed: true, mark: !isLiked, imageUrl: heroImg })}
        <div class="hero-meta">
          <div class="hero-kind">Public Playlist</div>
          <div class="hero-title">${esc(title)}</div>
          <div class="hero-desc">${esc(desc)}</div>
          <div class="hero-stats">
            <span class="spotify-dot">${ICONS.spotify(18)}</span>
            <span>${esc(creator)} &bull; ${ids.length} songs${ids.length ? `, ${fmtTotal(ids)}` : ""}</span>
          </div>
        </div>
      </div>
      <div class="action-bar">
        <div class="action-left">
          <button class="hero-play-btn" data-hero-play="${esc(title)}" data-ids="${ids.join(",")}">
            ${isCtxPlaying ? ICONS.pause(22) : ICONS.play(22)}
          </button>
          <button class="icon-btn" data-action="shuffle" title="Shuffle" style="${snap.shuffle ? "color:var(--green)" : ""}">${ICONS.shuffle(22)}</button>
          <button class="icon-btn" data-action="save-playlist" title="Save to Your Library">${ICONS.addCircle(24)}</button>
          <button class="icon-btn" data-action="download-playlist" title="Download">${ICONS.download(24)}</button>
          <button class="icon-btn" data-action="more-playlist" title="More options">${ICONS.more(22)}</button>
        </div>
        <div class="action-right">
          ${searchControl}
          <button id="btn-sort" class="action-right" data-action="sort-menu" style="gap:8px;padding:4px 8px;border-radius:4px;">
            <span>${sortLabel}</span>
            ${ICONS.list(15)}
          </button>
        </div>
      </div>
      <div id="pl-track-table">
        ${
          isLoadingPlaylist
            ? `<div style="padding:24px 12px;color:var(--text-secondary);font-size:13.5px;">Memuat lagu dari Spotify...</div>`
            : filteredIds.length
              ? renderTrackTableHTML(filteredIds, title)
              : `<div style="padding:24px 12px;color:var(--text-secondary);font-size:13.5px;">${
                  ui.playlistQuery.trim()
                    ? `No songs matching "${esc(ui.playlistQuery.trim())}".`
                    : "Belum ada lagu di playlist ini (atau playlist milik orang lain dibatasi oleh Spotify Development Mode)."
                }</div>`
        }
      </div>`;
    updateStickyBar();
  } else if (ui.view.kind === "lyrics") {
    const activeIdx = activeLyricIndex(snap.lyrics, snap.position_secs);
    const lines = snap.lyrics
      .map((line, i) => {
        const cls = i === activeIdx ? "active" : i < activeIdx ? "past" : "";
        return `<button class="lyric-line ${cls}" data-lyric-idx="${i}" data-seek-sec="${line.time_secs}">${esc(line.text)}</button>`;
      })
      .join("");
    mainScroll.innerHTML = `<div class="lyrics-wrap">${lines}</div>`;
    syncLyricsScroll();
  }
}

function syncLyricsScroll() {
  if (ui.view.kind !== "lyrics" || !snap) return;
  const track = currentTrack();
  const idx = activeLyricIndex(snap.lyrics, snap.position_secs);
  const key = `${track?.id}:${idx}`;
  if (key === ui.lastLyricKey || idx < 0) return;
  ui.lastLyricKey = key;
  const el = document.querySelector(`.lyric-line[data-lyric-idx="${idx}"]`);
  if (el) el.scrollIntoView({ behavior: "smooth", block: "center" });
}

function renderNowPlaying() {
  const card = document.getElementById("now-playing-card");
  const resizeRight = document.getElementById("resize-right");
  const visible = ui.rightPanel !== null;
  card.classList.toggle("hidden", !visible);
  resizeRight.classList.toggle("hidden", !visible);
  if (!visible) return;

  const t = currentTrack();
  const scroll = document.getElementById("np-scroll");

  if (ui.rightPanel === "queue") {
    document.getElementById("np-context").textContent = "Queue";
    const start = snap.queue_pos + 1;
    const hasUpcoming = start < snap.queue.length;
    let upcomingHtml = "";
    if (!hasUpcoming) {
      upcomingHtml = `<div style="font-size:12.5px;color:var(--text-muted);padding:4px 0;">Queue is empty. Right-click any song to add it to queue.</div>`;
    } else {
      for (let i = start; i < snap.queue.length; i++) {
        const qt = snap.tracks[snap.queue[i]];
        if (!qt) continue;
        upcomingHtml += `
          <div class="queue-row-wrap">
            <button class="queue-row" data-jump-queue="${i}">
              ${coverHTML(qt.primary_rgb, qt.accent_rgb, 38, { imageUrl: qt.image_url })}
              <div class="lib-row-text">
                <div class="lib-row-title" style="font-size:13px;">${esc(qt.title)}</div>
                <div class="lib-row-sub">${esc(qt.artist)}</div>
              </div>
            </button>
            <button class="icon-btn sm queue-remove-btn" data-remove-queue="${i}" title="Remove from queue">${ICONS.close(13)}</button>
          </div>`;
      }
    }

    scroll.innerHTML = `
      ${
        t
          ? `
        <div class="np-subcard">
          <div class="np-subcard-hdr"><span>Now playing</span></div>
          <div class="queue-row" style="cursor:default;">
            ${coverHTML(t.primary_rgb, t.accent_rgb, 40, { imageUrl: t.image_url })}
            <div class="lib-row-text">
              <div class="lib-row-title" style="font-size:13.5px;color:var(--green);">${esc(t.title)}</div>
              <div class="lib-row-sub">${esc(t.artist)}</div>
            </div>
          </div>
        </div>`
          : ""
      }
      <div class="np-subcard">
        <div class="np-subcard-hdr">
          <span>Next up (${Math.max(0, snap.queue.length - start)})</span>
          ${hasUpcoming ? `<button class="chip" data-action="clear-queue" style="padding:3px 10px;font-size:11px;">Clear queue</button>` : ""}
        </div>
        <div>${upcomingHtml}</div>
      </div>`;
    return;
  }

  document.getElementById("np-context").textContent = snap.active_context_name;
  if (!t) return;

  const activeIdx = Math.max(0, activeLyricIndex(snap.lyrics, snap.position_secs));
  const curLine = snap.lyrics[activeIdx]?.text || "";
  const nextLine = snap.lyrics[activeIdx + 1]?.text || "";
  const [r, g, b] = t.primary_rgb;
  const lyricsCardBg = `rgb(${Math.round(r * 0.34 + 18)},${Math.round(g * 0.34 + 18)},${Math.round(b * 0.34 + 18)})`;

  const artistObj = snap.artists.find((a) => a.name === t.artist);
  const listeners = artistObj ? artistObj.monthly_listeners : "642,300 monthly listeners";

  let queueHtml = "";
  const start = snap.queue_pos + 1;
  if (start >= snap.queue.length) {
    queueHtml = `<div style="font-size:12.5px;color:var(--text-muted);">End of queue</div>`;
  } else {
    for (let i = start; i < Math.min(start + 5, snap.queue.length); i++) {
      const qt = snap.tracks[snap.queue[i]];
      if (!qt) continue;
      queueHtml += `
        <button class="queue-row" data-jump-queue="${i}">
          ${coverHTML(qt.primary_rgb, qt.accent_rgb, 38, { imageUrl: qt.image_url })}
          <div class="lib-row-text">
            <div class="lib-row-title" style="font-size:13px;">${esc(qt.title)}</div>
            <div class="lib-row-sub">${esc(qt.artist)}</div>
          </div>
        </button>`;
    }
  }

  scroll.innerHTML = `
    <div class="np-art">${coverHTML(t.primary_rgb, t.accent_rgb, 250, { full: true, shadowed: true, mark: true, imageUrl: t.image_url })}</div>
    <div class="np-meta">
      <div class="lib-row-text">
        <div class="np-track-title">${esc(t.title)}</div>
        <div class="np-track-artist">${esc(t.artist)}</div>
      </div>
      <button class="icon-btn" data-like="${t.id}" style="${t.liked ? "color:var(--green)" : ""}">
        ${t.liked ? ICONS.checkCircle(20) : ICONS.addCircle(20)}
      </button>
    </div>
    ${
      ui.view.kind !== "lyrics"
        ? `
      <div class="np-subcard" style="background:${lyricsCardBg};">
        <div class="np-subcard-hdr">
          <span>Lyrics</span>
          <button class="chip" data-action="toggle-lyrics" style="padding:4px 11px;font-size:11.5px;">Show lyrics</button>
        </div>
        <div class="np-lyric-cur">${esc(curLine)}</div>
        <div class="np-lyric-next">${esc(nextLine)}</div>
      </div>`
        : ""
    }
    <div class="np-subcard">
      <div class="np-subcard-hdr"><span>About the artist</span></div>
      <div class="np-about-artist">${esc(t.artist)}</div>
      <div class="np-about-stat">${esc(listeners)}</div>
      <div class="np-about-album">Album: ${esc(t.album)}</div>
    </div>
    <div class="np-subcard">
      <div class="np-subcard-hdr">
        <span>Next in queue</span>
        <button class="section-link" data-action="open-queue" style="font-size:12px;">Open queue</button>
      </div>
      <div>${queueHtml}</div>
    </div>`;
}

function renderPlayerBar() {
  const t = currentTrack();
  const left = document.getElementById("player-left");
  if (t) {
    left.innerHTML = `
      ${coverHTML(t.primary_rgb, t.accent_rgb, 56, { shadowed: true, imageUrl: t.image_url })}
      <div class="player-track-info">
        <div class="player-track-title">${esc(t.title)}</div>
        <div class="player-track-artist">${esc(t.artist)}</div>
      </div>
      <button class="icon-btn sm" data-like="${t.id}" style="${t.liked ? "color:var(--green)" : ""}">
        ${t.liked ? ICONS.checkCircle(17) : ICONS.addCircle(17)}
      </button>`;
  } else {
    left.innerHTML = `<div class="player-track-info"><div class="player-track-artist">Nothing playing</div></div>`;
  }

  document.getElementById("btn-play").innerHTML = snap.is_playing ? ICONS.pause(15) : ICONS.play(15);

  const shuf = document.getElementById("btn-shuffle");
  shuf.classList.toggle("active", snap.shuffle);

  const rep = document.getElementById("btn-repeat");
  rep.classList.toggle("active", snap.repeat !== "off");
  rep.innerHTML = snap.repeat === "one" ? ICONS.repeatOne(16) : ICONS.repeat(16);

  document.getElementById("btn-lyrics").classList.toggle("active", ui.view.kind === "lyrics");
  document.getElementById("btn-queue").classList.toggle("active", ui.rightPanel === "queue");
  document.getElementById("btn-devices").classList.toggle("active", Boolean(snap.spotify.active_device || snap.spotify.audio_running));
  document.getElementById("btn-np").classList.toggle("active", ui.rightPanel === "now_playing");

  const effVol = snap.muted ? 0 : snap.volume;
  const volBtn = document.getElementById("btn-volume");
  volBtn.innerHTML = effVol <= 0.01 ? ICONS.volMute(16) : effVol < 0.45 ? ICONS.volLow(16) : ICONS.volHigh(16);

  updateProgressUI();
}

function updateProgressUI() {
  if (!snap) return;
  const t = currentTrack();
  const dur = Math.max(1, t?.duration_secs || 1);
  const pos = Math.min(dur, snap.position_secs);
  const pct = (pos / dur) * 100;

  document.getElementById("time-elapsed").textContent = fmtTime(pos);
  document.getElementById("time-duration").textContent = fmtTime(dur);
  document.getElementById("seek-fill").style.width = `${pct}%`;
  document.getElementById("seek-thumb").style.left = `${pct}%`;

  const volPct = (snap.muted ? 0 : snap.volume) * 100;
  document.getElementById("volume-fill").style.width = `${volPct}%`;
  document.getElementById("volume-thumb").style.left = `${volPct}%`;

  if (ui.view.kind === "lyrics") {
    const idx = activeLyricIndex(snap.lyrics, snap.position_secs);
    document.querySelectorAll(".lyric-line").forEach((el) => {
      const i = Number(el.dataset.lyricIdx);
      el.classList.toggle("active", i === idx);
      el.classList.toggle("past", i < idx);
    });
    syncLyricsScroll();
  } else if (ui.rightPanel === "now_playing") {
    const activeIdx = Math.max(0, activeLyricIndex(snap.lyrics, snap.position_secs));
    const curEl = document.querySelector(".np-lyric-cur");
    const nextEl = document.querySelector(".np-lyric-next");
    if (curEl) curEl.textContent = snap.lyrics[activeIdx]?.text || "";
    if (nextEl) nextEl.textContent = snap.lyrics[activeIdx + 1]?.text || "";
  }
}

function renderAll() {
  if (!snap) return;
  renderTopBar();
  renderLibrary();
  renderMain();
  renderNowPlaying();
  renderPlayerBar();
}

function surfaceBackendError() {
  const err = snap?.spotify?.last_error || null;
  if (err && err !== ui.lastErrorShown) {
    ui.lastErrorShown = err;
    showToast(err);
  } else if (!err) {
    ui.lastErrorShown = null;
  }
}

async function callBackend(cmd, args = {}) {
  try {
    snap = await invoke(cmd, args);
    renderAll();
    surfaceBackendError();
    return true;
  } catch (err) {
    showToast(String(err));
    return false;
  }
}

function wireRangeBar(elId, onChange) {
  const el = document.getElementById(elId);
  let dragging = false;
  const calc = (e) => {
    const rect = el.getBoundingClientRect();
    return Math.min(1, Math.max(0, (e.clientX - rect.left) / rect.width));
  };
  el.addEventListener("pointerdown", (e) => {
    dragging = true;
    el.classList.add("dragging");
    el.setPointerCapture(e.pointerId);
    onChange(calc(e));
  });
  el.addEventListener("pointermove", (e) => {
    if (dragging) onChange(calc(e));
  });
  const end = () => {
    dragging = false;
    el.classList.remove("dragging");
  };
  el.addEventListener("pointerup", end);
  el.addEventListener("pointercancel", end);
}

function wireResize(handleId, targetId, minW, maxW, fromRight = false) {
  const handle = document.getElementById(handleId);
  const target = document.getElementById(targetId);
  let active = false;
  let startX = 0;
  let startW = 0;

  handle.addEventListener("pointerdown", (e) => {
    active = true;
    startX = e.clientX;
    startW = target.getBoundingClientRect().width;
    handle.classList.add("dragging");
    target.classList.add("resizing");
    handle.setPointerCapture(e.pointerId);
    e.preventDefault();
  });
  handle.addEventListener("pointermove", (e) => {
    if (!active) return;
    const delta = fromRight ? startX - e.clientX : e.clientX - startX;
    const nextW = Math.round(Math.min(maxW, Math.max(minW, startW + delta)));
    target.style.width = `${nextW}px`;
  });
  const end = () => {
    if (!active) return;
    active = false;
    handle.classList.remove("dragging");
    target.classList.remove("resizing");
  };
  handle.addEventListener("pointerup", end);
  handle.addEventListener("pointercancel", end);
}

function initStaticIcons() {
  document.getElementById("btn-back").innerHTML = ICONS.chevronLeft(16);
  document.getElementById("btn-fwd").innerHTML = ICONS.chevronRight(16);
  document.getElementById("search-icon").innerHTML = ICONS.search(18);
  document.getElementById("btn-clear-search").innerHTML = ICONS.close(15);
  document.getElementById("btn-browse").innerHTML = ICONS.browse(19);
  document.getElementById("btn-bell").innerHTML = ICONS.bell(16);
  document.getElementById("btn-friends").innerHTML = ICONS.users(16);
  document.getElementById("icon-library").innerHTML = ICONS.library(20);
  document.getElementById("icon-plus").innerHTML = ICONS.plus(13);
  document.getElementById("lib-search-icon").innerHTML = ICONS.search(15);
  document.getElementById("lib-list-icon").innerHTML = ICONS.list(14);
  document.getElementById("btn-close-np").innerHTML = ICONS.close(15);
  document.getElementById("btn-shuffle").innerHTML = ICONS.shuffle(16);
  document.getElementById("btn-prev").innerHTML = ICONS.prev(16);
  document.getElementById("btn-next").innerHTML = ICONS.next(16);
  document.getElementById("btn-repeat").innerHTML = ICONS.repeat(16);
  document.getElementById("btn-lyrics").innerHTML = ICONS.mic(16);
  document.getElementById("btn-queue").innerHTML = ICONS.queue(16);
  document.getElementById("btn-devices").innerHTML = ICONS.device(16);
  document.getElementById("btn-np").innerHTML = ICONS.nowPlaying(16);
}

function sortTrackIds(ids) {
  const copy = [...ids];
  if (ui.sortMode === "title") {
    copy.sort((a, b) => (snap.tracks[a]?.title || "").localeCompare(snap.tracks[b]?.title || ""));
  } else if (ui.sortMode === "duration") {
    copy.sort((a, b) => (snap.tracks[a]?.duration_secs || 0) - (snap.tracks[b]?.duration_secs || 0));
  }
  return copy;
}

let toastTimer = null;
function showToast(msg) {
  const el = document.getElementById("toast");
  el.textContent = msg;
  el.classList.remove("hidden");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => el.classList.add("hidden"), 2400);
}

function closePopover() {
  ui.openPopover = null;
  document.getElementById("popover").classList.add("hidden");
  document.getElementById("btn-menu").classList.remove("open");
}

function openPopover(kind, anchorEl, html, alignRight = false) {
  const pop = document.getElementById("popover");
  if (ui.openPopover === kind) {
    closePopover();
    return;
  }
  closePopover();
  ui.openPopover = kind;
  pop.innerHTML = html;
  pop.classList.remove("hidden");
  if (kind === "menu") document.getElementById("btn-menu").classList.add("open");

  const rect = anchorEl.getBoundingClientRect();
  const popW = pop.offsetWidth || 230;
  let left = alignRight ? rect.right - popW : rect.left;
  left = Math.max(10, Math.min(window.innerWidth - popW - 10, left));
  const top = Math.min(window.innerHeight - pop.offsetHeight - 16, rect.bottom + 8);
  pop.style.left = `${left}px`;
  pop.style.top = `${Math.max(10, top)}px`;
}

function openContextMenu(x, y, html) {
  const pop = document.getElementById("popover");
  closePopover();
  ui.openPopover = "ctx";
  pop.innerHTML = html;
  pop.classList.remove("hidden");

  const popW = pop.offsetWidth || 220;
  const popH = pop.offsetHeight || 200;
  const left = Math.max(10, Math.min(window.innerWidth - popW - 10, x));
  const top = Math.max(10, Math.min(window.innerHeight - popH - 10, y));
  pop.style.left = `${left}px`;
  pop.style.top = `${top}px`;
}

function closeModal() {
  document.getElementById("modal-backdrop").classList.add("hidden");
}

function openModal(html) {
  closePopover();
  document.getElementById("modal-box").innerHTML = html;
  document.getElementById("modal-backdrop").classList.remove("hidden");
  const input = document.querySelector("#modal-box input");
  if (input) setTimeout(() => input.focus(), 20);
}

function showCreatePlaylistModal() {
  openModal(`
    <div class="modal-hdr">
      <div class="modal-title">Create playlist</div>
      <button class="icon-btn sm" data-close-modal>${ICONS.close(15)}</button>
    </div>
    <p class="modal-desc">Buat playlist baru di katalog SpotiRust kamu.</p>
    <input id="new-playlist-name" class="modal-input" type="text" placeholder="My Playlist #${snap.playlists.length + 1}" />
    <div class="modal-actions">
      <button class="btn-secondary" data-close-modal>Cancel</button>
      <button class="btn-primary" id="confirm-create-playlist">Create</button>
    </div>`);

  const submit = async () => {
    const name = document.getElementById("new-playlist-name").value;
    closeModal();
    await callBackend("create_playlist", { name });
    navigate({ kind: "playlist", index: snap.playlists.length - 1 });
    showToast(`Playlist "${snap.playlists[snap.playlists.length - 1].name}" dibuat`);
  };
  document.getElementById("confirm-create-playlist").addEventListener("click", submit);
  document.getElementById("new-playlist-name").addEventListener("keydown", (e) => {
    if (e.key === "Enter") submit();
  });
}

function showRenamePlaylistModal(idx) {
  const p = snap.playlists[idx];
  if (!p) return;
  openModal(`
    <div class="modal-hdr">
      <div class="modal-title">Rename playlist</div>
      <button class="icon-btn sm" data-close-modal>${ICONS.close(15)}</button>
    </div>
    <input id="rename-playlist-name" class="modal-input" type="text" value="${esc(p.name)}" />
    <div class="modal-actions">
      <button class="btn-secondary" data-close-modal>Cancel</button>
      <button class="btn-primary" id="confirm-rename-playlist">Save</button>
    </div>`);

  const input = document.getElementById("rename-playlist-name");
  setTimeout(() => input.select(), 25);
  const submit = async () => {
    const name = input.value.trim();
    closeModal();
    if (!name) return;
    await callBackend("rename_playlist", { playlistIdx: idx, name });
    showToast(`Playlist diubah ke "${name}"`);
  };
  document.getElementById("confirm-rename-playlist").addEventListener("click", submit);
  input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") submit();
  });
}

async function deletePlaylistByIndex(idx) {
  const name = snap.playlists[idx]?.name || "Playlist";
  const fixHist = (list) =>
    list
      .filter((v) => !(v.kind === "playlist" && v.index === idx))
      .map((v) => (v.kind === "playlist" && v.index > idx ? { kind: "playlist", index: v.index - 1 } : v));
  ui.back = fixHist(ui.back);
  ui.forward = fixHist(ui.forward);
  if (ui.view.kind === "playlist") {
    if (ui.view.index === idx) ui.view = { kind: "home" };
    else if (ui.view.index > idx) ui.view = { kind: "playlist", index: ui.view.index - 1 };
  }
  await callBackend("delete_playlist", { playlistIdx: idx });
  showToast(`Playlist "${name}" dihapus`);
}

async function triggerSpotifyLogin() {
  if (ui.loginBusy) return;
  ui.loginBusy = true;
  renderTopBar();
  showToast("Membuka browser untuk login Spotify...");
  try {
    snap = await invoke("spotify_login");
    ui.back = [];
    ui.forward = [];
    ui.view = { kind: "home" };
    renderAll();
    showToast(`Terhubung sebagai ${snap.spotify.user_name}!`);
  } catch (err) {
    showToast(String(err));
  } finally {
    ui.loginBusy = false;
    renderTopBar();
  }
}

function openDevicesPopover(anchorEl, alignRight = true) {
  const sp = snap.spotify;
  const devItems = sp.devices.length
    ? sp.devices
        .map(
          (d) => `
          <button class="pop-item" data-transfer-dev="${esc(d.id)}">
            <div>
              <div style="font-weight:700;color:${d.is_active ? "var(--green)" : "var(--text-primary)"};">${esc(d.name)}</div>
              <div style="font-size:11px;color:var(--text-secondary);">${esc(d.device_type)}${d.is_active ? " • Active" : ""}</div>
            </div>
            <span>${d.is_active ? "●" : ""}</span>
          </button>`
        )
        .join("")
    : `<div style="padding:10px 12px;font-size:12.5px;color:var(--text-secondary);">Belum ada device Spotify aktif. Aktifkan Built-in Speaker di bawah untuk memutar langsung dari PC ini.</div>`;

  openPopover(
    "devices",
    anchorEl,
    `
    <div class="pop-header">Connect to a device</div>
    <button class="pop-item" data-pop-cmd="toggle-audio">
      <div>
        <div style="font-weight:700;color:${sp.audio_running ? "var(--green)" : "var(--text-primary)"};">Built-in Speaker (SpotiRust)</div>
        <div style="font-size:11px;color:var(--text-secondary);">${sp.audio_running ? "Active • Klik untuk matikan" : "Klik untuk nyalakan audio di PC ini"}</div>
      </div>
      <span class="pop-shortcut">${sp.audio_running ? "ON" : "OFF"}</span>
    </button>
    <div class="pop-divider"></div>
    ${devItems}
    <div class="pop-divider"></div>
    <button class="pop-item" data-pop-cmd="sync-devices"><span>Refresh devices</span></button>
    <button class="pop-item" data-pop-cmd="settings"><span>Spotify &amp; Audio Setup</span></button>`,
    alignRight
  );
}

function showApiSettingsModal() {
  const sp = snap.spotify;
  openModal(`
    <div class="modal-hdr">
      <div class="modal-title">Spotify &amp; Built-in Audio Setup</div>
      <button class="icon-btn sm" data-close-modal>${ICONS.close(15)}</button>
    </div>

    <div class="status-row">
      <span>Spotify Account</span>
      <span class="status-badge ${sp.connected ? "ok" : "warn"}">${sp.connected ? `Connected • ${esc(sp.user_name)}` : "Demo Mode"}</span>
    </div>

    <label class="modal-label" for="cfg-client-id">Spotify Web API Client ID</label>
    <input id="cfg-client-id" class="modal-input" type="text" value="${esc(sp.client_id)}" placeholder="Client ID dari Developer Dashboard" style="margin-bottom:8px;" />
    <div class="shortcut-row" style="margin-bottom:8px;"><span>Redirect URI</span><span class="pop-shortcut">http://127.0.0.1:8888/callback</span></div>

    <div class="modal-section-hdr">Built-in Audio Speaker (librespot)</div>
    <div class="status-row">
      <span>Audio Engine</span>
      <span class="status-badge ${sp.audio_running ? "ok" : sp.audio_cached ? "" : "warn"}">${
        sp.audio_running ? "Running (Device: SpotiRust)" : sp.audio_cached ? "Stopped (Saved)" : "Not enabled"
      }</span>
    </div>
    <p class="modal-desc" style="margin-bottom:14px;">
      Putar audio Spotify langsung dari speaker PC ini tanpa aplikasi Spotify lain atau download tambahan (butuh akun Spotify Premium).
    </p>

    <div class="modal-actions" style="justify-content:space-between;flex-wrap:wrap;gap:8px;">
      <div style="display:flex;gap:8px;">
        ${
          sp.connected
            ? `<button class="btn-danger" id="btn-modal-logout">Disconnect</button>`
            : `<button class="btn-primary" id="btn-modal-login">Login with Spotify</button>`
        }
        <button class="btn-secondary" id="btn-modal-audio">${sp.audio_running ? "Stop Audio" : "Enable Built-in Audio"}</button>
      </div>
      <button class="btn-secondary" id="btn-modal-save">Save Client ID</button>
    </div>`);

  const saveCfg = async (silent = false) => {
    const clientId = document.getElementById("cfg-client-id").value;
    const ok = await callBackend("spotify_save_settings", { clientId });
    if (ok && !silent) showToast("Client ID tersimpan");
    return ok;
  };

  document.getElementById("btn-modal-save").addEventListener("click", async () => {
    if (await saveCfg(false)) showApiSettingsModal();
  });
  document.getElementById("btn-modal-audio").addEventListener("click", async () => {
    if (!snap.spotify.audio_running) {
      showToast(snap.spotify.audio_cached ? "Menyalakan Built-in Speaker..." : "Membuka browser untuk otorisasi Built-in Speaker...");
    }
    const ok = await callBackend("audio_toggle");
    if (ok) {
      showToast(snap.spotify.audio_running ? 'Built-in Speaker "SpotiRust" aktif!' : "Built-in Speaker dimatikan");
      showApiSettingsModal();
    }
  });
  const loginBtn = document.getElementById("btn-modal-login");
  if (loginBtn) {
    loginBtn.addEventListener("click", async () => {
      await saveCfg(true);
      closeModal();
      await triggerSpotifyLogin();
    });
  }
  const logoutBtn = document.getElementById("btn-modal-logout");
  if (logoutBtn) {
    logoutBtn.addEventListener("click", async () => {
      closeModal();
      await callBackend("spotify_logout");
      ui.back = [];
      ui.forward = [];
      ui.view = { kind: "home" };
      renderAll();
      showToast("Akun Spotify diputus (kembali ke Demo Mode)");
    });
  }
}

function showProfileModal() {
  const sp = snap.spotify;
  const likedCount = snap.tracks.filter((t) => t.liked).length;
  const initial = (sp.user_name || "K").trim().charAt(0).toUpperCase();
  openModal(`
    <div class="modal-hdr">
      <div class="modal-title">Profile</div>
      <button class="icon-btn sm" data-close-modal>${ICONS.close(15)}</button>
    </div>
    <div style="display:flex;align-items:center;gap:14px;margin-bottom:16px;">
      <div class="avatar-ring" style="width:56px;height:56px;cursor:default;">
        <span class="avatar-core" style="width:42px;height:42px;font-size:19px;">
          ${sp.user_avatar ? `<img src="${esc(sp.user_avatar)}" alt="" />` : esc(initial)}
        </span>
      </div>
      <div>
        <div style="font-size:20px;font-weight:800;">${esc(sp.user_name)}</div>
        <div style="font-size:12.5px;color:var(--text-secondary);">${sp.connected ? "Spotify Live" : "Demo Preview"} &bull; ${snap.playlists.length} Playlists &bull; ${likedCount} Liked Songs</div>
      </div>
    </div>
    <div class="modal-actions">
      <button class="btn-primary" data-close-modal>Close</button>
    </div>`);
}

function showShortcutsModal() {
  openModal(`
    <div class="modal-hdr">
      <div class="modal-title">Keyboard Shortcuts</div>
      <button class="icon-btn sm" data-close-modal>${ICONS.close(15)}</button>
    </div>
    <div class="shortcut-row"><span>Play / Pause</span><span class="keycaps"><kbd>Space</kbd></span></div>
    <div class="shortcut-row"><span>Search</span><span class="keycaps"><kbd>Ctrl</kbd><kbd>K</kbd></span></div>
    <div class="shortcut-row"><span>Close menu / dialog</span><span class="keycaps"><kbd>Esc</kbd></span></div>
    <div class="modal-actions" style="margin-top:16px;">
      <button class="btn-primary" data-close-modal>Close</button>
    </div>`);
}

function wireEvents() {
  document.getElementById("btn-back").addEventListener("click", navigateBack);
  document.getElementById("btn-fwd").addEventListener("click", navigateForward);
  document.getElementById("btn-home").addEventListener("click", () => navigate({ kind: "home" }));
  document.getElementById("btn-browse").addEventListener("click", () => navigate({ kind: "search" }));

  // Top-left 3-dots App Menu dropdown
  document.getElementById("btn-menu").addEventListener("click", (e) => {
    e.stopPropagation();
    openPopover(
      "menu",
      e.currentTarget,
      `
      <button class="pop-item" data-pop-cmd="new-playlist"><span>New Playlist</span></button>
      <div class="pop-divider"></div>
      <button class="pop-item" data-pop-cmd="toggle-play"><span>${snap.is_playing ? "Pause" : "Play"}</span><span class="pop-shortcut">Space</span></button>
      <button class="pop-item" data-pop-cmd="next-track"><span>Next Track</span></button>
      <button class="pop-item" data-pop-cmd="prev-track"><span>Previous Track</span></button>
      <button class="pop-item" data-pop-cmd="toggle-shuffle"><span>Shuffle: ${snap.shuffle ? "On" : "Off"}</span></button>
      <div class="pop-divider"></div>
      <button class="pop-item" data-pop-cmd="toggle-np"><span>${ui.rightPanel === "now_playing" ? "Hide" : "Show"} Now Playing</span></button>
      <button class="pop-item" data-pop-cmd="toggle-queue"><span>${ui.rightPanel === "queue" ? "Hide" : "Open"} Queue</span></button>
      <button class="pop-item" data-pop-cmd="toggle-lyrics"><span>${ui.view.kind === "lyrics" ? "Close" : "Open"} Lyrics</span></button>
      <div class="pop-divider"></div>
      <button class="pop-item" data-pop-cmd="shortcuts"><span>Keyboard Shortcuts</span></button>
      <button class="pop-item" data-pop-cmd="settings"><span>Spotify API Settings</span></button>`,
      false
    );
  });

  // Top-right Spotify Login / Active Device pill
  document.getElementById("btn-spotify-connect").addEventListener("click", (e) => {
    e.stopPropagation();
    if (!snap.spotify.connected) {
      triggerSpotifyLogin();
    } else {
      openDevicesPopover(e.currentTarget, true);
    }
  });

  // Top-right Bell (What's New) popover
  document.getElementById("btn-bell").addEventListener("click", (e) => {
    e.stopPropagation();
    const picks = snap.tracks.slice(0, 4);
    const items = picks.length
      ? picks
          .map(
            (t, idx) => `
          <button class="pop-feed-item" data-pop-play="${t.id}">
            ${coverHTML(t.primary_rgb, t.accent_rgb, 40, { imageUrl: t.image_url })}
            <div class="lib-row-text">
              <div class="pop-feed-title">${esc(t.title)}</div>
              <div class="pop-feed-sub">${esc(t.artist)} &bull; ${idx === 0 ? "Featured" : "Recently synced"}</div>
            </div>
          </button>`
          )
          .join("")
      : `<div style="padding:10px 12px;font-size:12.5px;color:var(--text-secondary);">No updates yet.</div>`;
    openPopover("bell", e.currentTarget, `<div class="pop-header">What's New</div>${items}`, true);
  });

  // Top-right Friends (Friend Activity) popover
  document.getElementById("btn-friends").addEventListener("click", (e) => {
    e.stopPropagation();
    const names = ["Raka", "Nadia", "Bima"];
    const agos = ["2m", "7m", "14m"];
    const items = names
      .map((name, i) => {
        const t = snap.tracks[i];
        if (!t) return "";
        return `
          <button class="pop-feed-item" data-pop-play="${t.id}">
            ${coverHTML(t.primary_rgb, t.accent_rgb, 38, { round: true, imageUrl: t.image_url })}
            <div class="lib-row-text">
              <div class="pop-feed-title">${esc(name)} <span style="font-weight:400;color:var(--text-muted);font-size:11px;">• ${agos[i]}</span></div>
              <div class="pop-feed-sub">${esc(t.title)} &bull; ${esc(t.artist)}</div>
            </div>
          </button>`;
      })
      .join("");
    openPopover("friends", e.currentTarget, `<div class="pop-header">Friend Activity</div>${items || '<div style="padding:10px 12px;font-size:12.5px;color:var(--text-secondary);">No activity.</div>'}`, true);
  });

  // Top-right Avatar (Profile) dropdown
  document.getElementById("btn-avatar").addEventListener("click", (e) => {
    e.stopPropagation();
    const sp = snap.spotify;
    const initial = (sp.user_name || "K").trim().charAt(0).toUpperCase();
    openPopover(
      "avatar",
      e.currentTarget,
      `
      <div class="pop-profile-hdr">
        <span class="avatar-core">${sp.user_avatar ? `<img src="${esc(sp.user_avatar)}" alt="" />` : esc(initial)}</span>
        <div>
          <div class="pop-profile-name">${esc(sp.user_name)}</div>
          <div class="pop-profile-sub">${sp.connected ? "Spotify Connected" : "Demo Preview Mode"}</div>
        </div>
      </div>
      <div class="pop-divider"></div>
      <button class="pop-item" data-pop-cmd="profile"><span>Profile</span></button>
      <button class="pop-item" data-pop-cmd="settings"><span>Spotify &amp; Audio Setup</span></button>
      <button class="pop-item" data-pop-cmd="shortcuts"><span>Keyboard Shortcuts</span></button>
      <div class="pop-divider"></div>
      ${
        sp.connected
          ? `<button class="pop-item" data-pop-cmd="spotify-logout" style="color:#f15e6c;"><span>Log out of Spotify</span></button>`
          : `<button class="pop-item" data-pop-cmd="spotify-login" style="color:var(--green);"><span>Log in with Spotify</span></button>`
      }`,
      true
    );
  });

  // Sidebar "+ Create" button
  document.getElementById("btn-create").addEventListener("click", (e) => {
    e.stopPropagation();
    openPopover(
      "create",
      e.currentTarget,
      `
      <button class="pop-item" data-pop-cmd="new-playlist">
        <div>
          <div style="font-weight:700;">Playlist</div>
          <div style="font-size:11.5px;color:var(--text-secondary);">Build a playlist with songs</div>
        </div>
      </button>
      <button class="pop-item" data-pop-cmd="liked-shortcut">
        <div>
          <div style="font-weight:700;">Liked Songs</div>
          <div style="font-size:11.5px;color:var(--text-secondary);">Jump to your saved tracks</div>
        </div>
      </button>`,
      false
    );
  });

  // Handle clicks inside Popover & Modal
  document.getElementById("popover").addEventListener("click", (e) => {
    e.stopPropagation();
    const cmdBtn = e.target.closest("[data-pop-cmd]");
    if (cmdBtn) {
      const cmd = cmdBtn.dataset.popCmd;
      closePopover();
      if (cmd === "new-playlist") showCreatePlaylistModal();
      else if (cmd === "toggle-play") callBackend("toggle_play");
      else if (cmd === "next-track") callBackend("next_track");
      else if (cmd === "prev-track") callBackend("prev_track");
      else if (cmd === "toggle-shuffle") callBackend("toggle_shuffle");
      else if (cmd === "toggle-np") {
        ui.rightPanel = ui.rightPanel === "now_playing" ? null : "now_playing";
        renderNowPlaying();
        renderPlayerBar();
      } else if (cmd === "toggle-queue") {
        ui.rightPanel = ui.rightPanel === "queue" ? null : "queue";
        renderNowPlaying();
        renderPlayerBar();
      } else if (cmd === "toggle-lyrics") {
        ui.view.kind === "lyrics" ? (ui.back.length ? navigateBack() : navigate({ kind: "home" })) : navigate({ kind: "lyrics" });
      } else if (cmd === "shortcuts") showShortcutsModal();
      else if (cmd === "settings") showApiSettingsModal();
      else if (cmd === "profile") showProfileModal();
      else if (cmd === "liked-shortcut") navigate({ kind: "liked" });
      else if (cmd === "spotify-login") triggerSpotifyLogin();
      else if (cmd === "spotify-logout") {
        callBackend("spotify_logout").then(() => {
          ui.back = [];
          ui.forward = [];
          ui.view = { kind: "home" };
          renderAll();
          showToast("Akun Spotify diputus (kembali ke Demo Mode)");
        });
      } else if (cmd === "toggle-audio") {
        if (!snap.spotify.audio_running) {
          showToast(snap.spotify.audio_cached ? "Menyalakan Built-in Speaker..." : "Membuka browser untuk otorisasi Built-in Speaker...");
        }
        callBackend("audio_toggle").then((ok) => {
          if (ok) showToast(snap.spotify.audio_running ? 'Built-in Speaker "SpotiRust" aktif!' : "Built-in Speaker dimatikan");
        });
      } else if (cmd === "sync-devices") {
        callBackend("spotify_sync_player").then(() => showToast("Daftar device diperbarui"));
      } else if (cmd === "reset-demo") {
        ui.back = [];
        ui.forward = [];
        ui.view = { kind: "home" };
        ui.homeFilter = "all";
        ui.libFilter = "all";
        ui.sortMode = "custom";
        renderAll();
        showToast("Demo view di-reset");
      }
      return;
    }

    const transferBtn = e.target.closest("[data-transfer-dev]");
    if (transferBtn) {
      const devId = transferBtn.dataset.transferDev;
      closePopover();
      callBackend("spotify_transfer_device", { deviceId: devId }).then((ok) => {
        if (ok) showToast("Playback dipindahkan ke device terpilih");
      });
      return;
    }

    const sortBtn = e.target.closest("[data-set-sort]");
    if (sortBtn) {
      ui.sortMode = sortBtn.dataset.setSort;
      closePopover();
      renderMain();
      return;
    }

    const playBtn = e.target.closest("[data-pop-play]");
    if (playBtn) {
      const tid = Number(playBtn.dataset.popPlay);
      closePopover();
      callBackend("play_in_context", {
        contextName: "Quick Play",
        trackIds: snap.tracks.map((t) => t.id),
        trackId: tid,
      });
      return;
    }

    const addQueueBtn = e.target.closest("[data-ctx-add-queue]");
    if (addQueueBtn) {
      const tid = Number(addQueueBtn.dataset.ctxAddQueue);
      closePopover();
      callBackend("add_to_queue", { trackId: tid });
      showToast(`"${snap.tracks[tid]?.title || "Track"}" ditambahkan ke Queue`);
      return;
    }

    const ctxLikeBtn = e.target.closest("[data-ctx-like]");
    if (ctxLikeBtn) {
      const tid = Number(ctxLikeBtn.dataset.ctxLike);
      const wasLiked = snap.tracks[tid]?.liked;
      closePopover();
      callBackend("toggle_like", { trackId: tid });
      showToast(wasLiked ? "Dihapus dari Liked Songs" : "Disimpan ke Liked Songs");
      return;
    }

    const ctxArtistBtn = e.target.closest("[data-ctx-artist]");
    if (ctxArtistBtn) {
      const artist = ctxArtistBtn.dataset.ctxArtist;
      closePopover();
      ui.searchQuery = artist;
      document.getElementById("search-input").value = artist;
      navigate({ kind: "search" });
      if (snap.spotify.connected) callBackend("spotify_search", { query: artist });
      return;
    }

    const addPlBtn = e.target.closest("[data-ctx-add-pl]");
    if (addPlBtn) {
      const [pIdx, tid] = addPlBtn.dataset.ctxAddPl.split(":").map(Number);
      const plName = snap.playlists[pIdx]?.name || "playlist";
      closePopover();
      callBackend("add_track_to_playlist", { playlistIdx: pIdx, trackId: tid });
      showToast(`Ditambahkan ke "${plName}"`);
      return;
    }

    const remPlBtn = e.target.closest("[data-ctx-rem-pl]");
    if (remPlBtn) {
      const [pIdx, tid] = remPlBtn.dataset.ctxRemPl.split(":").map(Number);
      closePopover();
      callBackend("remove_track_from_playlist", { playlistIdx: pIdx, trackId: tid });
      showToast("Dihapus dari playlist");
      return;
    }

    const playPlBtn = e.target.closest("[data-ctx-play-pl]");
    if (playPlBtn) {
      const idx = Number(playPlBtn.dataset.ctxPlayPl);
      const p = snap.playlists[idx];
      closePopover();
      if (p && p.track_ids.length) {
        callBackend("play_in_context", {
          contextName: p.name,
          trackIds: p.track_ids,
          trackId: p.track_ids[0],
        });
      } else {
        showToast("Playlist masih kosong");
      }
      return;
    }

    const renPlBtn = e.target.closest("[data-ctx-rename-pl]");
    if (renPlBtn) {
      const idx = Number(renPlBtn.dataset.ctxRenamePl);
      closePopover();
      showRenamePlaylistModal(idx);
      return;
    }

    const delPlBtn = e.target.closest("[data-ctx-del-pl]");
    if (delPlBtn) {
      const idx = Number(delPlBtn.dataset.ctxDelPl);
      closePopover();
      deletePlaylistByIndex(idx);
    }
  });

  document.getElementById("modal-backdrop").addEventListener("click", (e) => {
    if (e.target.id === "modal-backdrop" || e.target.closest("[data-close-modal]")) {
      closeModal();
    }
  });

  document.addEventListener("click", () => closePopover());

  let searchDebounce = null;
  const searchInput = document.getElementById("search-input");
  searchInput.addEventListener("input", () => {
    ui.searchQuery = searchInput.value;
    if (ui.searchQuery.length > 0 && ui.view.kind !== "search") {
      navigate({ kind: "search" });
    } else {
      renderTopBar();
      if (ui.view.kind === "search") renderMain();
    }
    if (snap?.spotify?.connected) {
      clearTimeout(searchDebounce);
      const q = ui.searchQuery.trim();
      if (q.length >= 2) {
        searchDebounce = setTimeout(() => callBackend("spotify_search", { query: q }), 340);
      }
    }
  });
  document.getElementById("btn-clear-search").addEventListener("click", () => {
    ui.searchQuery = "";
    searchInput.value = "";
    renderTopBar();
    if (ui.view.kind === "search") renderMain();
  });

  document.getElementById("btn-collapse-lib").addEventListener("click", () => {
    ui.libCollapsed = !ui.libCollapsed;
    document.getElementById("sidebar-left").classList.toggle("collapsed", ui.libCollapsed);
    document.getElementById("resize-left").classList.toggle("hidden", ui.libCollapsed);
    document.getElementById("btn-collapse-lib").title = ui.libCollapsed ? "Expand Your Library" : "Collapse Your Library";
  });

  wireResize("resize-left", "sidebar-left", 210, 420, false);
  wireResize("resize-right", "now-playing-card", 240, 420, true);

  const mainScroll = document.getElementById("main-scroll");
  mainScroll.addEventListener("scroll", updateStickyBar);
  mainScroll.addEventListener("input", (e) => {
    if (e.target.id !== "pl-search-input") return;
    ui.playlistQuery = e.target.value;
    const isLiked = ui.view.kind === "liked";
    const p = isLiked ? null : snap.playlists[ui.view.index];
    if (!isLiked && !p) return;
    const title = isLiked ? "Liked Songs" : p.name;
    const rawIds = isLiked ? snap.tracks.filter((t) => t.liked).map((t) => t.id) : p.track_ids;
    const filteredIds = filterPlaylistIds(sortTrackIds(rawIds));
    const tableWrap = document.getElementById("pl-track-table");
    if (tableWrap) {
      tableWrap.innerHTML = filteredIds.length
        ? renderTrackTableHTML(filteredIds, title)
        : `<div style="padding:24px 12px;color:var(--text-secondary);font-size:13.5px;">No songs matching "${esc(ui.playlistQuery.trim())}".</div>`;
    }
  });

  document.getElementById("btn-close-np").addEventListener("click", () => {
    ui.rightPanel = null;
    renderNowPlaying();
    renderPlayerBar();
  });
  document.getElementById("btn-np").addEventListener("click", () => {
    ui.rightPanel = ui.rightPanel === "now_playing" ? null : "now_playing";
    renderNowPlaying();
    renderPlayerBar();
  });
  document.getElementById("btn-queue").addEventListener("click", () => {
    ui.rightPanel = ui.rightPanel === "queue" ? null : "queue";
    renderNowPlaying();
    renderPlayerBar();
  });
  document.getElementById("btn-devices").addEventListener("click", (e) => {
    e.stopPropagation();
    if (!snap.spotify.connected) {
      showApiSettingsModal();
    } else {
      openDevicesPopover(e.currentTarget, true);
    }
  });
  document.getElementById("btn-lyrics").addEventListener("click", () => {
    if (ui.view.kind === "lyrics") {
      ui.back.length ? navigateBack() : navigate({ kind: "home" });
    } else {
      navigate({ kind: "lyrics" });
    }
  });

  document.getElementById("btn-play").addEventListener("click", () => callBackend("toggle_play"));
  document.getElementById("btn-next").addEventListener("click", () => callBackend("next_track"));
  document.getElementById("btn-prev").addEventListener("click", () => callBackend("prev_track"));
  document.getElementById("btn-shuffle").addEventListener("click", () => callBackend("toggle_shuffle"));
  document.getElementById("btn-repeat").addEventListener("click", () => callBackend("cycle_repeat"));
  document.getElementById("btn-volume").addEventListener("click", () => callBackend("toggle_mute"));

  wireRangeBar("seek-bar", (ratio) => callBackend("seek_ratio", { ratio }));
  wireRangeBar("volume-bar", (volume) => callBackend("set_volume", { volume }));

  // Delegated click handler for dynamic content
  document.getElementById("app").addEventListener("click", (e) => {
    const likeBtn = e.target.closest("[data-like]");
    if (likeBtn) {
      e.stopPropagation();
      callBackend("toggle_like", { trackId: Number(likeBtn.dataset.like) });
      return;
    }

    const quickPlay = e.target.closest("[data-quick-play]");
    if (quickPlay) {
      e.stopPropagation();
      const ids = quickPlay.dataset.ids.split(",").filter(Boolean).map(Number);
      if (ids.length) {
        callBackend("play_in_context", {
          contextName: quickPlay.dataset.quickPlay,
          trackIds: ids,
          trackId: ids[0],
        });
      }
      return;
    }

    const heroPlay = e.target.closest("[data-hero-play]");
    if (heroPlay) {
      const ids = heroPlay.dataset.ids.split(",").filter(Boolean).map(Number);
      if (!ids.length) return;
      if (JSON.stringify(snap.queue) === JSON.stringify(ids)) {
        callBackend("toggle_play");
      } else {
        callBackend("play_in_context", {
          contextName: heroPlay.dataset.heroPlay,
          trackIds: ids,
          trackId: ids[0],
        });
      }
      return;
    }

    const actionEl = e.target.closest("[data-action]");
    if (actionEl) {
      e.stopPropagation();
      const act = actionEl.dataset.action;
      if (act === "shuffle") callBackend("toggle_shuffle");
      else if (act === "toggle-lyrics") navigate({ kind: "lyrics" });
      else if (act === "open-queue") {
        ui.rightPanel = "queue";
        renderNowPlaying();
        renderPlayerBar();
      } else if (act === "clear-queue") {
        callBackend("clear_queue");
        showToast("Queue dikosongkan");
      } else if (act === "open-pl-search") {
        ui.playlistSearchOpen = true;
        renderMain();
        const input = document.getElementById("pl-search-input");
        if (input) input.focus();
      } else if (act === "close-pl-search") {
        ui.playlistSearchOpen = false;
        ui.playlistQuery = "";
        renderMain();
      } else if (act === "save-playlist") showToast("Disimpan ke Your Library (Demo)");
      else if (act === "download-playlist") showToast("Download offline butuh Spotify Premium");
      else if (act === "more-playlist") {
        const plExtras =
          ui.view.kind === "playlist"
            ? `
            <button class="pop-item" data-ctx-rename-pl="${ui.view.index}"><span>Rename playlist</span></button>
            <button class="pop-item" data-ctx-del-pl="${ui.view.index}" style="color:#f15e6c;"><span>Delete playlist</span></button>
            <div class="pop-divider"></div>`
            : "";
        openPopover(
          "more-playlist",
          actionEl,
          `
          ${plExtras}
          <button class="pop-item" data-pop-cmd="new-playlist"><span>Create similar playlist</span></button>
          <button class="pop-item" data-pop-cmd="toggle-shuffle"><span>Toggle Shuffle</span></button>
          <button class="pop-item" data-pop-cmd="settings"><span>Spotify API Settings</span></button>`,
          false
        );
      } else if (act === "sort-menu") {
        openPopover(
          "sort",
          actionEl,
          `
          <div class="pop-header">Sort by</div>
          <button class="pop-item" data-set-sort="custom"><span>Custom order</span><span>${ui.sortMode === "custom" ? "✓" : ""}</span></button>
          <button class="pop-item" data-set-sort="title"><span>Title</span><span>${ui.sortMode === "title" ? "✓" : ""}</span></button>
          <button class="pop-item" data-set-sort="duration"><span>Duration</span><span>${ui.sortMode === "duration" ? "✓" : ""}</span></button>`,
          true
        );
      }
      return;
    }

    const trackRow = e.target.closest("[data-play-track]");
    if (trackRow) {
      const ids = trackRow.dataset.ids.split(",").filter(Boolean).map(Number);
      callBackend("play_in_context", {
        contextName: trackRow.dataset.ctx,
        trackIds: ids,
        trackId: Number(trackRow.dataset.playTrack),
      });
      return;
    }

    const lyricBtn = e.target.closest("[data-seek-sec]");
    if (lyricBtn) {
      callBackend("seek_secs", { secs: Number(lyricBtn.dataset.seekSec) });
      return;
    }

    const removeQueueBtn = e.target.closest("[data-remove-queue]");
    if (removeQueueBtn) {
      e.stopPropagation();
      callBackend("remove_from_queue", { idx: Number(removeQueueBtn.dataset.removeQueue) });
      return;
    }

    const queueBtn = e.target.closest("[data-jump-queue]");
    if (queueBtn) {
      callBackend("jump_queue", { idx: Number(queueBtn.dataset.jumpQueue) });
      return;
    }

    const libFilter = e.target.closest("[data-lib-filter]");
    if (libFilter) {
      const f = libFilter.dataset.libFilter;
      ui.libFilter = ui.libFilter === f ? "all" : f;
      renderLibrary();
      return;
    }

    const homeFilter = e.target.closest("[data-home-filter]");
    if (homeFilter) {
      ui.homeFilter = homeFilter.dataset.homeFilter;
      renderMain();
      return;
    }

    const artistBtn = e.target.closest("[data-artist]");
    if (artistBtn) {
      ui.searchQuery = artistBtn.dataset.artist;
      searchInput.value = ui.searchQuery;
      navigate({ kind: "search" });
      if (snap.spotify.connected) callBackend("spotify_search", { query: ui.searchQuery });
      return;
    }

    const navEl = e.target.closest("[data-nav]");
    if (navEl) {
      if (navEl.dataset.nav === "liked") navigate({ kind: "liked" });
      else if (navEl.dataset.nav === "playlist") navigate({ kind: "playlist", index: Number(navEl.dataset.idx) });
    }
  });

  // Right-click context menus for tracks & playlists
  document.getElementById("app").addEventListener("contextmenu", (e) => {
    const trackRow = e.target.closest(".track-row[data-play-track]");
    if (trackRow) {
      e.preventDefault();
      const tid = Number(trackRow.dataset.playTrack);
      const t = snap.tracks[tid];
      if (!t) return;
      const plItems = snap.playlists
        .map((p, idx) => `<button class="pop-item" data-ctx-add-pl="${idx}:${tid}"><span>${esc(p.name)}</span></button>`)
        .join("");
      const remItem =
        ui.view.kind === "playlist"
          ? `<button class="pop-item" data-ctx-rem-pl="${ui.view.index}:${tid}"><span>Remove from this playlist</span></button>`
          : "";
      openContextMenu(
        e.clientX,
        e.clientY,
        `
        <button class="pop-item" data-ctx-add-queue="${tid}"><span>Add to queue</span></button>
        <button class="pop-item" data-ctx-like="${tid}"><span>${t.liked ? "Remove from Liked Songs" : "Save to Liked Songs"}</span></button>
        ${remItem}
        <button class="pop-item" data-ctx-artist="${esc(t.artist)}"><span>Go to artist</span></button>
        ${plItems ? `<div class="pop-divider"></div><div class="pop-header">Add to playlist</div>${plItems}` : ""}`
      );
      return;
    }

    const plRow = e.target.closest('.lib-row[data-nav="playlist"]');
    if (plRow) {
      e.preventDefault();
      const idx = Number(plRow.dataset.idx);
      openContextMenu(
        e.clientX,
        e.clientY,
        `
        <button class="pop-item" data-ctx-play-pl="${idx}"><span>Play</span></button>
        <button class="pop-item" data-ctx-rename-pl="${idx}"><span>Rename playlist</span></button>
        <div class="pop-divider"></div>
        <button class="pop-item" data-ctx-del-pl="${idx}" style="color:#f15e6c;"><span>Delete playlist</span></button>`
      );
    }
  });

  // Keyboard shortcuts (Ctrl+K / Ctrl+L for search, Space for Play/Pause, Esc closes menus/modals)
  window.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      closePopover();
      closeModal();
      return;
    }
    const inInput = document.activeElement?.tagName === "INPUT";
    if ((e.ctrlKey || e.metaKey) && (e.key.toLowerCase() === "k" || e.key.toLowerCase() === "l")) {
      e.preventDefault();
      searchInput.focus();
      searchInput.select();
      return;
    }
    if (e.code === "Space" && !inInput) {
      e.preventDefault();
      callBackend("toggle_play");
    }
  });
}

async function boot() {
  initStaticIcons();
  wireEvents();
  snap = await invoke("get_state");
  renderAll();

  // Restore saved Spotify session / Soloist speaker silently on startup
  try {
    snap = await invoke("spotify_bootstrap");
    renderAll();
  } catch (_) {}

  // Smooth local progress tick (250ms)
  setInterval(async () => {
    if (!snap || !snap.is_playing) return;
    const prevId = currentTrack()?.id;
    snap = await invoke("get_state");
    if (currentTrack()?.id !== prevId) {
      renderAll();
    } else {
      updateProgressUI();
    }
  }, 250);

  // Live Spotify Connect sync every 3s when logged in
  setInterval(async () => {
    if (!snap?.spotify?.connected) return;
    try {
      const prevKey = `${currentTrack()?.spotify_id}:${snap.is_playing}:${snap.spotify.active_device}`;
      snap = await invoke("spotify_sync_player");
      const nextKey = `${currentTrack()?.spotify_id}:${snap.is_playing}:${snap.spotify.active_device}`;
      if (prevKey !== nextKey) renderAll();
      else updateProgressUI();
    } catch (_) {}
  }, 3000);
}

boot();
