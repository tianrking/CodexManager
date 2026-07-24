import { invoke as tauriInvoke } from '@tauri-apps/api/core';

let invoke = tauriInvoke;

// 本地 Mock 数据 (仅用于离线/非 Tauri 预演)
let localProfiles = [
  {
    id: "work_account",
    name: "Work Account",
    note: "工作/公司项目 Profile (凭据+环境彻底隔离)",
    color: "#007AFF",
    default_project_path: ""
  },
  {
    id: "personal_account",
    name: "Personal Account",
    note: "个人开源与私有项目 (独立 Session)",
    color: "#34C759",
    default_project_path: ""
  }
];
let localRunningPids = {};

// DOM 节点引用
const profileGrid = document.getElementById('profileGrid');
const emptyState = document.getElementById('emptyState');
const statusDot = document.getElementById('statusDot');
const runningCountText = document.getElementById('runningCountText');
const stopAllBtn = document.getElementById('stopAllBtn');
const searchInput = document.getElementById('searchInput');

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

let selectedColor = '#007AFF';

// 初始化
document.addEventListener('DOMContentLoaded', () => {
  initEventListeners();
  loadData();
  setInterval(fetchRunningStatus, 1500);
});

function initEventListeners() {
  addProfileBtn.addEventListener('click', () => openModal());
  closeModalBtn.addEventListener('click', closeModal);
  cancelModalBtn.addEventListener('click', closeModal);
  
  // 色彩选择
  colorOptions.forEach(opt => {
    opt.addEventListener('click', () => {
      colorOptions.forEach(o => o.classList.remove('active'));
      opt.classList.add('active');
      selectedColor = opt.dataset.color;
    });
  });

  // 表单提交
  profileForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const id = editProfileId.value;
    const name = profileNameInput.value.trim();
    const note = profileNoteInput.value.trim();
    const default_project_path = projectPathInput.value.trim();

    if (!name) return;

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
    if (invoke) {
      await invoke('stop_all_profiles');
    } else {
      localRunningPids = {};
    }
    fetchRunningStatus();
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
  runningCountText.textContent = `${count} 个在线中`;
  if (count > 0) {
    statusDot.classList.add('active');
    stopAllBtn.classList.remove('hidden');
  } else {
    statusDot.classList.remove('active');
    stopAllBtn.classList.add('hidden');
  }
}

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
      <div class="card" id="card-${profile.id}" 
           ondragover="handleDragOver(event)" 
           ondragleave="handleDragLeave(event)" 
           ondrop="handleDrop(event, '${profile.id}')">
        <div class="card-header">
          <div class="card-title-group">
            <div class="profile-avatar" style="background-color: ${profile.color}20; color: ${profile.color}">
              ${profile.name.charAt(0).toUpperCase()}
            </div>
            <div class="profile-info">
              <h3>${escapeHtml(profile.name)}</h3>
              <p>${escapeHtml(profile.note || '物理隔离环境已激活')}</p>
            </div>
          </div>
          <span class="running-tag ${isRunning ? 'online' : 'offline'}">
            ${isRunning ? `● 在线 (PID: ${pid})` : '○ 离线'}
          </span>
        </div>

        <div class="card-body">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
          <span title="${escapeHtml(profile.default_project_path || '拖拽项目文件夹到此卡片亦可直接调起')}">
            ${escapeHtml(profile.default_project_path || '默认主页目录 (或拖拽文件夹启动)')}
          </span>
        </div>

        <div class="card-actions">
          ${isRunning ? `
            <button class="btn btn-secondary btn-launch" onclick="stopProfile('${profile.id}')" style="color: var(--accent-red); background: rgba(248, 113, 113, 0.15)" title="精准强行杀死该账号及全部 Helper 子进程">
              ■ 停止运行
            </button>
          ` : `
            <button class="btn btn-primary btn-launch" onclick="launchProfile('${profile.id}')" style="background: ${profile.color}" title="独立多开唤起进程">
              ▶ 独立启动
            </button>
          `}

          <button class="btn btn-secondary" onclick="openProfileDir('${profile.id}')" title="在 Finder / 资源管理器中查看此 Profile 凭据目录">
            📂
          </button>
          
          <button class="btn btn-secondary" onclick="editProfile('${profile.id}')" title="编辑 Profile">
            ✏️
          </button>
          <button class="btn btn-secondary" onclick="deleteProfile('${profile.id}')" title="删除 Profile" style="color: var(--accent-red)">
            🗑️
          </button>
        </div>
      </div>
    `;
  }).join('');
}

// 拖拽打开文件夹功能
window.handleDragOver = (e) => {
  e.preventDefault();
  e.currentTarget.classList.add('drag-over');
};

window.handleDragLeave = (e) => {
  e.currentTarget.classList.remove('drag-over');
};

window.handleDrop = (e, profileId) => {
  e.preventDefault();
  e.currentTarget.classList.remove('drag-over');
  if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
    const folderPath = e.dataTransfer.files[0].path;
    if (folderPath) {
      launchProfile(profileId, folderPath);
    }
  }
};

window.openProfileDir = async (id) => {
  if (invoke) {
    await invoke('open_profile_dir', { profileId: id });
  }
};

window.launchProfile = async (id, customPath = null) => {
  if (invoke) {
    await invoke('launch_profile', { profileId: id, projectPath: customPath || null });
  } else {
    localRunningPids[id] = 12345;
  }
  fetchRunningStatus();
};

window.stopProfile = async (id) => {
  if (invoke) {
    await invoke('stop_profile', { profileId: id });
  } else {
    delete localRunningPids[id];
  }
  fetchRunningStatus();
};

window.editProfile = (id) => {
  const profile = localProfiles.find(p => p.id === id);
  if (profile) {
    openModal(profile);
  }
};

window.deleteProfile = async (id) => {
  if (confirm("确认注销该 Profile 账号？专属凭据隔离物理目录也将被注销。")) {
    if (invoke) {
      await invoke('delete_profile', { profileId: id });
    } else {
      localProfiles = localProfiles.filter(p => p.id !== id);
      delete localRunningPids[id];
    }
    loadData();
  }
};

function openModal(profile = null) {
  if (profile) {
    modalTitle.textContent = "编辑 Codex 账号 Profile";
    editProfileId.value = profile.id;
    profileNameInput.value = profile.name;
    profileNoteInput.value = profile.note || "";
    projectPathInput.value = profile.default_project_path || "";
    selectedColor = profile.color || "#007AFF";
  } else {
    modalTitle.textContent = "新增 Codex 隔离账号 Profile";
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
