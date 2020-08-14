/**
 * CPU Fan PWM
 * Send UDP data packets through WIFI network to control fan rotation.
 * 
 * ```text
 * +-+-+-+-+-+-+-+-+-+-+
 * | flag  | value     |
 * +-+-+-+-+-+-+-+-+-+-+
 * |       | fan rate  |
 * +-+-+-+-+-+-+-+-+-+-+
 * | u8    | u16       |
 * +-+-+-+-+-+-+-+-+-+-+
 * |       | 0~1023    |
 * +-+-+-+-+-+-+-+-+-+-+
 * ```
 */

#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

/* WIFI authorization */
#ifndef STASSID
#define STASSID ""
#define STAPSK  ""
#endif

/**
 * `SPEED` Fan speed pin.
 * `PWM` Fan pwm pin.
 */
const int SPEED = D2;
const int PWM = D1;

int RATE = 0;
byte BUFFER[3];  // udp buffer.
int BUFFER_SIZE = 3;  // buffer size.
unsigned int PORT = 8088;  // udp server port.
WiFiUDP Udp;

/* bytes to int */
int into_int(int offset) {
    int i_into = 0;
    int b_size = BUFFER_SIZE - offset;
    for (int i = 0; i < b_size; i ++)
        i_into += BUFFER[i + offset] * pow(256, b_size - i - 1);
    return i_into;
}

/* decoder udp buffer */
int decoder_buffer() {
    if (!Udp.parsePacket()) return -1;
    if (Udp.read(BUFFER, BUFFER_SIZE) < BUFFER_SIZE) return -1;
    if (BUFFER[0] != 1) return -1;
    return into_int(1);
}

/* flush udp buffer */
int drop_buffer() {
    Udp.flush();
    for (int i = 0; i < BUFFER_SIZE; i ++)
        BUFFER[i] = 0;
}

/**
 * initialize pin.
 * initialize WIFI.
 */
void setup() {
    pinMode(PWM, OUTPUT);
    WiFi.mode(WIFI_STA);
    WiFi.begin(STASSID, STAPSK);
    while (WiFi.status() != WL_CONNECTED) delay(2000);
    Udp.begin(PORT);
}

/* main poll */
void loop() {
    int rate = decoder_buffer();
    analogWrite(PWM, rate < 0 ? RATE : rate);
    if (rate >= 0) drop_buffer();
    RATE = rate;
    delay(1000);
}
