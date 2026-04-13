// ============================================================
// auto-install-openclaw — Frontend Logic
// ============================================================

const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.shell;

// ------ Navigation ------
function goTo(stepId) {
  document.querySelectorAll('.step').forEach(s => s.classList.remove('active'));
  document.getElementById(stepId).classList.add('active');

  // Trigger step-specific logic
  if (stepId === 'step-copilot') checkCopilot();
  if (stepId === 'step-install') loadDefaults();
}

// ------ GitHub OAuth (Device Flow) ------
let accessToken = null;

async function startOAuth() {
  const btn = document.getElementById('btn-github');
  btn.disabled = true;
  btn.textContent = '연결 중...';

  try {
    // Request device code from backend
    const deviceCode = await invoke('github_device_code');
    
    // Show user code
    document.getElementById('user-code').textContent = deviceCode.user_code;
    document.getElementById('device-code-section').classList.remove('hidden');
    
    // Open browser for user to enter code
    await open(deviceCode.verification_uri);
    
    // Poll for token
    accessToken = await invoke('github_poll_token', { 
      deviceCode: deviceCode.device_code,
      interval: deviceCode.interval 
    });
    
    // Success — move to copilot check
    goTo('step-copilot');
  } catch (err) {
    btn.disabled = false;
    btn.textContent = 'GitHub로 로그인';
    const status = document.getElementById('auth-status');
    status.textContent = `오류: ${err}`;
    status.classList.remove('hidden');
    status.classList.add('error');
  }
}

// ------ Copilot Subscription Check ------
async function checkCopilot() {
  const spinner = document.getElementById('copilot-spinner');
  const statusText = document.getElementById('copilot-status');
  const failSection = document.getElementById('copilot-fail');
  const successSection = document.getElementById('copilot-success');
  
  spinner.classList.remove('hidden');
  statusText.classList.remove('hidden');
  failSection.classList.add('hidden');
  successSection.classList.add('hidden');

  try {
    const hasCopilot = await invoke('check_copilot', { token: accessToken });
    spinner.classList.add('hidden');
    statusText.classList.add('hidden');
    
    if (hasCopilot) {
      successSection.classList.remove('hidden');
    } else {
      failSection.classList.remove('hidden');
    }
  } catch (err) {
    spinner.classList.add('hidden');
    statusText.textContent = `확인 실패: ${err}`;
  }
}

async function recheckCopilot() {
  checkCopilot();
}

// ------ Install Path ------
async function loadDefaults() {
  try {
    const defaultPath = await invoke('get_default_install_path');
    document.getElementById('install-path').value = defaultPath;
  } catch (err) {
    document.getElementById('install-path').value = 'C:\\Program Files\\OpenClaw';
  }
}

async function browsePath() {
  try {
    const { open: dialogOpen } = window.__TAURI__.dialog;
    const selected = await dialogOpen({ directory: true });
    if (selected) {
      document.getElementById('install-path').value = selected;
    }
  } catch (err) {
    console.error('Path selection error:', err);
  }
}

// ------ Installation ------
async function startInstall() {
  goTo('step-progress');
  
  const installPath = document.getElementById('install-path').value;
  const autostart = document.getElementById('autostart').checked;
  const logEl = document.getElementById('progress-log');
  const progressFill = document.getElementById('progress-fill');
  
  const steps = [
    { msg: 'Node.js 확인 중...', cmd: 'install_check_node', pct: 10 },
    { msg: 'Node.js 설치 중...', cmd: 'install_node', pct: 30 },
    { msg: 'OpenClaw 설치 중...', cmd: 'install_openclaw', pct: 60 },
    { msg: '설정 파일 생성 중...', cmd: 'install_config', pct: 80 },
    { msg: 'Gateway 시작 중...', cmd: 'install_start_gateway', pct: 90 },
    { msg: '시작프로그램 등록 중...', cmd: 'install_autostart', pct: 95 },
  ];

  for (const step of steps) {
    addLog(logEl, step.msg, 'active');
    progressFill.style.width = step.pct + '%';
    
    try {
      const result = await invoke(step.cmd, { 
        path: installPath, 
        token: accessToken,
        autostart: autostart 
      });
      updateLastLog(logEl, step.msg.replace('...', ''), 'done', result);
    } catch (err) {
      updateLastLog(logEl, step.msg.replace('...', ''), 'error', `실패: ${err}`);
      // Continue with remaining steps if possible
    }
  }
  
  progressFill.style.width = '100%';
  
  // Small delay before showing complete
  setTimeout(() => goTo('step-complete'), 1000);
}

function addLog(container, message, className) {
  const div = document.createElement('div');
  div.className = `log-item ${className}`;
  const icon = className === 'active' ? '⏳' : className === 'done' ? '✅' : '❌';
  div.textContent = `${icon} ${message}`;
  container.appendChild(div);
  container.scrollTop = container.scrollHeight;
}

function updateLastLog(container, message, className, detail) {
  const items = container.querySelectorAll('.log-item');
  const last = items[items.length - 1];
  if (last) {
    const icon = className === 'done' ? '✅' : '❌';
    last.className = `log-item ${className}`;
    last.textContent = `${icon} ${message} ${detail || '완료'}`;
  }
}

// ------ Post-Install ------
async function openTelegram() {
  await open('https://t.me/BotFather');
}

async function openWebChat() {
  await open('http://localhost:3000');
}

async function closeApp() {
  const { exit } = window.__TAURI__.process;
  await exit(0);
}
