// AgroGlobalDex — Electron main process
// Hardened: single-instance, secure defaults, native menu with shortcuts,
// external links open in default browser, About dialog, window state persisted.

const { app, BrowserWindow, Menu, shell, dialog, ipcMain, nativeImage } = require('electron');
const path = require('path');
const fs = require('fs');

// ─── Single-instance lock ──────────────────────────────────────────────
const gotTheLock = app.requestSingleInstanceLock();
if (!gotTheLock) {
  app.quit();
  return;
}

let mainWindow = null;
const APP_NAME = 'AgroGlobalDex';
const APP_VERSION = require('./package.json').version;

// ─── Window state persistence ──────────────────────────────────────────
function stateFile() {
  return path.join(app.getPath('userData'), 'window-state.json');
}
function loadWindowState() {
  try {
    const raw = fs.readFileSync(stateFile(), 'utf8');
    return JSON.parse(raw);
  } catch (_) {
    return { width: 1400, height: 900, x: undefined, y: undefined, maximized: false };
  }
}
function saveWindowState(win) {
  if (!win || win.isDestroyed()) return;
  const bounds = win.getBounds();
  const state = {
    ...bounds,
    maximized: win.isMaximized(),
  };
  try {
    fs.mkdirSync(path.dirname(stateFile()), { recursive: true });
    fs.writeFileSync(stateFile(), JSON.stringify(state));
  } catch (_) {}
}

// ─── Create main window ────────────────────────────────────────────────
function createWindow() {
  const saved = loadWindowState();

  mainWindow = new BrowserWindow({
    width: saved.width || 1400,
    height: saved.height || 900,
    x: saved.x,
    y: saved.y,
    minWidth: 960,
    minHeight: 600,
    title: APP_NAME,
    icon: path.join(__dirname, 'build', 'icon.ico'),
    backgroundColor: '#05080A',
    show: false, // mostramos cuando esté ready-to-show
    autoHideMenuBar: false, // queremos el menú nativo accesible
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      webSecurity: true,
      allowRunningInsecureContent: false,
      experimentalFeatures: false,
      // SW se rompe en file:// → lo deshabilitamos en Electron.
      // El SW está condicionalmente skipped en js/pwa-install.js también.
      serviceWorkers: false,
      partition: 'persist:agroglobaldex', // partition propio para limpiar caches
    },
  });

  // Limpiar caches viejos al arrancar (por si el SW había quedado cacheado de v1)
  mainWindow.webContents.session.clearStorageData({
    storages: ['serviceworkers', 'cachestorage'],
  }).catch(() => {});

  if (saved.maximized) mainWindow.maximize();

  mainWindow.loadFile(path.join(__dirname, 'src', 'index.html'));

  // Mostrar solo cuando el contenido está listo (evita flash blanco)
  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
    mainWindow.focus();
  });

  // Persistir estado de ventana
  const persistBounds = () => saveWindowState(mainWindow);
  mainWindow.on('resize', persistBounds);
  mainWindow.on('move', persistBounds);
  mainWindow.on('maximize', persistBounds);
  mainWindow.on('unmaximize', persistBounds);

  // External links → navegador del sistema (no abrir ventanas Electron)
  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith('http://') || url.startsWith('https://') || url.startsWith('mailto:')) {
      shell.openExternal(url);
      return { action: 'deny' };
    }
    return { action: 'allow' };
  });

  // Prevenir navegación fuera de la app (security)
  mainWindow.webContents.on('will-navigate', (event, url) => {
    const allowedFile = url.startsWith('file://');
    const allowedLocal = url.startsWith('http://localhost') || url.startsWith('http://127.0.0.1');
    if (!allowedFile && !allowedLocal) {
      event.preventDefault();
      shell.openExternal(url);
    }
  });

  mainWindow.on('closed', () => { mainWindow = null; });
}

// ─── Native menu ───────────────────────────────────────────────────────
function buildMenu() {
  const isMac = process.platform === 'darwin';
  const accel = isMac ? 'Cmd' : 'Ctrl';

  const template = [
    ...(isMac ? [{
      label: APP_NAME,
      submenu: [
        { role: 'about' },
        { type: 'separator' },
        { role: 'services' },
        { type: 'separator' },
        { role: 'hide' },
        { role: 'hideOthers' },
        { role: 'unhide' },
        { type: 'separator' },
        { role: 'quit' },
      ],
    }] : []),
    {
      label: 'Archivo',
      submenu: [
        {
          label: 'Inicio',
          accelerator: `${accel}+H`,
          click: () => mainWindow?.loadFile(path.join(__dirname, 'src', 'index.html')),
        },
        {
          label: 'Marketplace',
          accelerator: `${accel}+M`,
          click: () => mainWindow?.loadFile(path.join(__dirname, 'src', 'marketplace.html')),
        },
        {
          label: 'Tokenizar',
          accelerator: `${accel}+T`,
          click: () => mainWindow?.loadFile(path.join(__dirname, 'src', 'tokenize.html')),
        },
        {
          label: 'Invertir',
          accelerator: `${accel}+I`,
          click: () => mainWindow?.loadFile(path.join(__dirname, 'src', 'invest.html')),
        },
        { type: 'separator' },
        isMac ? { role: 'close' } : { role: 'quit', label: 'Salir' },
      ],
    },
    {
      label: 'Editar',
      submenu: [
        { role: 'undo', label: 'Deshacer' },
        { role: 'redo', label: 'Rehacer' },
        { type: 'separator' },
        { role: 'cut', label: 'Cortar' },
        { role: 'copy', label: 'Copiar' },
        { role: 'paste', label: 'Pegar' },
        { role: 'selectAll', label: 'Seleccionar todo' },
      ],
    },
    {
      label: 'Ver',
      submenu: [
        { role: 'reload', label: 'Recargar', accelerator: `${accel}+R` },
        { role: 'forceReload', label: 'Recargar forzado', accelerator: `${accel}+Shift+R` },
        { type: 'separator' },
        { role: 'resetZoom', label: 'Zoom 100%', accelerator: `${accel}+0` },
        { role: 'zoomIn', label: 'Zoom in', accelerator: `${accel}+Plus` },
        { role: 'zoomOut', label: 'Zoom out', accelerator: `${accel}+-` },
        { type: 'separator' },
        { role: 'togglefullscreen', label: 'Pantalla completa' },
        { type: 'separator' },
        { role: 'toggleDevTools', label: 'DevTools (debug)', accelerator: `${accel}+Shift+I` },
      ],
    },
    {
      label: 'Ventana',
      submenu: [
        { role: 'minimize', label: 'Minimizar' },
        { role: 'zoom', label: 'Maximizar' },
        ...(isMac ? [
          { type: 'separator' },
          { role: 'front' },
        ] : []),
      ],
    },
    {
      label: 'Ayuda',
      submenu: [
        {
          label: 'Página web',
          click: () => shell.openExternal('https://github.com/carlospallottini-spec/agroglobaldex'),
        },
        {
          label: 'Documentación',
          click: () => shell.openExternal('https://github.com/carlospallottini-spec/agroglobaldex#readme'),
        },
        {
          label: 'Reportar un bug',
          click: () => shell.openExternal('https://github.com/carlospallottini-spec/agroglobaldex/issues/new'),
        },
        { type: 'separator' },
        {
          label: `Acerca de ${APP_NAME}`,
          click: () => {
            dialog.showMessageBox(mainWindow, {
              type: 'info',
              title: `Acerca de ${APP_NAME}`,
              message: `${APP_NAME} v${APP_VERSION}`,
              detail: 'Marketplace global de RWA agropecuarios tokenizados sobre Solana.\n\n' +
                      'Demo / Proof of Concept · MiCA-first design\n' +
                      'No constituye oferta de inversión.\n\n' +
                      '© 2026 Carlos Pallottini · MIT License',
              buttons: ['OK', 'Sitio web'],
              defaultId: 0,
              cancelId: 0,
              icon: nativeImage.createFromPath(path.join(__dirname, 'build', 'icon.png')),
            }).then((res) => {
              if (res.response === 1) {
                shell.openExternal('https://github.com/carlospallottini-spec/agroglobaldex');
              }
            });
          },
        },
      ],
    },
  ];

  Menu.setApplicationMenu(Menu.buildFromTemplate(template));
}

// ─── App lifecycle ─────────────────────────────────────────────────────
app.on('second-instance', () => {
  if (mainWindow) {
    if (mainWindow.isMinimized()) mainWindow.restore();
    mainWindow.focus();
  }
});

app.whenReady().then(async () => {
  // CRÍTICO: limpiar service workers y caches de instalaciones previas
  // ANTES de crear la ventana. Sin esto, el SW de v2.0.0/2.0.1 puede
  // seguir interceptando fetch y dejar la pantalla en negro al navegar.
  try {
    const { session } = require('electron');
    await session.defaultSession.clearStorageData({
      storages: ['serviceworkers', 'cachestorage', 'shadercache'],
    });
    await session.defaultSession.clearCache();
    // También la partition propia (por si quedó algo)
    const part = session.fromPartition('persist:agroglobaldex');
    await part.clearStorageData({
      storages: ['serviceworkers', 'cachestorage', 'shadercache'],
    }).catch(() => {});
    await part.clearCache().catch(() => {});
  } catch (e) {}

  buildMenu();
  createWindow();
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) createWindow();
});

// Security: deny all permission requests by default; only allow what we explicitly need.
app.on('web-contents-created', (event, contents) => {
  contents.session.setPermissionRequestHandler((webContents, permission, callback) => {
    // Solana wallets need clipboard / notifications. Allow only safe ones.
    const allowed = ['clipboard-read', 'clipboard-sanitized-write', 'notifications'];
    callback(allowed.includes(permission));
  });
});
