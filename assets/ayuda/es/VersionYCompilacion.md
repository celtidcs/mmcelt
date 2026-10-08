# ℹ️ Versión y Compilación

### 👀 Dónde se ve
La versión aparece en **el título de la ventana** y en la **barra de estado**, junto con la fecha de compilación.

Para el detalle completo: **❓ Ayuda → ℹ️ Acerca de MMCelt**, o pulsando la versión en la barra de estado. Muestra versión, fecha, commit, rama y —lo más útil— **la ruta del ejecutable que se está ejecutando**.

### 💻 Sin abrir el programa
```
mmcelt --version
```

Útil cuando tienes varias copias y quieres saber cuál es cuál sin abrirlas una a una.

### 🤔 ¿Por qué no basta el número de versión?
El número no cambia entre compilaciones. Dice qué versión *pretende* ser el programa, no **qué archivo concreto** has abierto.

Es la pregunta que surge cuando hay varias copias por el disco: la de la carpeta del proyecto, una portable en un USB, otra en una carpeta de trabajo. Todas se llaman igual y todas declaran lo mismo. La fecha, el commit y la ruta sí las distinguen.

### ⚠️ Aviso de «cambios sin guardar»
Si aparece, significa que el programa se compiló con modificaciones que aún no estaban guardadas en el repositorio: ese ejecutable no corresponde exactamente a ningún commit.

### ⬆ Aviso de versión nueva
Al arrancar, MMCelt pregunta a GitHub cuál es la última versión publicada. Si es más nueva que la tuya, en la barra superior aparece `⬆ Nueva versión disponible:` con el número de la versión. Es un enlace: al pulsarlo se abre la página de esa versión en el navegador. **El programa no descarga ni instala nada**; actualizar es decisión tuya.

Si no hay conexión o GitHub no responde, no pasa nada: simplemente no hay aviso. Para que no se compruebe, desmarca `Comprobar al arrancar si hay una versión nueva` en el menú `🎨 Ver y Diseño`; desactivada, el programa no se conecta a internet al arrancar.
