#include "cpp.h"

void setup() { 
    // -- The above line can sometimes spit out errors
    Serial.begin(BAUD_RATE);

    setup_wifi();
    setup_adxl();  
    setup_can_bus();
}


void loop() {
    unsigned char data[8] = {0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf1};

    parse_can_packet(
      0x220,
      0x08,
      data
    );

    handle_client();
    read_from_can_bus();
}

