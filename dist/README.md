# dist/ — directorio de deploy de Cloudflare

NO EDITAR A MANO. Este directorio es una copia exacta de `web 2.0/` (la fuente
canónica del sitio). Cloudflare Workers deploya `dist/` (ver wrangler.jsonc).

Tras cambiar algo en `web 2.0/`, sincronizar con:

    rm -rf dist && mkdir dist && cp -r "web 2.0/." dist/

y commitear ambos.
