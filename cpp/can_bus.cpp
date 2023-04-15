#include "can_bus.h"
#include <Arduino.h>

MCP_CAN CAN0(3); 
long unsigned int rx_Id;
unsigned char len = 0;
unsigned char rx_buf[8];
bool error = false;

// -- This stores the list of strings that we will send to the server
String string_list[MAX_STRING_LIST_SIZE];


// -- This is a list of Packet ID's that we accept
const long unsigned int valid_ids[] = {
    0x0B6, // -- Odometer and Extras
    0x220, // -- Door Status
};


void setup_can_bus() {
    if(CAN0.begin(MCP_ANY, CAN0_SPEED, CAN0_CLOCK) == CAN_FAIL) 
        error = true;

    // -- Set operation mode to normal so the MCP2515 sends acks to received data.
    CAN0.setMode(MCP_NORMAL);  
    pinMode(CAN0_INT, INPUT); 
}


void read_from_can_bus() {
     // -- If CAN0_INT pin is low, read receive buffer
    if(digitalRead(CAN0_INT)) return;                  
    
    // -- Read data: len = data length, buf = data byte(s)
    CAN0.readMsgBuf(&rx_Id, &len, rx_buf);      
    
    // -- Determine if ID is standard (11 bits) or extended (29 bits)
    if((rx_Id & 0x80000000) == 0x80000000)     
        rx_Id = (rx_Id & 0x1FFFFFFF);

    // -- Determine if message is a remote request frame.
    if((rx_Id & 0x40000000) == 0x40000000) return;
    
    // -- Check if the ID is valid
    bool valid = false;
    for (int i = 0; i < sizeof(valid_ids); i++) {
        if (rx_Id == valid_ids[i]) {
            valid = true;
            break;
        }
    }

    if (!valid && !FREE_FLOW) return;
    else parse_can_packet(rx_Id, len, rx_buf);
}


void parse_can_packet(
    long unsigned int id,
    unsigned char len,
    unsigned char rx_buf[8]
) {
    // -- Convert the packet into a pseduo JSON string
    // String frame = "(" + String(rx_Id, HEX) + "," + String(len, HEX) + ")";

    String data;
    for(byte i = 0; i < len; i++){
        data += String(rx_buf[i], HEX);
        if(i < len-1) data += ",";
    }

    String formated_object = String(id, HEX) + ":" + data
    add_string(formated_object.c_str());
}   


void add_string(const char* str) {
    // -- If the list is full, trim it
    int list_size = sizeof(string_list) / sizeof(string_list[0]);

    if (list_size >= MAX_STRING_LIST_SIZE) {
        for (size_t i = 0; i < MAX_STRING_LIST_SIZE - 1; i++) {
            string_list[i] = string_list[i + 1];
        }
        list_size--;
    }

    // -- Add the new string to the list
    string_list[list_size] = str;
}


void clear_string_list() {
    int list_size = sizeof(string_list) / sizeof(string_list[0]);
    for (size_t i = 0; i < list_size; i++) {
        string_list[i] = "";
    }
}


String read_string_list() {
    String output;
    int list_size = sizeof(string_list) / sizeof(string_list[0]);
    for (size_t i = 0; i < list_size; i++) {
        output += string_list[i];
        if (i < list_size - 1) output += "|";  
    }
    clear_string_list();
    return output;
}
