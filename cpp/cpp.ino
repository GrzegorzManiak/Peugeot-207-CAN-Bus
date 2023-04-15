#include "cpp.h"

// -- CAN
bool error = false;
MCP_CAN CAN0(3); 
long unsigned int rx_Id;
unsigned char len = 0;
unsigned char rx_buf[8];


void setup() { 
  // -- The above line can sometimes spit out errors
  Serial.begin(BAUD_RATE);

  setup_wifi();
  setup_adxl();  

  //
  // -- Begin CAN
  //
  if(CAN0.begin(MCP_ANY, CAN0_SPEED, CAN0_CLOCK) == CAN_FAIL) 
    error = true;

  // -- Set operation mode to normal so the MCP2515 sends acks to received data.
  CAN0.setMode(MCP_NORMAL);  
  pinMode(CAN0_INT, INPUT); 
}


void loop()
{
  handle_client();

  // -- If CAN0_INT pin is low, read receive buffer
  if(digitalRead(CAN0_INT)) return;                  
   
  // -- Read data: len = data length, buf = data byte(s)
  CAN0.readMsgBuf(&rx_Id, &len, rx_buf);      
  
  // -- Determine if ID is standard (11 bits) or extended (29 bits)
  if((rx_Id & 0x80000000) == 0x80000000)     
    rx_Id = (rx_Id & 0x1FFFFFFF);

  // -- Determine if message is a remote request frame.
  if((rx_Id & 0x40000000) == 0x40000000) return;
  

  // -- Array to store bytes
  // - ; is used as a delimiter
  // - . is used as a delimiter as id.size.data
  // - No spaces are used
  // String frame = "(" + String(rx_Id, HEX) + "," + String(len, HEX) + ")";

  // -- Convert the received data to bytes
  // for(byte i = 0; i<len; i++){
  //   frame += String(rx_buf[i], HEX);
  //   if(i < len-1) frame += ",";
  // }
}

