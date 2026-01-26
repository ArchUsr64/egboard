#include <RF24.h>
#include <Wire.h>

RF24 radio(8, 9);

const uint8_t RADIO_ADDRESS[6] = "ArchUsr64";

#define STATUS_LED A0
#define TIMEOUT_MS 5000

uint32_t last_transmission_timestamp = 0;
uint8_t keyboard_bytes[5];

void setup() {
	Serial.begin(115200);
	Serial.println(__FILE__);
	Serial.println("Start, Initializing Radio!");
	while (!radio.begin())
		;
	Serial.println("Init success!");
	radio.openReadingPipe(0, RADIO_ADDRESS);
	radio.setDataRate(RF24_2MBPS);
	radio.setPALevel(RF24_PA_HIGH);
	radio.startListening();

	Wire.begin(0x64);
	Wire.onRequest(i2c_response);

	pinMode(STATUS_LED, OUTPUT);
	digitalWrite(STATUS_LED, HIGH);
	memset(keyboard_bytes, 0, sizeof(keyboard_bytes));
}

void loop() {
	if (radio.available()) {
		digitalWrite(STATUS_LED, HIGH);
		last_transmission_timestamp = millis();
		radio.read(&keyboard_bytes, sizeof(keyboard_bytes));
		Serial.print("State: ");
		Serial.print("0: ");
		Serial.print(keyboard_bytes[0], BIN);
		Serial.print(", 1: ");
		Serial.print(keyboard_bytes[1], BIN);
		Serial.print(", 2: ");
		Serial.print(keyboard_bytes[2], BIN);
		Serial.print(", 3: ");
		Serial.print(keyboard_bytes[3], BIN);
		Serial.print(", 4: ");
		Serial.println(keyboard_bytes[4], BIN);
	}
	if (millis() - last_transmission_timestamp > TIMEOUT_MS) {
		digitalWrite(STATUS_LED, LOW);
		memset(keyboard_bytes, 0, sizeof(keyboard_bytes));
		Serial.println("Timed out, connection broked?");
	}
}

void i2c_response() {
	Wire.write(keyboard_bytes, sizeof(keyboard_bytes));
}
