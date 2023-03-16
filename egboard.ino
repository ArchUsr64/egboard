#define len(x) sizeof(x) / sizeof(int)
#define iter_over(i, x) for (int i = 0; i < len(x); i++)
#define iter(i, x) for (int i = 0; i < (x); i++)

#define DEBOUNCE_PERIOD 4

#define SYNC (char)0xaa

#define FLASH_DURATION 20

// currently not being used for anything
int button = 3;

int rows[] = {A3, A2, A1, A0};
int cols[] = {2, 3, 4, 5, 6};

int led = 9;
unsigned led_timer = 32;

void setup() {
	Serial.begin(4800);

	// drive all digital pins to save power
	iter(i, 20) if (i != button) pinMode(i, OUTPUT);

	pinMode(led, OUTPUT);
	iter_over(i, cols) pinMode(cols[i], INPUT_PULLUP);
	iter_over(i, rows) {
		pinMode(rows[i], OUTPUT);
		digitalWrite(rows[i], HIGH);
	}
}

long get_key_state() {
	long state = 0L;
	iter_over(i, rows) {
		digitalWrite(rows[i], LOW);
		iter_over(j, cols) {
			long mask = 1L << (i * len(cols) + j);
			bool pressed = !digitalRead(cols[j]);
			if (pressed)
				state |= mask;
			else
				state &= ~mask;
		}
		digitalWrite(rows[i], HIGH);
	}
	return state;
}

long battery_remaining_3bit() {
#define DISCHARGE_VOLTAGE 2950
#define CHARGE_VOLTAGE 4150
	long battery_percentage = read_vcc();
	battery_percentage -= DISCHARGE_VOLTAGE;
	battery_percentage /= (CHARGE_VOLTAGE - DISCHARGE_VOLTAGE) / 100;
	if (battery_percentage < 0)
		return 0;
	if (battery_percentage > 100)
		return 7;
	return battery_percentage / 13;
}

long read_vcc() {
	long result;
	// Read 1.1V reference against AVcc
	ADMUX = _BV(REFS0) | _BV(MUX3) | _BV(MUX2) | _BV(MUX1);
	// Wait for Vref to settle
	delay(2);
	// Convert
	ADCSRA |= _BV(ADSC);
	while (bit_is_set(ADCSRA, ADSC))
		;
	result = ADCL;
	result |= ADCH << 8;
	// Back-calculate AVcc in mV
	result = 1111407 / result;
	// Vcc in mV
	return result;
}

long debounced_key_state() {
	long key_state_before = get_key_state();
	delay(DEBOUNCE_PERIOD / 2);
	long key_state_after = get_key_state();
	delay(DEBOUNCE_PERIOD / 2);
	long debounced_key_state = key_state_before & key_state_after;
	return debounced_key_state;
}

inline long add_crc(long input) {
	return ((input << 3) | input % 7);
}

void loop() {
	static int iter = 0;
	static long previous_key_state = 0;
	byte battery_bits = 0;

	if (led_timer > 0) {
		led_timer--;
	} else {
		digitalWrite(led, LOW);
	}

	if (iter % 256 == 0) {
		digitalWrite(led, HIGH);
		led_timer = FLASH_DURATION;
		battery_bits = battery_remaining_3bit();
	}

	long key_state = debounced_key_state();
	long data = add_crc((key_state << 3) | battery_bits);

	iter(i, 2 * (key_state != previous_key_state) + 1) {
		Serial.print((char)data);
		Serial.print((char)(data >> 8));
		Serial.print((char)(data >> 16));
		Serial.print(SYNC);
	}

	previous_key_state = key_state;
	iter++;
}
