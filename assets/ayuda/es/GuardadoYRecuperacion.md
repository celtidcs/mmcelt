# 💾 Guardado y Recuperación

### 💾 Guardar tú (Ctrl + S)
Va donde tú elijas. Si algo falla —permisos, disco lleno, carpeta de nube sin sincronizar— **la aplicación te lo dice**. Nunca informa de un guardado correcto sin haberlo comprobado.

Cuando hay que elegir carpeta, el diálogo se abre en el sitio más probable: la del archivo actual, o **la del proyecto de código al que se refiere el mapa** si sus nodos llevan rutas de archivo. Es una sugerencia: puedes ir a donde quieras.

### 🛟 Guardado automático
Cada dos minutos, si el mapa ha cambiado, se escribe una copia de seguridad. Si el programa se cierra de forma inesperada, al volver a abrirlo se te ofrece recuperarla.

Ese intervalo **se puede cambiar, o apagar del todo**, en `🎨 Ver y Diseño → 💾 Autoguardado`: desactivado, cada minuto, cada dos, cada cinco o cada diez. La elección se recuerda entre sesiones.

**No sustituye a guardar.** Es una red de seguridad, no un lugar donde tu trabajo esté a salvo. Sigue pulsando Ctrl + S.

La copia va a la carpeta de datos de MMCelt (`%APPDATA%\MMCelt\recuperacion\` en Windows), **nunca junto a tus archivos**: así no te aparecen archivos sueltos en tus proyectos. Solo se conserva la última sesión, y se borra en cuanto guardas de verdad.

En los modos de disposición automática, mover un nodo no cuenta como cambio: el programa va a recolocarlo de todas formas, y un mapa que solo estás mirando no provoca escrituras.

En **Posición Libre Manual** sí cuenta, porque ahí colocar los nodos es tu trabajo y no lo recoge nada más.

### ⚠️ Si al abrir te ofrece recuperar
Te dice de cuándo es la copia y qué mapa contiene. Decides tú: si cerraste a propósito sin guardar, descártala. No se restaura sola.

### 🛡️ Archivos dañados
Un mapa que describa una estructura imposible —con ciclos, sin nodo raíz, con referencias rotas— se rechaza al abrirlo, indicando el nodo concreto. No es una restricción caprichosa: cargarlo bloquearía la aplicación. Si el archivo lo generó una IA, pídele que lo regenere; si hay un `.bak` al lado, prueba con él.

### 🌐 Formatos abiertos de intercambio (OPML y FreeMind .mm)
Además del formato nativo `.mmcelt`, puedes exportar e importar mapas en dos estándares abiertos no propietarios: OPML (esquemas jerárquicos) y `.mm` (FreeMind y Freeplane).

Al exportar a OPML o `.mm`, el título, la jerarquía de ramas y las notas de texto libre se guardan en las etiquetas estándar del formato, legibles por cualquier visor externo. Los datos propios de MMCelt que esos formatos no admiten de forma nativa (estado, prioridad, rol, supervisión humana, rutas a código fuente y conexiones cruzadas) viajan serializados en JSON estructurado al pie de la nota.

Al reimportar ese archivo en MMCelt, se reconstruye el mapa al 100% sin perder un solo metadato ni conexión lateral. Si abres el archivo en otra herramienta, verás la estructura del mapa y leerás los metadatos como texto informativo al final de cada nota.

Si importas un archivo externo generado por otra aplicación que no contenga metadatos de MMCelt, el programa asigna valores iniciales seguros (estado Idea, prioridad Media, rol Subtema salvo la raíz, que recibe Idea Central), aplica automáticamente una disposición visual equilibrada para que los nodos no aparezcan amontonados en el centro, y te muestra un aviso transparente explicando qué atributos se han configurado por defecto.
