#include "can_bus.h"

MCP_CAN CAN0(3); 
long unsigned int rx_Id;
unsigned char len = 0;
unsigned char rx_buf[8];
bool error = false;
char** string_list = NULL; 
size_t list_size = 0;


// -- This is a list of Packet ID's that we accept
const long unsigned int valid_ids[] = {
    0x0110100000,
    0x1000100000,
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

    if (!valid) return;
    else parse_can_packet(rx_Id, len, rx_buf);
}


void parse_can_packet(
    long unsigned int id,
    unsigned char len,
    unsigned char rx_buf[8]
) {
    // -- Convert the packet into a pseduo JSON string
    // String frame = "(" + String(rx_Id, HEX) + "," + String(len, HEX) + ")";

    String data = "[";
    for(byte i = 0; i < len; i++){
        data += String(rx_buf[i], HEX);
        if(i < len-1) data += ",";
    }
    data += "]";

    String formated_object = "{ \"id\": \"" + String(id, HEX) + "\", \"data\": " + data + " }";
    add_string(formated_object.c_str());
}   


void add_string(const char* str) {
    size_t str_len = strlen(str);

    // -- If the string list is not large enough to hold the new string, trim it
    if (list_size == 0 || list_size <= strlen(str)) {

        char** new_list = (char**)malloc((list_size + 1) * sizeof(char*));
        if (new_list == NULL) return; 

        for (size_t i = 0; i < list_size; i++) {
            new_list[i] = string_list[i]; }

        free(string_list); 
        string_list = new_list;
        list_size++; 
    }


    // -- Allocate memory for the new string and add it to the list
    char* new_string = (char*)malloc(str_len + 1); // Add 1 for the null terminator
    if (new_string == NULL) return; 
    strncpy(new_string, str, str_len);
    new_string[str_len] = '\0';
    string_list[list_size - 1] = new_string; 
}


String read_string_list() {
    String output = "[";
    for (size_t i = 0; i < list_size; i++) {
        output += string_list[i];
        if (i < list_size - 1) output += ",";  
    }
    clear_string_list();
    return output + "]";
}


void clear_string_list() {
  for (size_t i = 0; i < list_size; i++) {
    free(string_list[i]);
    string_list[i] = NULL;
  }
}
