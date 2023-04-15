#ifndef CPP_H
#define CPP_H

    #include <Arduino.h>
    #include "wifi.h"
    #include "adxl.h"
    #include "can_bus.h"

    #define DEBUG false
    #define BAUD_RATE 115200

    void loop();
    void setup();

    #if DEBUG
        #define DEBUG_MODE true
    #else
        #define DEBUG_MODE false
    #endif

#endif  // CPP_H