#include <RF24.h>

RF24 radio(8, 9);

const uint8_t RADIO_ADDRESS[6] = "ArchUsr64";

#define STATUS_LED A0
#define TIMEOUT_MS 5000

uint32_t last_transmission_timestamp = 0;

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

	pinMode(STATUS_LED, OUTPUT);
	digitalWrite(STATUS_LED, HIGH);
}

void loop() {
	uint8_t keyboard_bytes[5];
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
		Serial.println("Timed out, connection broked?");
	}
}
