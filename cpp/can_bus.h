#ifndef CAN_BUS_H
#define CAN_BUS_H
    #include <Arduino.h>
    #include "mcp_can.h"

    // -- Config
    #define CAN0_INT 2      // Set INT to pin 2
    #define CAN0_SPEED CAN_125KBPS
    #define CAN0_CLOCK MCP_8MHZ
    #define FREE_FLOW true
    #define MAX_STRING_LIST_SIZE 50

    void setup_can_bus();
    void parse_can_packet(
        long unsigned int id,
        unsigned char len,
        unsigned char data[8]
    );
    void read_from_can_bus();
    void clear_string_list();
    String read_string_list();
    void add_string(const char* str);

#endif  // CAN_BUS_H
