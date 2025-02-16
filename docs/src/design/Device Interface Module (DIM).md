---
tags:
  - hardware
  - design
aliases:
  - DIM
---
# Device Interface Module (DIM)

A **Device Interface Emulator (DIM)** sits in between the host vehicle's [[Main Computer (MC)|Main Computer (MC)]] and any device, such as a sensor or actuator. The DIM is the MFC's "gateway" to the low-level device. The DIM transforms device activities into data that can be interpreted by the MFC. In this way, DIMs abstract device specifics from the main flight software.