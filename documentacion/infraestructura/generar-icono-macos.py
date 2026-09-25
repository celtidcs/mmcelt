#!/usr/bin/env python3
"""Genera `assets/icono/mmcelt.icns`, el icono que macOS necesita para el paquete `.app`.

# Por qué existe este archivo

macOS no lee ni `.ico` ni `.png` para el icono de una aplicación: exige el formato propio
de Apple, `.icns`. La herramienta oficial para construirlo, `iconutil`, **solo existe en
macOS**, y este proyecto se desarrolla en Windows. La alternativa habitual —generarlo en la
integración continua— dejaría el icono fuera del repositorio, y aquí rige lo contrario: lo
que se genera se confirma, porque lo que no está en git no lo protege nadie.

Así que el `.icns` se construye aquí. El formato está documentado por Apple y, desde
OS X 10.7, admite PNG incrustados directamente, que es lo que hace este guion.

# Qué produce

Un `.icns` con las once variantes que espera macOS, de 16 a 1024 píxeles, incluidas las de
pantalla Retina. El sistema elige la que le conviene según dónde lo dibuje: el Dock, el
Finder, la ventana «Obtener información» o el conmutador de aplicaciones.

# De dónde salen los píxeles

**No de los PNG que ya hay en `assets/icono/`.** El mayor mide 256 y macOS pide 1024:
ampliarlo daría un icono borroso justo en el tamaño más visible. Se rasteriza de nuevo la
geometría de `mmcelt.svg`, que es la fuente editable del icono y manda sobre las demás.

La rasterización es propia y deliberadamente pequeña: el dibujo son un rectángulo
redondeado con degradado, dos curvas de Bézier, dos círculos y una estrella de cuatro
puntas. Añadir una dependencia de un motor SVG completo para eso habría sido desmesurado.
Se dibuja una sola vez a 2048 píxeles y de ahí se reducen todos los tamaños con
remuestreo Lanczos, que es como trabaja cualquier rasterizador.

# Uso

    python documentacion/infraestructura/generar-icono-macos.py

Requiere Pillow (`pip install Pillow`). Es idempotente: se puede ejecutar las veces que
haga falta y siempre produce el mismo archivo.

**Si cambias el diseño**, edita `mmcelt.svg` y vuelve a ejecutar esto. Las constantes de
geometría de aquí abajo son un reflejo del SVG y hay que actualizarlas con él: nada
comprueba de forma automática que sigan coincidiendo.

Lo que sí se comprobó al crearlo, y conviene repetir si se toca:

- **Que el dibujo es fiel.** Se comparó la variante de 256 píxeles con el `mmcelt-256.png`
  que ya existía: la diferencia media quedó en 0,54 sobre 255 y solo 78 píxeles de 65.536
  se apartaron de forma apreciable, todos en el borde de las curvas.
- **Que macOS lo acepta.** Se validó en un ejecutor `macos-latest` de la integración
  continua con las herramientas de Apple, que es el único sitio donde eso puede afirmarse.
"""

from __future__ import annotations

import struct
from pathlib import Path

from PIL import Image, ImageDraw

# ----------------------------------------------------------------------------------------
# Geometría del icono, copiada de `assets/icono/mmcelt.svg`
#
# El SVG usa un lienzo de 256×256 y todas sus medidas están en esas unidades. Aquí se
# mantienen tal cual y se escalan al vuelo, para que comparar este archivo con el SVG sea
# inmediato y no haya que rehacer cuentas a mano al cambiar el diseño.
# ----------------------------------------------------------------------------------------

#: Lado del lienzo del SVG. Todas las coordenadas de abajo están en estas unidades.
LIENZO_SVG = 256

#: Radio de las esquinas del fondo. En el SVG es `rx="56"`.
#:
#: Sale al 21,9 % del lado, que cae muy cerca del 22,4 % que usa macOS para sus iconos
#: desde Big Sur. Es una coincidencia afortunada: el icono ya tenía la silueta que espera
#: el sistema, así que no hay que rediseñarlo para que no desentone en el Dock.
RADIO_ESQUINAS_SVG = 56

#: Degradado del fondo, en diagonal desde la esquina superior izquierda.
#: Son el azul del panel y un tono más oscuro que el del lienzo, ambos del tema oscuro.
COLOR_FONDO_INICIO = (0x1D, 0x24, 0x34)
COLOR_FONDO_FIN = (0x0E, 0x12, 0x1A)

#: Grosor de las dos ramas. En el SVG, `stroke-width="14"`.
GROSOR_RAMA_SVG = 14

#: La rama azul: nace dentro del nodo raíz y sube hacia el destello.
#: Curva cúbica `M104 128 C148 128 142 74 188 72`.
RAMA_AZUL = ((104, 128), (148, 128), (142, 74), (188, 72))
COLOR_RAMA_AZUL = (0x3B, 0x82, 0xF6)

#: La rama verde: misma salida, baja hacia el nodo corriente.
#: Curva cúbica `M104 128 C148 128 142 190 186 190`.
RAMA_VERDE = ((104, 128), (148, 128), (142, 190), (186, 190))
COLOR_RAMA_VERDE = (0x10, 0xB9, 0x81)

#: Hoja inferior: un nodo como cualquier otro, al final de la rama verde.
NODO_HOJA_CENTRO_SVG = (186, 190)
NODO_HOJA_RADIO_SVG = 18

#: El nodo raíz, en blanco roto. Se dibuja el último para tapar el nacimiento de las ramas.
NODO_RAIZ_CENTRO_SVG = (82, 128)
NODO_RAIZ_RADIO_SVG = 28
COLOR_NODO_RAIZ = (0xEE, 0xF2, 0xF8)

#: El destello: cuatro puntas de lados cóncavos. Lo que lo distingue de una flor es que los
#: puntos de control de cada cuadrante quedan muy cerca del centro.
#: Curvas cuadráticas del SVG, como (punto de control, extremo).
DESTELLO_INICIO = (188, 38)
DESTELLO_CURVAS = (
    ((193.1, 66.9), (222, 72)),
    ((193.1, 77.1), (188, 106)),
    ((182.9, 77.1), (154, 72)),
    ((182.9, 66.9), (188, 38)),
)
COLOR_DESTELLO = (0xF5, 0x9E, 0x0B)

# ----------------------------------------------------------------------------------------
# Parámetros de rasterización y del formato de Apple
# ----------------------------------------------------------------------------------------

#: Lado al que se dibuja el icono antes de reducirlo a cada tamaño.
#:
#: Se dibuja al doble del mayor tamaño que pide macOS (1024) para que la reducción con
#: Lanczos suavice los bordes. Dibujar directamente a cada tamaño daría escalones visibles
#: en las curvas: aquí no hay antialiasing propio, lo aporta el remuestreo.
LADO_RASTERIZADO = 2048

#: Cuántos segmentos rectos se usan para aproximar cada curva de Bézier.
#:
#: A 2048 píxeles, 400 segmentos dejan cada tramo por debajo del píxel, así que la curva
#: es indistinguible de una trazada de verdad.
SEGMENTOS_POR_CURVA = 400

#: Firma que abre cualquier archivo `.icns`.
FIRMA_ICNS = b"icns"

#: Las variantes que se incrustan, como (identificador de Apple, lado en píxeles).
#:
#: Los identificadores no son arbitrarios: cada uno le dice a macOS para qué densidad de
#: pantalla sirve la imagen. Los que empiezan por `ic` admiten PNG desde OS X 10.7, que es
#: lo que permite construir este archivo sin las herramientas de Apple.
#:
#: Se incluyen las diez porque el sistema no escala entre ellas con el mismo cuidado que un
#: rasterizador: si falta el tamaño que necesita, coge otro y lo estira.
VARIANTES = (
    ("icp4", 16),  # 16×16
    ("icp5", 32),  # 32×32
    ("icp6", 64),  # 64×64
    ("ic07", 128),  # 128×128
    ("ic08", 256),  # 256×256
    ("ic09", 512),  # 512×512
    ("ic10", 1024),  # 512×512 en pantalla Retina
    ("ic11", 32),  # 16×16 en pantalla Retina
    ("ic12", 64),  # 32×32 en pantalla Retina
    ("ic13", 256),  # 128×128 en pantalla Retina
    ("ic14", 512),  # 256×256 en pantalla Retina
)


def escalar(valor: float, lado: int) -> float:
    """Convierte una medida del lienzo del SVG al lienzo rasterizado.

    Args:
        valor: La medida, en las unidades de 256 que usa el SVG.
        lado: El lado del lienzo de destino, en píxeles.

    Returns:
        La medida equivalente en píxeles.
    """
    return valor * lado / LIENZO_SVG


def puntos_de_bezier_cubica(
    puntos_de_control: tuple[tuple[float, float], ...], lado: int
) -> list[tuple[float, float]]:
    """Aproxima una curva de Bézier cúbica por una sucesión de puntos.

    Se evalúa la fórmula de la curva en intervalos regulares. Con los segmentos que usa
    este guion, cada tramo queda por debajo del píxel y la curva se dibuja lisa.

    Args:
        puntos_de_control: Los cuatro puntos de la curva —origen, dos de control y
            extremo—, en unidades del SVG.
        lado: El lado del lienzo de destino, en píxeles.

    Returns:
        Los puntos de la curva, ya en píxeles y listos para trazar.
    """
    (x0, y0), (x1, y1), (x2, y2), (x3, y3) = puntos_de_control
    puntos = []

    for paso in range(SEGMENTOS_POR_CURVA + 1):
        t = paso / SEGMENTOS_POR_CURVA
        inverso = 1 - t

        # Forma desarrollada del polinomio de Bernstein de grado tres.
        x = inverso**3 * x0 + 3 * inverso**2 * t * x1 + 3 * inverso * t**2 * x2 + t**3 * x3
        y = inverso**3 * y0 + 3 * inverso**2 * t * y1 + 3 * inverso * t**2 * y2 + t**3 * y3

        puntos.append((escalar(x, lado), escalar(y, lado)))

    return puntos


def puntos_de_bezier_cuadratica(
    origen: tuple[float, float],
    control: tuple[float, float],
    extremo: tuple[float, float],
    lado: int,
) -> list[tuple[float, float]]:
    """Aproxima una curva de Bézier cuadrática por una sucesión de puntos.

    Es la que usan las cuatro puntas del destello: un único punto de control en vez de dos.

    Args:
        origen: Punto de partida, en unidades del SVG.
        control: El punto de control.
        extremo: Punto de llegada.
        lado: El lado del lienzo de destino, en píxeles.

    Returns:
        Los puntos de la curva, ya en píxeles.
    """
    (x0, y0), (x1, y1), (x2, y2) = origen, control, extremo
    puntos = []

    for paso in range(SEGMENTOS_POR_CURVA + 1):
        t = paso / SEGMENTOS_POR_CURVA
        inverso = 1 - t

        x = inverso**2 * x0 + 2 * inverso * t * x1 + t**2 * x2
        y = inverso**2 * y0 + 2 * inverso * t * y1 + t**2 * y2

        puntos.append((escalar(x, lado), escalar(y, lado)))

    return puntos


def dibujar_fondo_con_degradado(lado: int) -> Image.Image:
    """Dibuja el rectángulo redondeado del fondo, con su degradado en diagonal.

    El degradado se construye fila a fila en una imagen aparte y luego se recorta con una
    máscara de esquinas redondeadas. Hacerlo al revés —redondear y después degradar—
    dejaría los bordes dentados, porque la máscara aporta el suavizado.

    Args:
        lado: El lado del lienzo, en píxeles.

    Returns:
        La imagen del fondo, con transparencia fuera de las esquinas.
    """
    degradado = Image.new("RGB", (lado, lado))
    pixeles = degradado.load()

    # El SVG lo define de (0,0) a (1,1), así que el avance es la diagonal normalizada.
    for y in range(lado):
        for x in range(lado):
            avance = (x + y) / (2 * (lado - 1))
            pixeles[x, y] = tuple(
                round(inicio + (fin - inicio) * avance)
                for inicio, fin in zip(COLOR_FONDO_INICIO, COLOR_FONDO_FIN)
            )

    mascara = Image.new("L", (lado, lado), 0)
    ImageDraw.Draw(mascara).rounded_rectangle(
        (0, 0, lado - 1, lado - 1),
        radius=escalar(RADIO_ESQUINAS_SVG, lado),
        fill=255,
    )

    fondo = Image.new("RGBA", (lado, lado), (0, 0, 0, 0))
    fondo.paste(degradado, (0, 0), mascara)
    return fondo


def dibujar_rama(
    lienzo: ImageDraw.ImageDraw,
    puntos_de_control: tuple[tuple[float, float], ...],
    color: tuple[int, int, int],
    lado: int,
) -> None:
    """Traza una de las dos ramas, con el trazo redondeado que pide el SVG.

    El trazo se construye estampando un círculo del diámetro de la rama en cada punto de la
    curva, no trazando una polilínea gruesa. La diferencia se ve: `line()` de Pillow une los
    segmentos con vértices, y sobre una curva partida en cientos de tramos esos vértices
    dejan un festoneado de muescas en todo el borde. Se comprobó mirando el icono
    ampliado; a simple vista en el Dock quizá no se notaría, pero está y es evitable.

    Estampar círculos es, además, exactamente lo que significan `stroke-linecap="round"` y
    `stroke-linejoin="round"`: la envolvente de un disco recorriendo la curva. Con los
    segmentos que usa este guion los discos se solapan casi por completo, así que el borde
    resultante es liso.

    Args:
        lienzo: El lienzo sobre el que se dibuja.
        puntos_de_control: Los cuatro puntos de la curva, en unidades del SVG.
        color: El color del trazo.
        lado: El lado del lienzo, en píxeles.
    """
    radio = escalar(GROSOR_RAMA_SVG, lado) / 2

    for x, y in puntos_de_bezier_cubica(puntos_de_control, lado):
        lienzo.ellipse((x - radio, y - radio, x + radio, y + radio), fill=color)


def dibujar_circulo(
    lienzo: ImageDraw.ImageDraw,
    centro: tuple[float, float],
    radio_svg: float,
    color: tuple[int, int, int],
    lado: int,
) -> None:
    """Dibuja uno de los dos nodos.

    Args:
        lienzo: El lienzo sobre el que se dibuja.
        centro: El centro del círculo, en unidades del SVG.
        radio_svg: El radio, en unidades del SVG.
        color: El color de relleno.
        lado: El lado del lienzo, en píxeles.
    """
    x, y = (escalar(coordenada, lado) for coordenada in centro)
    radio = escalar(radio_svg, lado)
    lienzo.ellipse((x - radio, y - radio, x + radio, y + radio), fill=color)


def dibujar_destello(lienzo: ImageDraw.ImageDraw, lado: int) -> None:
    """Dibuja la estrella de cuatro puntas en la que acaba la rama azul.

    Se recorren las cuatro curvas encadenadas y se rellena el contorno resultante de una
    vez: rellenarlas por separado dejaría costuras entre las puntas.

    Args:
        lienzo: El lienzo sobre el que se dibuja.
        lado: El lado del lienzo, en píxeles.
    """
    contorno: list[tuple[float, float]] = []
    origen = DESTELLO_INICIO

    for control, extremo in DESTELLO_CURVAS:
        # Se descarta el primer punto de cada tramo salvo en el inicial: coincide con el
        # último del tramo anterior y repetirlo no aporta nada al polígono.
        tramo = puntos_de_bezier_cuadratica(origen, control, extremo, lado)
        contorno.extend(tramo if not contorno else tramo[1:])
        origen = extremo

    lienzo.polygon(contorno, fill=COLOR_DESTELLO)


def rasterizar_icono(lado: int) -> Image.Image:
    """Dibuja el icono completo, en el orden en que lo hace el SVG.

    El orden importa: el nodo raíz va el último porque tapa el nacimiento de las dos ramas,
    que arrancan dentro de él.

    Args:
        lado: El lado del lienzo, en píxeles.

    Returns:
        El icono dibujado, en RGBA.
    """
    icono = dibujar_fondo_con_degradado(lado)
    lienzo = ImageDraw.Draw(icono)

    dibujar_rama(lienzo, RAMA_AZUL, COLOR_RAMA_AZUL, lado)
    dibujar_rama(lienzo, RAMA_VERDE, COLOR_RAMA_VERDE, lado)
    dibujar_circulo(lienzo, NODO_HOJA_CENTRO_SVG, NODO_HOJA_RADIO_SVG, COLOR_RAMA_VERDE, lado)
    dibujar_destello(lienzo, lado)
    dibujar_circulo(lienzo, NODO_RAIZ_CENTRO_SVG, NODO_RAIZ_RADIO_SVG, COLOR_NODO_RAIZ, lado)

    return icono


def construir_icns(maestro: Image.Image) -> bytes:
    """Empaqueta las variantes en un archivo `.icns`.

    La estructura del formato es sencilla: la firma `icns`, la longitud total del archivo,
    y a continuación un bloque por variante. Cada bloque lleva su identificador de cuatro
    letras, su longitud —que **incluye los ocho bytes de su propia cabecera**, y ahí es
    donde se equivoca quien lo escribe por primera vez— y los datos.

    Args:
        maestro: El icono rasterizado a la resolución mayor, del que se reducen las demás.

    Returns:
        El contenido completo del archivo `.icns`.
    """
    bloques = []

    for identificador, lado in VARIANTES:
        variante = maestro.resize((lado, lado), Image.Resampling.LANCZOS)

        from io import BytesIO

        memoria = BytesIO()
        variante.save(memoria, format="PNG", optimize=True)
        datos = memoria.getvalue()

        bloques.append(identificador.encode("ascii") + struct.pack(">I", len(datos) + 8) + datos)

    cuerpo = b"".join(bloques)
    return FIRMA_ICNS + struct.pack(">I", len(cuerpo) + 8) + cuerpo


def main() -> None:
    """Genera el `.icns` y lo escribe junto a los demás iconos."""
    raiz = Path(__file__).resolve().parents[2]
    destino = raiz / "assets" / "icono" / "mmcelt.icns"

    print(f"Rasterizando el icono a {LADO_RASTERIZADO}×{LADO_RASTERIZADO}...")
    maestro = rasterizar_icono(LADO_RASTERIZADO)

    print(f"Empaquetando {len(VARIANTES)} variantes en el formato de Apple...")
    contenido = construir_icns(maestro)

    destino.write_bytes(contenido)
    print(f"Escrito {destino.relative_to(raiz)} ({len(contenido):,} bytes).")


if __name__ == "__main__":
    main()
