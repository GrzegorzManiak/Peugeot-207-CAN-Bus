#ifndef CPP_H
#define CPP_H

#include <Arduino.h>
#include "mcp_can.h"
#include "wifi.h"
#include "adxl.h"

// -- Config
#define CAN0_INT 2      // Set INT to pin 2

#define DEBUG true
#define CAN0_SPEED CAN_125KBPS
#define CAN0_CLOCK MCP_8MHZ
#define BAUD_RATE 115200

void loop();
void setup();

#endif  // CPP_H