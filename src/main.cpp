
#include <Arduino.h>

#define RECEIVER 1
#define TRANSMITTER 2

#define MODE TRANSMITTER

#define BAUD_RATE 9600

const int PORT_SELECT_PIN = 4;
const int WINDOW_COMPARATOR_PIN = 3;

volatile bool pin_floating = false;
volatile bool interrupt = false;
volatile bool led_on = true;
volatile bool port_selected = true;

void handleInterrupt() {
    pin_floating = !digitalRead(WINDOW_COMPARATOR_PIN);
    interrupt = true;
    if (pin_floating) {
        Serial.end();
        // Empty Buffer
        while (Serial.available() > 0) 
            Serial.read();
    } else {
        Serial.begin(BAUD_RATE);
        // Empty Buffer
        while (Serial.available() > 0) 
            Serial.read();
    }
}

void set_port(bool b) {
    delay(75);
    if (!b) {
        digitalWrite(PORT_SELECT_PIN, LOW);
        port_selected = false;
    } else {
        digitalWrite(PORT_SELECT_PIN, HIGH);
        port_selected = true;
    }
    delay(75);
}

void change_port() {
    set_port(!port_selected);
}

void setup() {
    Serial.begin(9600);
    Serial1.begin(BAUD_RATE);
    Serial1.setTimeout(100);

    pinMode(WINDOW_COMPARATOR_PIN, INPUT);
    pinMode(LED_BUILTIN, OUTPUT);
    pinMode(PORT_SELECT_PIN, OUTPUT);

    digitalWrite(PORT_SELECT_PIN, HIGH);

    attachInterrupt(digitalPinToInterrupt(WINDOW_COMPARATOR_PIN), handleInterrupt, CHANGE);
    pin_floating = digitalRead(WINDOW_COMPARATOR_PIN);

    digitalWrite(LED_BUILTIN, HIGH);
    delay(100);
    digitalWrite(LED_BUILTIN, LOW);
    delay(100);
    digitalWrite(LED_BUILTIN, HIGH);
}

#if MODE == RECEIVER
void loop() {
    while (true) {
        if (pin_floating) {
            Serial.print("Disconnected");
            while (pin_floating) {
                Serial.print('.');
                delay(10);
            }
            Serial.println("");
        }
        if (Serial1.available() > 0) {
            Serial.write(Serial1.read());
        }

        if (pin_floating) {
            digitalWrite(LED_BUILTIN, LOW);
        } else {
            digitalWrite(LED_BUILTIN, HIGH);
        }
    }
}
#elif MODE == TRANSMITTER
void loop() {
    while (true) {
        /*
        String msg = Serial1.readStringUntil('\n');
        if (msg == NULL) {
            Serial.println("Timeout");
        } else {
            Serial.print("From R: ");
            Serial.println(msg);
        }
        */
        
        set_port(true);

        Serial.write("Sender: Should not be seen!\n");
        Serial1.write("Should not be seen!\n");

        set_port(false);

        Serial.write("Sender: Should be seen!\n");
        Serial1.write("Should be seen!\n");

        if (led_on) {
            digitalWrite(LED_BUILTIN, LOW);
        } else {
            digitalWrite(LED_BUILTIN, HIGH);
        }
        led_on = !led_on;
    }
}
#endif

