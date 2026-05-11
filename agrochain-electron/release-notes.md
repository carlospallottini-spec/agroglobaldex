## AgroChain v1.0 — Hackathon build

Aplicación de escritorio del marketplace global de activos agropecuarios tokenizados.

### Descarga

- **AgroChain-Portable.exe** → No requiere instalación. Doble clic y abre.
- **AgroChain-Setup.exe** → Instalador con accesos directos en escritorio y menú inicio.

### Requisitos

- Windows 10 / 11 (x64)
- ~75 MB de espacio en disco

### Aviso de Windows SmartScreen

La primera vez que ejecutes el `.exe`, Windows mostrará un aviso de "Editor desconocido" porque el ejecutable no está firmado digitalmente (la firma comercial cuesta ~300 €/año y se omite en proyectos no comerciales).

Para ejecutarlo:

1. Pulsa **Más información**
2. Pulsa **Ejecutar de todos modos**

### Contenido

Empaqueta el sitio web completo (5 páginas: Inicio, Nosotros, Marketplace, Equipo, Contacto) dentro de una ventana Electron con Chromium. Compilado con `electron-builder` 25.

### Código fuente

Todo el código fuente está en este mismo repositorio. Ver [README.md](../README.md) para detalles técnicos.
