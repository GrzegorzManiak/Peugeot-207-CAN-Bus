#ifndef ADXL_H
#define ADXL_H

    #include <Arduino.h>
    #include <Wire.h>

    #define ADXL345_TIME 75 
    #define ADXL345_CAN_ID 415
    #define ADXL345_CLOCK 50000

    void setup_adxl();
    void read_adxl();
    void aggregate_adxl();

#endif  // ADXL_H