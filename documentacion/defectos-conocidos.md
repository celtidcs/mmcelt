Defectos y limitaciones conocidos de MMCelt

Este documento reúne los defectos y las limitaciones de la versión 0.16 de MMCelt que se conocen y siguen sin corregir: qué ocurre, a quién afecta y cómo evitarlo. Ninguno impide el uso normal del programa. Lo ya corregido se cuenta en el documento historial-de-cambios.md de esta misma carpeta.

Dependencias sin mantenimiento en la accesibilidad de Linux

La auditoría de dependencias señala tres bibliotecas que ya no reciben mantenimiento por separado: zbus_names, zvariant y zvariant_derive (avisos RUSTSEC-2026-0371, RUSTSEC-2026-0370 y RUSTSEC-2026-0372). Son avisos de falta de mantenimiento, no vulnerabilidades conocidas: sus autores las han integrado en zbus 6.

Llegan a MMCelt solo a través de la accesibilidad, la parte que permite a los lectores de pantalla leer los botones y los menús. En Linux, esa parte habla con el escritorio mediante zbus 5, y ni la versión más reciente de la biblioteca de accesibilidad que usa egui ha pasado todavía a zbus 6. Se ha preferido conservar la accesibilidad a quitarla para silenciar el aviso. Desaparecerá al actualizar egui cuando esa biblioteca dé el paso.

Limitaciones del aviso de versión nueva

El aviso usa el proxy declarado en las variables de entorno HTTPS_PROXY, HTTP_PROXY y ALL_PROXY y, en Windows, si no hay ninguna, el proxy activado en las opciones de internet del sistema. No interpreta los archivos de configuración automática del proxy (PAC o WPAD) ni se autentica ante el proxy con NTLM o Kerberos. En un equipo cuya única salida a internet sea un proxy de ese tipo, el aviso no aparece y el fallo queda anotado en el registro de errores; no se pierde nada más. Para evitarlo basta con definir la variable HTTPS_PROXY con la dirección del proxy.

Plataformas

macOS no se ha probado. El código tiene en cuenta esa plataforma, con sus rutas y sin X11 ni Wayland, pero todavía no se ha ejecutado en ella. Si lo pruebas, cuenta qué tal en una incidencia del repositorio.

Si encuentras algo que falle y no esté aquí, abre una incidencia en https://github.com/celtidcs/mmcelt/issues con los pasos para reproducirlo.
