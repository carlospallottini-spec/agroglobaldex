## AgroGlobalDex v1.1 — Hackathon build (Demo / PoC)

> ⚠️ **AgroGlobalDex es un prototipo en fase de diseño y desarrollo.** No es una plataforma operativa. No constituye oferta de inversión ni servicios sobre criptoactivos. No autorizado actualmente como CASP MiCA. Las cifras y operaciones mostradas son ilustrativas (mock data).

### Cambios respecto a v1.0

- **Renombrado oficial:** AgroChain → **AgroGlobalDex**.
- **Banner Demo/PoC** persistente en todas las páginas.
- **Eliminadas todas las cifras inventadas** (retornos, capital activo, inversores, países).
- **Reescrito el copy** para reflejar el estado pre-MVP real.
- **Equipo:** mostramos solo al fundador real + roles abiertos para hiring (Q3 2026).
- **Footer:** disclaimer explícito de no autorización CASP MiCA.
- **Roadmap regulatorio** explícito (Folleto Art. 1.4 / CASP MiCA / SPV por activo).

### Descarga

- **AgroGlobalDex-Portable.exe** → Doble clic. No requiere instalación. **Recomendado para evaluación.**
- **AgroGlobalDex-Setup.exe** → Instalador con accesos directos en escritorio y menú inicio.

### Requisitos

- Windows 10 / 11 (x64)
- ~75 MB de espacio en disco

### Aviso de Windows SmartScreen

La primera vez que ejecutes el `.exe`, Windows mostrará un aviso de "Editor desconocido" porque el ejecutable **no está firmado digitalmente**. Para ejecutarlo:

1. Pulsa **Más información**
2. Pulsa **Ejecutar de todos modos**

Esto es normal en proyectos no comerciales.

### Contenido

Empaqueta el sitio web completo (5 páginas: Inicio, Nosotros, Marketplace, Equipo, Contacto) dentro de una ventana Electron con Chromium. Compilado con `electron-builder` 25.

### Código fuente

Todo el código fuente del prototipo está en este repositorio bajo licencia MIT. Ver [README.md](../README.md) y [docs/whitepaper-v0.1.md](../docs/whitepaper-v0.1.md) para detalles técnicos y la arquitectura prevista.
