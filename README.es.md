[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Čeština](README.cz.md)

# RoliD Clock 0.6.0

Widget no oficial para **Braiins Deck**: reloj personalizable, fecha y día,
alarmas, cuenta atrás, efectos RGB horarios y sonidos MP3 propios.
Sin afiliación con Braiins Forge. Licencia: [GPL-3.0-or-later](LICENSE).

## Elegir idioma

Instala **solo una** variante: `rolid-clock-0.6.0-es.nar`, `-en.nar`, `-de.nar`
o `-cz.nar`. Todas comparten UUID y se sustituyen entre sí. La variante determina
el idioma del formulario de ajustes y el idioma inicial de la pantalla.
El ajuste **Idioma** cambia solo la pantalla. Una actualización puede conservar
el idioma guardado; cámbialo manualmente si hace falta. Los nombres de alarmas
escritos por el usuario no se traducen.

## Instalación

Descarga el NAR elegido y `rolid-audio-0.6.0.tar.gz` desde Releases.
Sustituye `DECK_IP` por la dirección IP del Deck.

1. Para usar MP3, ejecuta en tu ordenador, desde la carpeta de descargas:

   ```sh
   ssh root@DECK_IP 'mkdir -p /tmp/rolid-audio-060 && tar -xz -C /tmp/rolid-audio-060 && sh /tmp/rolid-audio-060/install.sh' < rolid-audio-0.6.0.tar.gz
   ```

2. En la web del Deck, carga el NAR mediante **Display Widgets → Install Widget**
   y añade el widget a una escena. Mantén activado el servicio MP3 local, ajusta
   el volumen del gong al 75% y pulsa **Sonido**. Activa el gong horario por separado
   si lo deseas.

Requiere SSH como root, uhttpd, madplay y OpenWrt/procd. El servicio solo escucha
en `127.0.0.1:18764`. Los permisos CGI 755 corrigen el error HTTP 403. Sin el
servicio funcionan el reloj y las luces, pero no los MP3 propios.

## Pantalla y alarmas

Colores y tamaños configurables para hora, fecha y cuenta atrás. Fecha y día
en una misma línea. Hora en formato de 24 horas y fecha día/mes/año. La columna
izquierda muestra los cinco próximos avisos, incluidos varios de una misma
regla repetida. Hasta 32 reglas, aplazamiento, parada y parada automática.
Opcionalmente incluye la próxima alarma nativa del Deck, sin cambiar su sonido.

Los campos son **Hora**, **Repetición**, **Nombre**. Hora: `07:00` o `7:00`.
**Las reglas se escriben en inglés en todos los idiomas.** Las reglas checas
anteriores siguen siendo válidas. Cambiar el idioma no cambia la programación.

| Repetición | Significado |
|---|---|
| Mon-Fri | De lunes a viernes |
| Sat,Sun | Fin de semana |
| Mon,Tue,Sat | Días seleccionados |
| daily | Todos los días |
| off | Desactivar la regla |
| once 2026-12-24 | Una fecha concreta |
| monthly 15 | Día 15 de cada mes |
| yearly 24.12 | Cada 24 de diciembre (día.mes) |
| days 2 2026-11-01 | Cada dos días desde la fecha |
| hours 6 2026-11-01 | Cada seis horas desde la fecha y el campo Hora |
| minutes 30 2026-11-01 | Cada 30 minutos desde la fecha y el campo Hora |
| Mon,Wed weeks 2 2026-11-02 | Lunes y miércoles cada dos semanas desde la fecha |

Días: Mon,Tue,Wed,Thu,Fri,Sat,Sun. Sin espacios en las listas de días.
Intervalos: 1–365. Fechas AAAA-MM-DD, excepto yearly, que usa día.mes. Las reglas
inválidas muestran un error. El día 31 omite meses sin ese día. Se respeta la zona
horaria del Deck y el horario de verano. Antes de editar reglas antiguas de 17
columnas, anótalas y vuelve a introducirlas en el formato compacto. No reordenes
filas con una alarma activa o aplazada.

## RGB y audio

La luz horaria es amarilla de día y naranja de noche. Noche: 22–8 por defecto
o modo nocturno del sistema. Intensidad inicial 15%, ajustable entre 1–15%.
A medianoche toda la tira alterna doce colores, uno por segundo.
**No es un arcoíris que recorre la tira:** la API del widget no permite colores
individuales por LED. Luz y gong tienen horarios separados; inicio y fin iguales
significan todo el día. No se recuperan avisos horarios perdidos. Otros widgets
y ajustes del sistema pueden tener prioridad. Botones: **Luz**, **RGB**, **Sonido**.

Archivos: `/mnt/data/rolid-audio/sounds/gong.mp3` y `alarm.mp3`. Ambos contienen
un gong sintetizado. El audio de alarma se reinicia cada cinco segundos; usa
archivos de hasta cinco segundos. Parar y posponer detienen nuestro reproductor.
75% equivale a madplay -A -10; 0% es silencio. Con la alarma detenida, carga tu
sonido desde el ordenador:

```sh
ssh root@DECK_IP 'cat > /mnt/data/rolid-audio/sounds/gong.mp3' < mi-gong.mp3
```

Para la alarma usa `alarm.mp3` como destino. La reinstalación conserva los MP3
propios. Para desactivar el servicio en el Deck:

```sh
/etc/init.d/rolid-audio stop
/etc/init.d/rolid-audio disable
```

Para eliminarlo por completo, guarda tus grabaciones y elimina únicamente
`/etc/init.d/rolid-audio` y `/mnt/data/rolid-audio`. Tras actualizar el firmware
puede ser necesario reinstalar el script de inicio.

## Diagnóstico y pruebas

En el Deck, `wget -O - 'http://127.0.0.1:18764/cgi-bin/audio?status'` debe responder
`rolid-audio-v1 ready`. Registro: `/tmp/rolid-audio/player.log`. Reinicio:
`/etc/init.d/rolid-audio restart`. El éxito HTTP no demuestra que el sonido sea audible.
Deben ejecutarse el widget y el servicio. El audio nativo del SDK no funciona en
el firmware probado; desactivar el servicio MP3 no ofrece audio alternativo allí.

Equipo probado: sistema `2026-09-11-0-ff0d18a2-26.09`, paquetes
`2026.10.26-43d1b6e7-26.10`. RGB y reproducción MP3 confirmados antes de traducir.
Todos los idiomas se prueban con un host WASM simulado. Falta comprobar en el
Deck el diseño traducido, el inicio tras reiniciar y las alarmas a largo plazo.

[Compilación (EN)](docs/BUILD.md) · [Notas de versión (EN)](RELEASE_NOTES.md)
