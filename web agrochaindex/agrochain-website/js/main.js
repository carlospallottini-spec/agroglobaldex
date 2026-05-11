/* ═══════════════════════════════════════════════════════════
 * AGROCHAIN — SHARED JS
 * Real Web3 wallet connection (MetaMask, Phantom, Coinbase)
 * No token AGRO — uses BTC, ETH, SOL, USDC, EUR Fiat
 * ═══════════════════════════════════════════════════════════ */

/* ═══════ LIVE TICKER DATA ═══════ */
const TICKER_DATA = [
  {n:'BTC/USD',v:'$68,420',c:'+1.4%',u:1},
  {n:'ETH/USD',v:'$3,842',c:'+1.2%',u:1},
  {n:'SOL/USD',v:'$187.32',c:'+3.8%',u:1},
  {n:'USDC/EUR',v:'€0.92',c:'+0.01%',u:1},
  {n:'EUR/USD',v:'$1.08',c:'-0.2%',u:0},
  {n:'Aceite Olivar ES',v:'€4.20/L',c:'+2.1%',u:1},
  {n:'Vino Rioja',v:'€8.50/kg',c:'+0.8%',u:1},
  {n:'Queso Manchego',v:'€18.5/kg',c:'+1.5%',u:1},
  {n:'Café Colombia',v:'$5.85/kg',c:'+4.2%',u:1},
  {n:'Trigo EU',v:'€245/T',c:'-0.3%',u:0},
  {n:'Soja BR',v:'$478/T',c:'+0.9%',u:1},
  {n:'Cacao',v:'$3,180/T',c:'+5.3%',u:1},
  {n:'Vol 24h',v:'$2.4M',c:'+12%',u:1},
  {n:'Productores',v:'2.4K',c:'+8%',u:1},
  {n:'Países',v:'14',c:'LIVE',u:1},
];

/* ═══════ TICKER RENDERER ═══════ */
function renderTicker() {
  const el = document.getElementById('ticker');
  if (!el) return;
  const html = TICKER_DATA.map(d =>
    `<span class="ti"><span class="ti-live"></span><span class="tn2">${d.n}</span><span class="tv">${d.v}</span><span class="${d.u?'tup':'tdn'}">${d.c}</span></span>`
  ).join('');
  el.innerHTML = html + html + html;
}

/* ═══════ WALLET MANAGER ═══════ */
const Wallet = {
  address: null,
  type: null,
  chain: null,
  balance: null,

  init() {
    const saved = localStorage.getItem('agrochain_wallet');
    if (saved) {
      try {
        const data = JSON.parse(saved);
        this.address = data.address;
        this.type = data.type;
        this.chain = data.chain;
        this.updateUI();
      } catch (e) {}
    }
    // Listen for MetaMask account changes
    if (window.ethereum) {
      window.ethereum.on('accountsChanged', (accounts) => {
        if (accounts.length === 0) this.disconnect();
        else {
          this.address = accounts[0];
          this.save();
          this.updateUI();
          showToast('Cuenta cambiada: ' + this.short(), 'success');
        }
      });
      window.ethereum.on('chainChanged', () => window.location.reload());
    }
    // Listen for Phantom disconnect
    if (window.phantom?.solana) {
      window.phantom.solana.on('disconnect', () => this.disconnect());
    }
  },

  /* REAL MetaMask connection (Ethereum) */
  async connectMetaMask() {
    if (typeof window.ethereum === 'undefined' || !window.ethereum.isMetaMask) {
      showToast('MetaMask no detectado. Instálalo desde metamask.io', 'error');
      setTimeout(() => window.open('https://metamask.io/download/', '_blank', 'noopener'), 1500);
      return false;
    }
    try {
      const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
      if (!accounts || accounts.length === 0) throw new Error('Sin cuentas');
      this.address = accounts[0];
      this.type = 'metamask';
      this.chain = 'ethereum';
      // Fetch balance
      const balWei = await window.ethereum.request({
        method: 'eth_getBalance',
        params: [this.address, 'latest']
      });
      this.balance = (parseInt(balWei, 16) / 1e18).toFixed(4) + ' ETH';
      this.save();
      this.updateUI();
      showToast('✓ MetaMask conectada: ' + this.short(), 'success');
      return true;
    } catch (err) {
      if (err.code === 4001) showToast('Conexión rechazada', 'error');
      else showToast('Error: ' + (err.message || 'desconocido'), 'error');
      return false;
    }
  },

  /* REAL Phantom connection (Solana) */
  async connectPhantom() {
    const provider = window.phantom?.solana;
    if (!provider?.isPhantom) {
      showToast('Phantom no detectado. Instálalo desde phantom.app', 'error');
      setTimeout(() => window.open('https://phantom.app/download', '_blank', 'noopener'), 1500);
      return false;
    }
    try {
      const resp = await provider.connect();
      this.address = resp.publicKey.toString();
      this.type = 'phantom';
      this.chain = 'solana';
      this.balance = '— SOL';
      this.save();
      this.updateUI();
      showToast('✓ Phantom conectada: ' + this.short(), 'success');
      // Fetch SOL balance
      try {
        const r = await fetch('https://api.mainnet-beta.solana.com', {
          method: 'POST',
          headers: {'Content-Type': 'application/json'},
          body: JSON.stringify({
            jsonrpc: '2.0', id: 1, method: 'getBalance',
            params: [this.address]
          })
        });
        const j = await r.json();
        if (j.result?.value !== undefined) {
          this.balance = (j.result.value / 1e9).toFixed(4) + ' SOL';
          this.save();
          this.updateUI();
        }
      } catch(e) {}
      return true;
    } catch (err) {
      showToast('Conexión rechazada', 'error');
      return false;
    }
  },

  /* REAL Coinbase Wallet (via ethereum provider) */
  async connectCoinbase() {
    if (typeof window.ethereum === 'undefined') {
      showToast('Coinbase Wallet no detectado', 'error');
      setTimeout(() => window.open('https://www.coinbase.com/wallet/downloads', '_blank', 'noopener'), 1500);
      return false;
    }
    try {
      let provider = window.ethereum;
      if (window.ethereum.providers) {
        provider = window.ethereum.providers.find(p => p.isCoinbaseWallet) || window.ethereum;
      }
      const accounts = await provider.request({ method: 'eth_requestAccounts' });
      this.address = accounts[0];
      this.type = 'coinbase';
      this.chain = 'ethereum';
      this.save();
      this.updateUI();
      showToast('✓ Coinbase conectada: ' + this.short(), 'success');
      return true;
    } catch (err) {
      showToast('Conexión rechazada', 'error');
      return false;
    }
  },

  /* WalletConnect placeholder (requires @walletconnect/ethereum-provider SDK) */
  connectWalletConnect() {
    showToast('WalletConnect requiere SDK en producción. Usa MetaMask o Phantom.', 'error');
  },

  disconnect() {
    this.address = null;
    this.type = null;
    this.chain = null;
    this.balance = null;
    localStorage.removeItem('agrochain_wallet');
    this.updateUI();
    if (window.phantom?.solana?.isConnected) {
      try { window.phantom.solana.disconnect(); } catch(e) {}
    }
    showToast('Wallet desconectada', 'success');
  },

  save() {
    localStorage.setItem('agrochain_wallet', JSON.stringify({
      address: this.address, type: this.type, chain: this.chain
    }));
  },

  short() {
    if (!this.address) return '';
    return this.address.slice(0, 6) + '...' + this.address.slice(-4);
  },

  updateUI() {
    document.querySelectorAll('.wallet-btn, .nwb').forEach(btn => {
      if (this.address) {
        btn.classList.add('connected');
        btn.innerHTML = `<div class="wdot"></div><svg viewBox="0 0 24 24"><rect x="2" y="5" width="20" height="14" rx="2"/><line x1="2" y1="10" x2="22" y2="10"/></svg><span>${this.short()}</span>`;
      } else {
        btn.classList.remove('connected');
        btn.innerHTML = `<div class="wdot"></div><svg viewBox="0 0 24 24"><rect x="2" y="5" width="20" height="14" rx="2"/><line x1="2" y1="10" x2="22" y2="10"/></svg><span>Conectar Wallet</span>`;
      }
    });
    // Update modal if open
    const modal = document.getElementById('wallet-modal');
    if (modal) {
      const discon = modal.querySelector('.wp-connected');
      const opts = modal.querySelector('.wp-options');
      if (this.address) {
        if (opts) opts.style.display = 'none';
        if (discon) {
          discon.classList.add('show');
          const valEl = discon.querySelector('.wp-balance-val');
          const subEl = discon.querySelector('.wp-balance-sub');
          if (valEl) valEl.textContent = this.balance || '—';
          if (subEl) subEl.textContent = this.address;
        }
      } else {
        if (opts) opts.style.display = 'block';
        if (discon) discon.classList.remove('show');
      }
    }
  }
};

/* ═══════ WALLET MODAL ═══════ */
function openWalletModal() {
  let modal = document.getElementById('wallet-modal');
  if (!modal) {
    const html = `
      <div class="modal-overlay" id="wallet-modal">
        <div class="modal">
          <button class="modal-close" aria-label="Cerrar">✕</button>
          <h3>${Wallet.address ? 'Tu cuenta' : 'Conectar Wallet'}</h3>
          <p>${Wallet.address ? 'Gestiona tu wallet conectada.' : 'Conecta tu wallet para acceder a AgroChain. Soportamos las principales wallets Web3. Nunca almacenamos tus claves privadas.'}</p>

          <div class="wp-options">
            <div class="wallet-option" data-w="metamask">
              <svg viewBox="0 0 40 40"><polygon points="36,4 22.3,14.2 24.8,8" fill="#E17726"/><polygon points="4,4 17.6,14.3 15.3,8" fill="#E27625"/><polygon points="30.9,27.5 27.2,33.2 35.2,35.4 37.5,27.6" fill="#E27625"/><polygon points="2.6,27.6 4.8,35.4 12.8,33.2 9.1,27.5" fill="#E27625"/><polygon points="12.4,17.5 10.2,20.8 18,21.2 17.7,12.8" fill="#E27625"/><polygon points="27.6,17.5 22.3,12.7 22,21.2 29.8,20.8" fill="#E27625"/></svg>
              <div class="wo-info"><div class="wo-name">MetaMask</div><div class="wo-sub">Ethereum · ERC-20 · EVM</div></div>
              <div class="wo-arr">→</div>
            </div>
            <div class="wallet-option" data-w="phantom">
              <svg viewBox="0 0 40 40"><circle cx="20" cy="20" r="19" fill="#AB9FF2"/><path d="M29 16c0-4.4-3.6-8-9-8s-9 3.6-9 8c0 4.2 3.4 7.5 7.5 7.5.7 0 1.4-.1 2-.3 3.5-1 6.5-4.2 6.5-7.2z" fill="white"/><ellipse cx="15.5" cy="16.5" rx="1.5" ry="2" fill="#AB9FF2"/><ellipse cx="21.5" cy="16.5" rx="1.5" ry="2" fill="#AB9FF2"/></svg>
              <div class="wo-info"><div class="wo-name">Phantom</div><div class="wo-sub">Solana · SPL Tokens · Recomendado</div></div>
              <div class="wo-arr">→</div>
            </div>
            <div class="wallet-option" data-w="coinbase">
              <svg viewBox="0 0 40 40"><circle cx="20" cy="20" r="19" fill="#0052FF"/><rect x="14" y="14" width="12" height="12" rx="2" fill="white"/></svg>
              <div class="wo-info"><div class="wo-name">Coinbase Wallet</div><div class="wo-sub">Multi-chain</div></div>
              <div class="wo-arr">→</div>
            </div>
            <div class="wallet-option" data-w="walletconnect">
              <svg viewBox="0 0 40 40"><circle cx="20" cy="20" r="19" fill="#3B99FC"/><path d="M11.5 16c4.7-4.6 12.3-4.6 17 0l.6.6c.2.2.2.6 0 .8l-2 2c-.1.1-.3.1-.4 0l-.8-.8c-3.3-3.2-8.6-3.2-11.9 0l-.9.8c-.1.1-.3.1-.4 0l-2-2c-.2-.2-.2-.6 0-.8l.8-.6z" fill="white"/></svg>
              <div class="wo-info"><div class="wo-name">WalletConnect</div><div class="wo-sub">300+ wallets compatibles</div></div>
              <div class="wo-arr">→</div>
            </div>
            <p class="wp-note">Al conectar aceptas nuestros <a href="#">Términos</a> y <a href="#">Política de Privacidad</a>.</p>
          </div>

          <div class="wp-connected">
            <div class="wp-balance">
              <div class="wp-balance-label">Balance on-chain</div>
              <div class="wp-balance-val">—</div>
              <div class="wp-balance-sub"></div>
            </div>
            <div class="wp-actions">
              <button class="wp-act" data-act="copy">Copiar</button>
              <button class="wp-act" data-act="explorer">Explorer</button>
            </div>
            <button class="wp-disconnect">Desconectar wallet</button>
          </div>
        </div>
      </div>`;
    document.body.insertAdjacentHTML('beforeend', html);
    modal = document.getElementById('wallet-modal');

    modal.addEventListener('click', (e) => {
      if (e.target === modal) closeWalletModal();
    });
    modal.querySelector('.modal-close').addEventListener('click', closeWalletModal);

    modal.querySelectorAll('.wallet-option').forEach(opt => {
      opt.addEventListener('click', async () => {
        const w = opt.dataset.w;
        let ok = false;
        if (w === 'metamask') ok = await Wallet.connectMetaMask();
        else if (w === 'phantom') ok = await Wallet.connectPhantom();
        else if (w === 'coinbase') ok = await Wallet.connectCoinbase();
        else if (w === 'walletconnect') Wallet.connectWalletConnect();
        if (ok) closeWalletModal();
      });
    });

    modal.querySelector('.wp-disconnect').addEventListener('click', () => {
      Wallet.disconnect();
      closeWalletModal();
    });

    modal.querySelectorAll('.wp-act').forEach(b => {
      b.addEventListener('click', () => {
        if (b.dataset.act === 'copy' && Wallet.address) {
          navigator.clipboard.writeText(Wallet.address).then(() =>
            showToast('Dirección copiada al portapapeles', 'success'));
        } else if (b.dataset.act === 'explorer' && Wallet.address) {
          const url = Wallet.chain === 'solana'
            ? `https://solscan.io/account/${Wallet.address}`
            : `https://etherscan.io/address/${Wallet.address}`;
          window.open(url, '_blank', 'noopener');
        }
      });
    });
  }
  Wallet.updateUI();
  modal.classList.add('active');
  document.body.style.overflow = 'hidden';
}

function closeWalletModal() {
  const modal = document.getElementById('wallet-modal');
  if (modal) modal.classList.remove('active');
  document.body.style.overflow = '';
}

/* ═══════ TOAST ═══════ */
function showToast(msg, type = 'success') {
  let toast = document.getElementById('toast');
  if (!toast) {
    toast = document.createElement('div');
    toast.id = 'toast';
    toast.className = 'toast';
    document.body.appendChild(toast);
  }
  toast.className = `toast ${type}`;
  toast.innerHTML = `<div class="toast-ico">${type==='success'?'✓':'!'}</div><div class="toast-txt">${msg}</div>`;
  requestAnimationFrame(() => toast.classList.add('show'));
  clearTimeout(window._toastTimer);
  window._toastTimer = setTimeout(() => toast.classList.remove('show'), 3200);
}

/* ═══════ MOBILE MENU ═══════ */
function initMobileMenu() {
  const burger = document.getElementById('burger');
  const menu = document.getElementById('mob-menu');
  const overlay = document.getElementById('mob-overlay');
  if (!burger || !menu || !overlay) return;
  const close = () => {
    burger.classList.remove('open');
    menu.classList.remove('open');
    overlay.classList.remove('open');
    document.body.style.overflow = '';
  };
  burger.addEventListener('click', () => {
    const isOpen = burger.classList.toggle('open');
    menu.classList.toggle('open');
    overlay.classList.toggle('open');
    document.body.style.overflow = isOpen ? 'hidden' : '';
  });
  overlay.addEventListener('click', close);
  menu.querySelectorAll('a').forEach(a => a.addEventListener('click', close));
}

/* ═══════ REVEAL ON SCROLL ═══════ */
function initReveal() {
  const obs = new IntersectionObserver((entries) => {
    entries.forEach(e => {
      if (e.isIntersecting) {
        e.target.classList.add('vis');
        obs.unobserve(e.target);
      }
    });
  }, { threshold: 0.12, rootMargin: '0px 0px -40px 0px' });
  document.querySelectorAll('.rv').forEach(el => obs.observe(el));
}

/* ═══════ ACTIVE NAV ═══════ */
function setActiveNav() {
  const current = location.pathname.split('/').pop() || 'index.html';
  document.querySelectorAll('.nav-links a, .mobile-menu a').forEach(a => {
    const href = a.getAttribute('href');
    if (href === current || (current === '' && href === 'index.html')) {
      a.classList.add('active');
    }
  });
}

/* ═══════ INIT ═══════ */
document.addEventListener('DOMContentLoaded', () => {
  renderTicker();
  Wallet.init();
  initMobileMenu();
  initReveal();
  setActiveNav();
  // Bind all wallet buttons
  document.querySelectorAll('.wallet-btn, .nwb').forEach(btn => {
    btn.addEventListener('click', openWalletModal);
  });
});
