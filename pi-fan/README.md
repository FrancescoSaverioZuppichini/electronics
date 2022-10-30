# PI - FAN

![alt](./images/header.png)

PWD controller fan, turning on only when a certain temperature is reached.

## Schematics

### Parts

- 5V fan
- one diode, in my case `1n4001`
- one bjt transistor, in my case `2n2222`
- `1k` ohm resistor, in my case two `470` ohm resistors

The schematic below shows the circuit.

![alt](./images/schematics.png)
*made with https://www.circuitlab.com/*
**R2 should be 1k ohm**

## Build

I've used a perfboard to create the circuit. Here the results

![alt](/images/0.jpeg)

The final result is very small!

![alt](/images/1.jpeg)

A little test to ensure everything is working :) 

![alt](/images/test.mp4)

## Software