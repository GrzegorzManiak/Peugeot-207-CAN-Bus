#include <Arduino.h>
#include <SPI.h>
#include "mcp_can.h"


// -- Pins
#define CAN0_INT 2      // Set INT to pin 2
MCP_CAN CAN0(53);       // Set CS to pin 53


// -- Config
#define CAN0_SPEED CAN_125KBPS
#define CAN0_CLOCK MCP_8MHZ
#define BAUD_RATE 115200

#define FRAME_BATCH_SIZE 15 // -- Number of frames to send over serial at once
#define FRAME_BATCH_MS 100  // -- Time in MS to collect frames before sending, if not full


// -- Variables
long unsigned int rx_Id;
unsigned char len = 0;
unsigned char rx_buf[8];

String combined_frames = "";
unsigned long time_since_last_frame = 0;
unsigned int frame_count = 0;



void setup() {
  Serial.begin(BAUD_RATE);

  // -- Begin CAN
  if(CAN0.begin(MCP_ANY, CAN0_SPEED, CAN0_CLOCK) == CAN_OK)
    Serial.println("MCP2515 Initialized Successfully!");
  else Serial.println("Error Initializing MCP2515...");

  // -- Set operation mode to normal so the MCP2515 sends acks to received data.
  CAN0.setMode(MCP_NORMAL);  

  // -- Configuring pin for /INT input 
  pinMode(CAN0_INT, INPUT); 
}


void loop()
{
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
  String frame = "";

  // -- Add ID and Lenght
  frame += "(" + String(rx_Id, HEX) + "," + String(len, HEX) + ")";

  // -- Convert the received data to bytes
  for(byte i = 0; i<len; i++){
    frame += String(rx_buf[i], HEX);
    if(i < len-1) frame += ",";
    else frame += " ";
  }

  // -- Add frame to combined_frames
  combined_frames += frame;
  frame_count++;

  // -- If frame_count is equal to FRAME_BATCH_SIZE send frames
  if (
    frame_count >= FRAME_BATCH_SIZE ||
    (millis() - time_since_last_frame) > FRAME_BATCH_MS
  ) {
    Serial.println(combined_frames + ";");
    combined_frames = "";
    frame_count = 0;
  }

  time_since_last_frame = millis();
}
