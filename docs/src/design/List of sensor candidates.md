---
tags:
  - design
  - ideation
  - sensors
---
# Sensors to use

> [!NOTE] Some info may not apply
> Some of the sensors and use cases here might not apply to vehicles that are not airborne.
## Observables

Here is a (non-exhaustive) list of sensors and the observables they can could
provide.

| Sensor                | Observables                                                                 |
| --------------------- | --------------------------------------------------------------------------- |
| Clock                 | Time                                                                        |
| GPS                   | Position, velocity                                                          |
| IMU                   | Orientation, Velocity, Angular velocity, Acceleration, Angular acceleration |
| Barometer             | Air pressure                                                                |
| Barometric Altitmeter | Air pressure, altitude                                                      |
| Thermometer           | Air temperature                                                             |
| Hygrometer            | Air humidity                                                                |

And maybe some other telemetry that would be derived from the above sensors.

| Source                                                 | Observable                 |
| ------------------------------------------------------ | -------------------------- |
| Air temperature, humidity, pressure                    | Air density                |
| Ballistic trajectory vs observed velocity and position | Wind speed, wind direction |
| Dead reckoning from accelerations                      | Position, velocity         |

## Parts

- [RFM96W LoRa Transceiver (915 MHz)](https://www.sparkfun.com/products/12775).
  This also comes in a 433 MHz version, but as far as I can tell it's a region
  lock thing---The Americas use 915 Mhz for the license-free ISM band. The 915
  MHz version has an operating power of up to 100 mW and 300 kbps, which should
  be enough for me.
- [NEO-M9N GPS](https://www.sparkfun.com/products/18378). According to
  the datasheet it should work up to 80 km. This is the newest version of this
  chip but I think modules as old as NEO-M6 would work.
- [BME280 Atmospheric Sensor](https://www.sparkfun.com/products/15440). The
  myth, the legend, the classic pressure, humidity, and temperature sensor.

Some other parts that are overkill but would be nice to have:

- [Qwiic MicroPressure Sensor](https://www.sparkfun.com/products/16476). This is
  a barometer that has a calibrated sensing range of 60mbar to 2.5bar. I think
  it would be useful to have a sensitive barometer for higher altitudes where a
  more general purpose barometer might not be sensitive enough.
- [OpenLog Artemis](https://www.sparkfun.com/products/16832). This is a
  data logger board with a built-in IMU, voltage loggers, some high-rate
  sampling for a few channels or ~250Hz logging in general. It automatically
  detects, configures, and logs Qwiic boards, including all the sensors I'd want
  to use for this project.
