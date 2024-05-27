
### Component List

First the general function of the component is described and then an example component is listed.
For each component, speed and low power should be considered as the most important criterias.

- For the base board:
  - [Arduino Nano 33 IoT](https://docs.arduino.cc/hardware/nano-33-iot/)
- For the other boards:
  - [STM32F030F4](https://www.st.com/en/microcontrollers-microprocessors/stm32f030f4.html)
- Window Comparator: [TLV1702](https://www.ti.com/product/TLV1702)
  - 1 per port.
- [Magnetic Pogo Connectors](https://www.alibaba.com/product-detail/Convenient-charging-connector-Magnetic-Pogo-Pin_1600656938123.html?spm=a2700.galleryofferlist.normal_offer.d_title.39c02de6yGUCsz) 
  - 1 per port.
- Multiplexer: [SN74HCS151](https://www.ti.com/product/SN74HCS151)
  - 1 per 8 ports.
- Demultiplexer: [SN74LVC1G139](https://www.ti.com/product/SN74LVC1G139)
  - 1 per 4 ports.
- Transistors:
  - BC547
    - 1 per port, for the TX pin.
- Resistors:
  - 22k
    - 1 per port, at the base of the TX Transistor.
  - 100k
    - 1 per board, for the Window Comparator Reference Voltage.
    - 1 per port, for the pull-up resistor at the emitter of the TX Transistor.
    - 1 per port, for the pull-up resistor at the output of the window comparator.
  - 120k
    - 1 per board, for the Window Comparator Reference Voltage.
  - 150k
    - 1 per port, as the upper resistor of the voltage divider at the input to the window comparator.
  - 390k
    - 1 per board, for the Window Comparator Reference Voltage.
    - 1 per port, as the upper resistor of the voltage divider at the input to the window comparator.
- Capacitors:
  - 1uF per port, at the output of the window comparator to avoid bouncing.
