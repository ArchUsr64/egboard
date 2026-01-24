#include <RF24.h>

uint8_t COLS[] = {0, 2, 3, 4, A0, A1, A2, A3, A4, A5};
uint8_t ROWS[] = {5, 6, 7, 8};

#define COL_COUNT 10
#define ROW_COUNT 4

#define REFRESH_MS 1000

RF24 radio(10, 9);

const byte RADIO_ADDRESS[6] = "ArchUsr64";

uint32_t last_transmission_timestamp = 0;
uint8_t new_keyboard_bytes[5], old_keyboard_bytes[5];

void setup() {
	uint8_t i, j;
	Serial.begin(9600);
	Serial.println(__FILE__);
	Serial.println("Start, Initializing Radio!");
	while (!radio.begin())
		;
	Serial.println("Init success!");
	radio.setDataRate(RF24_2MBPS);
	radio.openWritingPipe(RADIO_ADDRESS);
	radio.setPALevel(RF24_PA_HIGH);
	radio.stopListening();
	Serial.end();

	for (i = 0; i < ROW_COUNT; i++)
		pinMode(ROWS[i], INPUT);

	for (j = 0; j < COL_COUNT; j++) {
		pinMode(COLS[j], OUTPUT);
		digitalWrite(COLS[j], LOW);
	}
}

void loop() {
	uint8_t key_index = 0xff, i, j;
	memset(new_keyboard_bytes, 0, sizeof(new_keyboard_bytes));

	for (i = 0; i < COL_COUNT; i++) {
		for (j = 0; j < ROW_COUNT; j++) {
			digitalWrite(COLS[i], HIGH);
			if (digitalRead(ROWS[j]) == HIGH) {
				key_index = j * COL_COUNT + i;
				new_keyboard_bytes[key_index >> 3] |= 1 << (key_index & 0b111);
			}
			digitalWrite(COLS[i], LOW);
		}
	}

	if (memcmp(new_keyboard_bytes, old_keyboard_bytes, sizeof(new_keyboard_bytes))) {
		radio.write(&new_keyboard_bytes, sizeof(new_keyboard_bytes));
		last_transmission_timestamp = millis();

		Serial.begin(9600);
		Serial.print("State: ");
		Serial.print(key_index, DEC);
		Serial.print(", 0: ");
		Serial.print(new_keyboard_bytes[0], BIN);
		Serial.print(", 1: ");
		Serial.print(new_keyboard_bytes[1], BIN);
		Serial.print(", 2: ");
		Serial.print(new_keyboard_bytes[2], BIN);
		Serial.print(", 3: ");
		Serial.print(new_keyboard_bytes[3], BIN);
		Serial.print(", 4: ");
		Serial.println(new_keyboard_bytes[4], BIN);
		Serial.end();
	}

	if (millis() - last_transmission_timestamp > REFRESH_MS) {
		Serial.begin(9600);
		Serial.println("refresh!");
		Serial.end();
		radio.write(&new_keyboard_bytes, sizeof(new_keyboard_bytes));
		last_transmission_timestamp = millis();
	}

	memcpy(old_keyboard_bytes, new_keyboard_bytes, sizeof(new_keyboard_bytes));
}
