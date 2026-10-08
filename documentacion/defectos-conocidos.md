# Defectos conocidos

Esta versión no tiene ningún defecto abierto que afecte al uso normal del programa. Las
correcciones de cada versión están en el [historial de cambios](historial-de-cambios.md).

## Limitaciones del aviso de versión nueva

**No usa el proxy configurado en las opciones de internet de Windows.** Sí respeta el proxy declarado
en las variables de entorno `HTTPS_PROXY`, `HTTP_PROXY` y `ALL_PROXY` (y las excepciones de
`NO_PROXY`), pero no lee el del registro de Windows, ni archivos de configuración automática
(PAC/WPAD), ni hace autenticación integrada (NTLM/Kerberos). En un equipo cuya única salida es un
proxy configurado así, el aviso no aparece y el fallo queda en el registro de errores; no se pierde
nada más. **Rodeo:** definir `HTTPS_PROXY` con la dirección del proxy.

**No admite GitHub Enterprise Server.** Su API vive en `https://SERVIDOR/api/v3`, con ruta, y la
dirección configurable (`url_api` en `ajustes_de_version` de `preferencias.json`) solo admite un
nombre de servidor sin ruta, para no abrir la puerta a direcciones peligrosas. Funcionan
`api.github.com`, que es el valor de fábrica, y GitHub Enterprise Cloud con residencia de datos.

## Plataformas

**macOS no se ha probado.** El código tiene en cuenta esa plataforma (rutas, ausencia de X11 y
Wayland), pero nadie lo ha ejecutado ahí todavía. Si lo pruebas, cuenta qué tal en un issue.

Si encuentras algo que falle, abre un
[issue](https://github.com/celtidcs/mmcelt/issues) con los pasos para reproducirlo.
