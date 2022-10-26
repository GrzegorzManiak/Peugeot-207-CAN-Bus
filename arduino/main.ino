
#include <SPI.h>
#include "mcp_can.h"

long unsigned int rxId;
unsigned char len = 0;
unsigned char rxBuf[8];
char msgString[128];    // Array to store serial string

#define CAN0_INT 2      // Set INT to pin 2
MCP_CAN CAN0(53);       // Set CS to pin 53

void setup() {
  Serial.begin(115200);

    
  if(CAN0.begin(MCP_ANY, CAN_125KBPS, MCP_8MHZ) == CAN_OK)
    Serial.println("MCP2515 Initialized Successfully!");
  else Serial.println("Error Initializing MCP2515...");

  
  CAN0.setMode(MCP_NORMAL);   // Set operation mode to normal so the MCP2515 sends acks to received data.
  pinMode(CAN0_INT, INPUT);   // Configuring pin for /INT input
}


void loop()
{

  // -- If CAN0_INT pin is low, read receive buffer
  if(!digitalRead(CAN0_INT)) {

    // -- Read data: len = data length, buf = data byte(s)
    CAN0.readMsgBuf(&rxId, &len, rxBuf);      

    // -- String to hold the can frame
    String canFrame = "";

    // -- Determine if ID is standard (11 bits) or extended (29 bits)
    if((rxId & 0x80000000) == 0x80000000)    
      canFrame += "Extended ID: 0x" + String((rxId & 0x1FFFFFFF), HEX) + " DLC: " + String(len) + " Data:";
    else
      canFrame += "Standard ID: 0x" + String(rxId, HEX) + " DLC: " + String(len) + " Data:";
  
    // -- Read the CAN frame
    else for(byte i = 0; i<len; i++) {
      canFrame += " 0x" + String(rxBuf[i], HEX);
    }
      
    // -- Print the CAN frame
    Serial.println(canFrame);
  }
}
