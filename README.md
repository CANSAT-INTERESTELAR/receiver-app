# Aplicación receptora de Interestelar

Este programa se encarga de interpretar, mostrar y guardar los datos transmitidos por nuestro CanSat.

Por ahora está en fase de prueba de concepto, no se debe esperar buen código :D

Confirmación de features:
- [x] Logs de telemetría en un archivo.
- [x] Visualización de la orientación del satélite (falta un modelo 3D).
- [ ] Mapa mostrando la posición del satélite (usar GMaps).
- [x] Gráficos mostrando los datos (usar [Chartist](https://github.com/gionkunz/chartist-js)).

Además de la altura obtenida por medio del GPS, la aplicación calcula la altura con la presión:
![Altura en función de la presión](/images/presion-altura.png)