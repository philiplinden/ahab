# Sensors

| Sensor                | Observables                                                                 |
| --------------------- | --------------------------------------------------------------------------- |
| Clock                 | Time                                                                        |
| GPS                   | Position, velocity                                                          |
| IMU                   | Orientation, Velocity, Angular velocity, Acceleration, Angular acceleration |
| Barometer             | Air pressure                                                                |
| Barometric Altitmeter | Air pressure, altitude                                                      |
| Thermometer           | Air temperature                                                             |
| Hygrometer            | Air humidity                                                                |

## Virtual sensors, derived telemetry

| Source                                                 | Observable                 |
| ------------------------------------------------------ | -------------------------- |
| Air temperature, humidity, pressure                    | Air density                |
| Ballistic trajectory vs observed velocity and position | Wind speed, wind direction |
| Dead reckoning from accelerations                      | Position, velocity         |
