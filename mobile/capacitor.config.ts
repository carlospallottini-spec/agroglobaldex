import type { CapacitorConfig } from '@capacitor/cli';

/**
 * AgroGlobalDex — configuración Capacitor de PRODUCCIÓN.
 *
 * Modo "remote wrapper": la app carga el sitio ya deployado en
 * https://agroglobaldex.com dentro del WebView nativo. Así la app se
 * actualiza sola cada vez que se actualiza la web, sin re-publicar en
 * las stores (solo cambios de UI web; cambios nativos sí requieren
 * nueva versión en la store).
 *
 * `webDir: 'www'` contiene únicamente la página de fallback offline
 * ("Sin conexión"), que el WebView muestra vía `errorPath` cuando no
 * hay internet o el sitio no responde.
 */
const config: CapacitorConfig = {
  // ⚠️ NO CAMBIAR appId después de publicar: identifica la app en las
  // stores para siempre (Play Store no permite cambiarlo jamás).
  appId: 'io.agroglobaldex.app',
  appName: 'AgroGlobalDex',
  webDir: 'www',

  server: {
    // Sitio en producción. Sin cleartext: SOLO https.
    url: 'https://agroglobaldex.com',
    androidScheme: 'https',
    iosScheme: 'https',
    cleartext: false,
    // Página local que se muestra si el sitio no carga (sin conexión).
    errorPath: 'index.html',
    // Navegación PERMITIDA dentro del WebView: únicamente dominios
    // propios. Cualquier otro link (Solscan, Phantom, redes sociales,
    // docs…) se abre en el navegador / app externa del sistema, que es
    // el comportamiento correcto: las wallets móviles (Phantom,
    // Solflare) se conectan por deep-link hacia su propia app.
    allowNavigation: [
      'agroglobaldex.com',
      '*.agroglobaldex.com',
    ],
  },

  plugins: {
    SplashScreen: {
      launchShowDuration: 1200,
      launchAutoHide: true,
      backgroundColor: '#05080A', // token --bg de web 2.0/css/tokens.css
      androidScaleType: 'CENTER_CROP',
      showSpinner: false,
      splashFullScreen: true,
      splashImmersive: true,
    },
    StatusBar: {
      style: 'DARK', // fondo oscuro → iconos claros
      backgroundColor: '#05080A',
      overlaysWebView: false,
    },
    Keyboard: {
      resize: 'body',
      style: 'DARK',
      resizeOnFullScreen: true,
    },
  },

  android: {
    allowMixedContent: false,         // nada de contenido http en páginas https
    captureInput: true,
    webContentsDebuggingEnabled: false, // sin debugging remoto en producción
  },

  ios: {
    contentInset: 'automatic',
    limitsNavigationsToAppBoundDomains: false,
  },
};

export default config;
