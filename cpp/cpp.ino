#include "cpp.h"

void setup() { 
    // -- The above line can sometimes spit out errors
    Serial.begin(BAUD_RATE);

    setup_wifi();
    setup_adxl();  
    setup_can_bus();
}


void loop() {
    handle_client();
    read_from_can_bus();
}

