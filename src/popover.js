// 托盘弹出窗：富内容 profile 列表 + 快捷操作
// 复用主窗口的主题/强调色/语言（同源 localStorage 共享）
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

const root = document.documentElement;
root.dataset.theme = localStorage.getItem('codex_manager_theme') || 'slate';
root.style.setProperty('--accent', localStorage.getItem('codex_manager_accent') || '#6e8ef2');
const lang = localStorage.getItem('codex_manager_lang') || 'en';

const T = {
  online:   { en: '{n} online',  zh: '{n} 个在线',     es: '{n} en línea' },
  launch:   { en: 'Launch',      zh: '启动',           es: 'Iniciar' },
  stop:     { en: 'Stop',        zh: '停止',           es: 'Detener' },
  open:     { en: 'Open',        zh: '打开',           es: 'Abrir' },
  stopAll:  { en: 'Stop All',    zh: '全部停止',       es: 'Detener todo' },
  quit:     { en: 'Quit',        zh: '退出',           es: 'Salir' },
  empty:    { en: 'No profiles', zh: '暂无账号',       es: 'Sin perfiles' },
  running:  { en: 'pid {n}',     zh: 'pid {n}',        es: 'pid {n}' },
  offline:  { en: 'offline',     zh: '离线',           es: 'inactivo' },
};
const tr = (k, n) => (T[k][lang] || T[k].en).replace('{n}', n ?? '');

let profiles = [];
let running = {};
const listEl = document.getElementById('popList');
const countEl = document.getElementById('onlineCount');

function escapeHtml(s) {
  return String(s).replace(/[&<>"']/g, m => ({ '&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#039;' }[m]));
}

function render() {
  const n = Object.keys(running).length;
  countEl.textContent = tr('online', n);
  countEl.style.color = n > 0 ? 'var(--green)' : 'var(--text-dim)';

  if (!profiles.length) {
    listEl.innerHTML = `<div class="pop-empty">${tr('empty')}</div>`;
    return;
  }
  listEl.innerHTML = profiles.map(p => {
    const pid = running[p.id];
    const on = !!pid;
    const status = on ? `● ${tr('running', pid)}` : `○ ${tr('offline')}`;
    const btnClass = on ? 'btn-danger-outline' : 'btn-primary';
    const action = on ? 'stop' : 'launch';
    const label = on ? tr('stop') : tr('launch');
    return `<div class="pop-row">
      <div class="pop-avatar" style="background:${p.color}22;color:${p.color}">${escapeHtml((p.name[0] || '?').toUpperCase())}</div>
      <div class="pop-meta">
        <div class="pop-name">${escapeHtml(p.name)}</div>
        <div class="pop-status ${on ? 'on' : 'off'}">${status}</div>
      </div>
      <button class="btn ${btnClass} pop-act" data-action="${action}" data-id="${escapeHtml(p.id)}">${label}</button>
    </div>`;
  }).join('');
}

async function refresh() {
  try {
    [profiles, running] = await Promise.all([invoke('get_profiles'), invoke('get_running_status')]);
  } catch (e) {
    console.error('popover refresh failed:', e);
    return;
  }
  render();
}

// 行内按钮（事件委托）
listEl.addEventListener('click', async (e) => {
  const btn = e.target.closest('.pop-act');
  if (!btn) return;
  const { id, action } = btn.dataset;
  try {
    if (action === 'launch') {
      await invoke('launch_profile', { profileId: id, projectPath: null });
    } else {
      await invoke('stop_profile', { profileId: id });
    }
  } catch (err) { console.error(err); }
  setTimeout(refresh, 250);
});

document.getElementById('openMainBtn').addEventListener('click', async () => {
  try { await invoke('show_main_window'); } catch (e) { console.error(e); }
  await getCurrentWindow().hide();
});
document.getElementById('stopAllBtn').addEventListener('click', async () => {
  try { await invoke('stop_all_profiles'); } catch (e) { console.error(e); }
  setTimeout(refresh, 250);
});
document.getElementById('quitBtn').addEventListener('click', () => {
  try { invoke('quit_app'); } catch (e) { console.error(e); }
});

// 本地化底部按钮
document.getElementById('openMainBtn').textContent = tr('open');
document.getElementById('stopAllBtn').textContent = tr('stopAll');
document.getElementById('quitBtn').textContent = tr('quit');

refresh();
setInterval(refresh, 2000);
