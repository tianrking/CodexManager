import { invoke as tauriInvoke } from '@tauri-apps/api/core';

let invoke = tauriInvoke;

// 三语字典 (EN / ZH / ES)
const translations = {
  en: {
    appTitle: "Codex Multi-Account Manager",
    appSubtitle: "Cross-Platform (macOS / Win / Linux) · Zero Session Overwrite · Native Agent Unrestricted",
    searchPlaceholder: "Search Profile... (Press '/' to focus)",
    onlineCount: "{count} Online",
    stopAll: "Stop All",
    addProfile: "Add Profile",
    noProfilesTitle: "No Codex Isolated Profiles",
    noProfilesDesc: "Click 'Add Profile' above to create a physically isolated Auth environment.",
    noMatchTitle: "No Profile Found",
    modalAddTitle: "Add Codex Isolated Profile",
    modalEditTitle: "Edit Codex Isolated Profile",
    labelProfileName: "Profile Name",
    namePlaceholder: "e.g., Work / Personal / Client-A",
    labelNote: "Description (Optional)",
    notePlaceholder: "e.g., Main work account, sufficient quota",
    labelProjectPath: "Default Project Path (Optional)",
    pathPlaceholder: "e.g., /Users/username/Developer/project",
    btnBrowse: "Browse...",
    labelBadgeColor: "Badge Color",
    btnCancel: "Cancel",
    btnSave: "Save Profile",
    statusOnline: "Online",
    statusOffline: "Offline",
    defaultPathLabel: "Default Project Directory (or drag folder here)",
    btnStop: "Stop",
    btnLaunch: "Launch",
    tipOpenFolder: "Open profile data folder in Finder / Explorer",
    tipEdit: "Edit Profile",
    tipDelete: "Delete Profile",
    confirmDelete: "Are you sure you want to delete profile '{name}'? Its isolated credential directory will be removed.",
    confirmStopAll: "Are you sure you want to stop all active Codex instances?",
    duplicateNameError: "A profile with this name already exists. Please use a unique name.",
    codexNotFound: "Codex App was not found in standard system locations. Please ensure Codex Desktop App is installed.",
    appearanceLabel: "Appearance",
    appearanceTheme: "Theme",
    appearanceAccent: "Accent"
  },
  zh: {
    appTitle: "Codex 多账号隔离管理器",
    appSubtitle: "跨平台支持 (macOS / Win / Linux) · 0 踢下线 · Agent 原生能力无损",
    searchPlaceholder: "搜索 Profile 账号... (按 '/' 聚焦)",
    onlineCount: "{count} 个在线中",
    stopAll: "全部停止",
    addProfile: "新增账号",
    noProfilesTitle: "尚无 Codex 隔离 Profile",
    noProfilesDesc: "点击上方「新增账号」创建物理隔离的 Auth 与凭据环境",
    noMatchTitle: "未找到匹配的账号",
    modalAddTitle: "新增 Codex 隔离账号 Profile",
    modalEditTitle: "编辑 Codex 账号 Profile",
    labelProfileName: "账号名称",
    namePlaceholder: "例如: Work / Personal / Client-A",
    labelNote: "备注说明 (可选)",
    notePlaceholder: "如：公司主账号，配额充足",
    labelProjectPath: "默认关联项目路径 (可选)",
    pathPlaceholder: "例如 /Users/username/Developer/project",
    btnBrowse: "浏览...",
    labelBadgeColor: "卡片标识颜色",
    btnCancel: "取消",
    btnSave: "保存 Profile",
    statusOnline: "在线",
    statusOffline: "离线",
    defaultPathLabel: "默认主页目录 (或拖拽文件夹启动)",
    btnStop: "停止运行",
    btnLaunch: "独立启动",
    tipOpenFolder: "在 Finder / 文件资源管理器中查看此 Profile 凭据目录",
    tipEdit: "编辑 Profile",
    tipDelete: "删除 Profile",
    confirmDelete: "确认注销该 Profile 账号？专属凭据隔离物理目录也将被注销。",
    confirmStopAll: "确认一键关闭所有由本管理器启动的 Codex 实例？",
    duplicateNameError: "已存在同名 Profile 账号，请输入唯一的名称。",
    codexNotFound: "未在系统标准路径检测到 Codex 桌面客户端，请确保已安装 Codex App。",
    appearanceLabel: "外观",
    appearanceTheme: "主题",
    appearanceAccent: "强调色"
  },
  es: {
    appTitle: "Administrador de Cuentas Codex",
    appSubtitle: "Multiplataforma (macOS / Win / Linux) · Cero Cierre de Sesión · Agente Nativo Completo",
    searchPlaceholder: "Buscar Perfil... (Presione '/' para enfocar)",
    onlineCount: "{count} En línea",
    stopAll: "Detener Todo",
    addProfile: "Añadir Perfil",
    noProfilesTitle: "Sin Perfiles de Codex Aislados",
    noProfilesDesc: "Haga clic en 'Añadir Perfil' arriba para crear un entorno aislado.",
    noMatchTitle: "Perfil no encontrado",
    modalAddTitle: "Añadir Perfil Aislado de Codex",
    modalEditTitle: "Editar Perfil de Codex",
    labelProfileName: "Nombre del Perfil",
    namePlaceholder: "ej., Trabajo / Personal / Cliente-A",
    labelNote: "Descripción (Opcional)",
    notePlaceholder: "ej., Cuenta principal de trabajo",
    labelProjectPath: "Ruta del Proyecto por Defecto (Opcional)",
    pathPlaceholder: "ej., /Users/usuario/Developer/proyecto",
    btnBrowse: "Examinar...",
    labelBadgeColor: "Color de la Tarjeta",
    btnCancel: "Cancelar",
    btnSave: "Guardar Perfil",
    statusOnline: "En línea",
    statusOffline: "Desconectado",
    defaultPathLabel: "Directorio por defecto (o arrastre una carpeta aquí)",
    btnStop: "Detener",
    btnLaunch: "Iniciar",
    tipOpenFolder: "Abrir carpeta de datos en Finder / Explorador",
    tipEdit: "Editar Perfil",
    tipDelete: "Eliminar Perfil",
    confirmDelete: "¿Está seguro de eliminar el perfil '{name}'? Se eliminará su directorio aislado.",
    confirmStopAll: "¿Está seguro de detener todas las instancias activas de Codex?",
    duplicateNameError: "Ya existe un perfil con este nombre. Por favor use un nombre único.",
    codexNotFound: "No se encontró la aplicación Codex en las ubicaciones estándar del sistema.",
    appearanceLabel: "Apariencia",
    appearanceTheme: "Tema",
    appearanceAccent: "Acento"
  }
};

let currentLang = localStorage.getItem('codex_manager_lang') || 'en';

// 本地 Mock 数据 (仅用于离线/非 Tauri 预演)
let localProfiles = [
  {
    id: "work_account",
    name: "Work Account",
    note: "Work / Corporate Profile (Isolated Auth)",
    color: "#007AFF",
    default_project_path: ""
  },
  {
    id: "personal_account",
    name: "Personal Account",
    note: "Personal Open Source & Private Projects",
    color: "#34C759",
    default_project_path: ""
  }
];
let localRunningPids = {};
let pollingInterval = null;

// DOM 节点引用
const profileGrid = document.getElementById('profileGrid');
const emptyState = document.getElementById('emptyState');
const statusDot = document.getElementById('statusDot');
const runningCountText = document.getElementById('runningCountText');
const stopAllBtn = document.getElementById('stopAllBtn');
const searchInput = document.getElementById('searchInput');
const langSelect = document.getElementById('langSelect');

const modalOverlay = document.getElementById('modalOverlay');
const addProfileBtn = document.getElementById('addProfileBtn');
const closeModalBtn = document.getElementById('closeModalBtn');
const cancelModalBtn = document.getElementById('cancelModalBtn');
const profileForm = document.getElementById('profileForm');
const modalTitle = document.getElementById('modalTitle');
const editProfileId = document.getElementById('editProfileId');
const profileNameInput = document.getElementById('profileName');
const profileNoteInput = document.getElementById('profileNote');
const projectPathInput = document.getElementById('projectPath');
const browsePathBtn = document.getElementById('browsePathBtn');
const colorOptions = document.querySelectorAll('.color-option');

// 外观（主题 / 强调色）DOM
const themeSwatchEls = document.querySelectorAll('.theme-swatch');
const accentPicker = document.getElementById('accentPicker');
const accentHex = document.getElementById('accentHex');
const appearanceBtn = document.getElementById('appearanceBtn');
const appearancePopover = document.getElementById('appearancePopover');
const DEFAULT_ACCENT = '#6e8ef2';

let selectedColor = '#007AFF';

// 应用主题 / 强调色并持久化
function applyTheme(name) {
  document.documentElement.dataset.theme = name;
  localStorage.setItem('codex_manager_theme', name);
  themeSwatchEls.forEach(el => el.classList.toggle('active', el.dataset.theme === name));
}
function applyAccent(color) {
  const c = (color || DEFAULT_ACCENT).toLowerCase();
  document.documentElement.style.setProperty('--accent', c);
  localStorage.setItem('codex_manager_accent', c);
  if (accentPicker) accentPicker.value = c;
  if (accentHex) accentHex.textContent = c;
}
// 尽早应用（模块在解析后、DOMContentLoaded 前执行），减少闪烁
applyTheme(localStorage.getItem('codex_manager_theme') || 'slate');
applyAccent(localStorage.getItem('codex_manager_accent') || DEFAULT_ACCENT);

// i18n 辅助函数
function t(key, vars = {}) {
  let str = (translations[currentLang] && translations[currentLang][key]) || translations['en'][key] || key;
  for (const [k, v] of Object.entries(vars)) {
    str = str.replace(`{${k}}`, v);
  }
  return str;
}

function updateUILanguage() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    el.textContent = t(key);
  });
  
  document.querySelectorAll('[data-i18n-ph]').forEach(el => {
    const key = el.getAttribute('data-i18n-ph');
    el.placeholder = t(key);
  });

  document.querySelectorAll('[data-i18n-title]').forEach(el => {
    el.title = t(el.getAttribute('data-i18n-title'));
  });

  if (langSelect) {
    langSelect.value = currentLang;
  }

  updateStatusBadge();
  renderProfiles();
}

// 初始化与键盘快捷键
document.addEventListener('DOMContentLoaded', () => {
  initEventListeners();
  initAppearance();
  initKeyboardShortcuts();
  updateUILanguage();
  loadData();
  pollingInterval = setInterval(fetchRunningStatus, 1500);
});

window.addEventListener('beforeunload', () => {
  if (pollingInterval) clearInterval(pollingInterval);
});

function initKeyboardShortcuts() {
  document.addEventListener('keydown', (e) => {
    // 按 '/' 快捷聚焦搜索框 (非输入状态下)
    if (e.key === '/' && document.activeElement.tagName !== 'INPUT' && document.activeElement.tagName !== 'TEXTAREA') {
      e.preventDefault();
      searchInput.focus();
    }
    // 按 'Esc' 关闭 Modal / 外观面板
    if (e.key === 'Escape') {
      if (!modalOverlay.classList.contains('hidden')) closeModal();
      else if (appearancePopover && !appearancePopover.classList.contains('hidden')) appearancePopover.classList.add('hidden');
    }
  });
}

// 外观面板：主题切换 + 自定义强调色
function initAppearance() {
  appearanceBtn.addEventListener('click', (e) => {
    e.stopPropagation();
    appearancePopover.classList.toggle('hidden');
  });
  themeSwatchEls.forEach(el => {
    el.addEventListener('click', () => applyTheme(el.dataset.theme));
  });
  accentPicker.addEventListener('input', (e) => applyAccent(e.target.value));
  // 点击面板外部关闭
  document.addEventListener('click', (e) => {
    if (appearancePopover.classList.contains('hidden')) return;
    if (!e.target.closest('.popover-wrap')) appearancePopover.classList.add('hidden');
  });
}

function initEventListeners() {
  addProfileBtn.addEventListener('click', () => openModal());
  closeModalBtn.addEventListener('click', closeModal);
  cancelModalBtn.addEventListener('click', closeModal);
  
  // 语言切换
  langSelect.addEventListener('change', (e) => {
    currentLang = e.target.value;
    localStorage.setItem('codex_manager_lang', currentLang);
    updateUILanguage();
  });

  // 色彩选择
  colorOptions.forEach(opt => {
    opt.addEventListener('click', () => {
      colorOptions.forEach(o => o.classList.remove('active'));
      opt.classList.add('active');
      selectedColor = opt.dataset.color;
    });
  });

  // 表单提交与重名防呆校验
  profileForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const id = editProfileId.value;
    const name = profileNameInput.value.trim();
    const note = profileNoteInput.value.trim();
    const default_project_path = projectPathInput.value.trim();

    if (!name) return;

    // 重名检查 (对非编辑项)
    const isDuplicate = localProfiles.some(p => p.id !== id && p.name.toLowerCase() === name.toLowerCase());
    if (isDuplicate) {
      alert(t('duplicateNameError'));
      return;
    }

    const payload = {
      id: id || `profile_${Date.now()}`,
      name,
      note,
      color: selectedColor,
      default_project_path: default_project_path || null
    };

    if (invoke) {
      if (id) {
        await invoke('update_profile', { profile: payload });
      } else {
        await invoke('add_profile', { profile: payload });
      }
    } else {
      if (id) {
        const idx = localProfiles.findIndex(p => p.id === id);
        if (idx !== -1) localProfiles[idx] = payload;
      } else {
        localProfiles.push(payload);
      }
    }

    closeModal();
    loadData();
  });

  // 搜索过滤
  searchInput.addEventListener('input', renderProfiles);

  // 一键全停
  stopAllBtn.addEventListener('click', async () => {
    if (confirm(t('confirmStopAll'))) {
      if (invoke) {
        await invoke('stop_all_profiles');
      } else {
        localRunningPids = {};
      }
      fetchRunningStatus();
    }
  });

  // 浏览选择路径 (Tauri dialog)
  browsePathBtn.addEventListener('click', async () => {
    if (invoke) {
      try {
        const dialog = await import('@tauri-apps/plugin-dialog');
        const selected = await dialog.open({ directory: true });
        if (selected) {
          projectPathInput.value = selected;
        }
      } catch (err) {
        console.error("Dialog error:", err);
      }
    }
  });

  // 卡片事件委托：一个监听器处理所有卡片的点击 / 拖拽，替代内联 onclick 与 ondrop
  profileGrid.addEventListener('click', (e) => {
    const btn = e.target.closest('[data-action]');
    if (!btn) return;
    const card = btn.closest('[data-card]');
    const id = card && card.getAttribute('data-profile-id');
    if (!id) return;
    switch (btn.getAttribute('data-action')) {
      case 'launch':  launchProfile(id); break;
      case 'stop':    stopProfile(id); break;
      case 'openDir': openProfileDir(id); break;
      case 'edit':    editProfile(id); break;
      case 'delete':  deleteProfile(id); break;
    }
  });

  const dropTarget = (e) => e.target.closest('[data-card]');
  profileGrid.addEventListener('dragover', (e) => {
    const card = dropTarget(e);
    if (!card) return;
    e.preventDefault();
    card.classList.add('drag-over');
  });
  profileGrid.addEventListener('dragleave', (e) => {
    const card = dropTarget(e);
    if (!card) return;
    card.classList.remove('drag-over');
  });
  profileGrid.addEventListener('drop', (e) => {
    const card = dropTarget(e);
    if (!card) return;
    e.preventDefault();
    card.classList.remove('drag-over');
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const folderPath = e.dataTransfer.files[0].path;
      if (folderPath) {
        launchProfile(card.getAttribute('data-profile-id'), folderPath);
      }
    }
  });
}

async function loadData() {
  if (invoke) {
    try {
      localProfiles = await invoke('get_profiles');
    } catch (e) {
      console.error("Failed to load profiles:", e);
    }
  }
  await fetchRunningStatus();
  renderProfiles();
}

async function fetchRunningStatus() {
  if (invoke) {
    try {
      localRunningPids = await invoke('get_running_status');
    } catch (e) {
      console.error("Failed to fetch running status:", e);
    }
  }
  updateStatusBadge();
  renderProfiles();
}

function updateStatusBadge() {
  const count = Object.keys(localRunningPids).length;
  runningCountText.textContent = t('onlineCount', { count });
  if (count > 0) {
    statusDot.classList.add('active');
    stopAllBtn.classList.remove('hidden');
  } else {
    statusDot.classList.remove('active');
    stopAllBtn.classList.add('hidden');
  }
}

// 线条图标（lucide 风格，与 header 统一，替代 emoji）
const ICON_PLAY = '<svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M8 5.14v14l11-7-11-7z"/></svg>';
const ICON_STOP = '<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><rect x="6" y="6" width="12" height="12" rx="2"/></svg>';
const ICON_FOLDER = '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>';
const ICON_EDIT = '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4z"/></svg>';
const ICON_DELETE = '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg>';

function renderProfiles() {
  const query = searchInput.value.toLowerCase().trim();
  const filtered = localProfiles.filter(p => 
    p.name.toLowerCase().includes(query) || 
    (p.note && p.note.toLowerCase().includes(query))
  );

  if (filtered.length === 0) {
    profileGrid.innerHTML = '';
    emptyState.classList.remove('hidden');
    return;
  }

  emptyState.classList.add('hidden');
  profileGrid.innerHTML = filtered.map(profile => {
    const isRunning = !!localRunningPids[profile.id];
    const pid = localRunningPids[profile.id];

    return `
      <div class="card" data-card data-profile-id="${escapeHtml(profile.id)}">
        <div class="card-header">
          <div class="card-title-group">
            <div class="profile-avatar" style="background-color: ${profile.color}1f; color: ${profile.color}">
              ${escapeHtml(profile.name.charAt(0).toUpperCase())}
            </div>
            <div class="profile-info">
              <h3>${escapeHtml(profile.name)}</h3>
              <p>${escapeHtml(profile.note || '')}</p>
            </div>
          </div>
          <span class="running-tag ${isRunning ? 'online' : 'offline'}">
            ${isRunning ? `● ${t('statusOnline')} <span class="pid">${pid}</span>` : t('statusOffline')}
          </span>
        </div>

        <div class="card-body">
          ${ICON_FOLDER}
          <span title="${escapeHtml(profile.default_project_path || t('defaultPathLabel'))}">
            ${escapeHtml(profile.default_project_path || t('defaultPathLabel'))}
          </span>
        </div>

        <div class="card-actions">
          ${isRunning ? `
            <button class="btn btn-danger-outline btn-stop" data-action="stop">
              ${ICON_STOP}<span>${t('btnStop')}</span>
            </button>
          ` : `
            <button class="btn btn-launch" data-action="launch" style="background:${profile.color}1a;color:${profile.color};border-color:${profile.color}33">
              ${ICON_PLAY}<span>${t('btnLaunch')}</span>
            </button>
          `}

          <button class="btn btn-secondary btn-icon" data-action="openDir" title="${escapeHtml(t('tipOpenFolder'))}" aria-label="${escapeHtml(t('tipOpenFolder'))}">${ICON_FOLDER}</button>
          <button class="btn btn-secondary btn-icon" data-action="edit" title="${escapeHtml(t('tipEdit'))}" aria-label="${escapeHtml(t('tipEdit'))}">${ICON_EDIT}</button>
          <button class="btn btn-secondary btn-icon btn-danger" data-action="delete" title="${escapeHtml(t('tipDelete'))}" aria-label="${escapeHtml(t('tipDelete'))}">${ICON_DELETE}</button>
        </div>
      </div>
    `;
  }).join('');
}

// 卡片操作：不再使用内联 onclick / ondrop，改为事件委托，规避 innerHTML + 内联事件的 XSS 面
async function openProfileDir(id) {
  if (invoke) {
    await invoke('open_profile_dir', { profileId: id });
  }
}

async function launchProfile(id, customPath = null) {
  if (invoke) {
    const ok = await invoke('launch_profile', { profileId: id, projectPath: customPath || null });
    if (!ok) {
      alert(t('codexNotFound'));
    }
  } else {
    localRunningPids[id] = 12345;
  }
  fetchRunningStatus();
}

async function stopProfile(id) {
  if (invoke) {
    await invoke('stop_profile', { profileId: id });
  } else {
    delete localRunningPids[id];
  }
  fetchRunningStatus();
}

function editProfile(id) {
  const profile = localProfiles.find(p => p.id === id);
  if (profile) {
    openModal(profile);
  }
}

async function deleteProfile(id) {
  const profile = localProfiles.find(p => p.id === id);
  const confirmMsg = t('confirmDelete', { name: profile ? profile.name : id });
  if (confirm(confirmMsg)) {
    if (invoke) {
      await invoke('delete_profile', { profileId: id });
    } else {
      localProfiles = localProfiles.filter(p => p.id !== id);
      delete localRunningPids[id];
    }
    loadData();
  }
}

function openModal(profile = null) {
  if (profile) {
    modalTitle.textContent = t('modalEditTitle');
    editProfileId.value = profile.id;
    profileNameInput.value = profile.name;
    profileNoteInput.value = profile.note || "";
    projectPathInput.value = profile.default_project_path || "";
    selectedColor = profile.color || "#007AFF";
  } else {
    modalTitle.textContent = t('modalAddTitle');
    editProfileId.value = "";
    profileNameInput.value = "";
    profileNoteInput.value = "";
    projectPathInput.value = "";
    selectedColor = "#007AFF";
  }

  colorOptions.forEach(o => {
    if (o.dataset.color === selectedColor) {
      o.classList.add('active');
    } else {
      o.classList.remove('active');
    }
  });

  modalOverlay.classList.remove('hidden');
}

function closeModal() {
  modalOverlay.classList.add('hidden');
}

function escapeHtml(str) {
  return str.replace(/[&<>"']/g, function(m) {
    return {
      '&': '&amp;',
      '<': '&lt;',
      '>': '&gt;',
      '"': '&quot;',
      "'": '&#039;'
    }[m];
  });
}
