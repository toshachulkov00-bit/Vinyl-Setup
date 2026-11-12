import "./style.css";
import { invoke } from "@tauri-apps/api/core";

const app = document.querySelector("#app");
app.innerHTML = `
  <main class="shell">
    <header class="top"><span class="brand"><i></i> VINYL</span><span class="live"><b></b> WINDOWS MEDIA</span></header>
    <section class="player">
      <div class="record-wrap"><div class="record" id="record"><div class="grooves"></div><div class="label"><span>V</span></div></div></div>
      <div class="details">
        <div class="eyebrow">NOW PLAYING</div>
        <h1 id="title">Нет воспроизведения</h1>
        <p id="artist">Запусти музыку в любом приложении</p>
        <div class="progress"><div id="progress"></div></div>
        <div class="times"><span id="elapsed">0:00</span><span id="duration">0:00</span></div>
        <div class="controls">
          <button id="prev" aria-label="Предыдущий трек">⏮</button>
          <button id="toggle" class="play" aria-label="Воспроизвести">▶</button>
          <button id="next" aria-label="Следующий трек">⏭</button>
        </div>
        <div class="status" id="status">Ожидание медиасессии Windows…</div>
      </div>
    </section>
    <footer><span>VINYL HUD</span><span>SMTC · WINDOWS 10/11</span></footer>
  </main>`;

const $ = (id) => document.getElementById(id);
const time = (seconds) => {
  if (!Number.isFinite(seconds) || seconds < 0) return "0:00";
  const s = Math.floor(seconds);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
};

let playing = false;
async function refresh() {
  try {
    const state = await invoke("media_state");
    $("title").textContent = state.title || "Неизвестный трек";
    $("artist").textContent = state.artist || "Неизвестный исполнитель";
    playing = state.playing;
    $("record").classList.toggle("spinning", playing);
    $("toggle").textContent = playing ? "Ⅱ" : "▶";
    $("progress").style.width = state.duration > 0 ? `${Math.min(100, state.position / state.duration * 100)}%` : "0%";
    $("elapsed").textContent = time(state.position);
    $("duration").textContent = time(state.duration);
    $("status").textContent = state.available ? (playing ? "СЕЙЧАС ИГРАЕТ" : "ВОСПРОИЗВЕДЕНИЕ НА ПАУЗЕ") : "Нет активной медиасессии";
  } catch (e) {
    $("status").textContent = "Не удалось прочитать Windows SMTC";
  }
}
async function control(action) {
  try { await invoke("media_control", { action }); await refresh(); } catch (e) { $("status").textContent = String(e); }
}
$("toggle").addEventListener("click", () => control("toggle"));
$("prev").addEventListener("click", () => control("previous"));
$("next").addEventListener("click", () => control("next"));
refresh();
setInterval(refresh, 1000);