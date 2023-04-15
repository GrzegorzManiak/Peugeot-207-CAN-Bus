#include "adxl.h"

const int ADXL345 = 0x53; 
float X_out, Y_out, Z_out;  
float roll, pitch, rollF, pitchF = 0;
float avg_roll, avg_pitch = 0;
int adxl_count, adxl_time = 0;

void setup_adxl() {
    Wire.begin(); // Initiate the Wire library
    // Set ADXL345 in measuring mode
    Wire.setClock(ADXL345_CLOCK);
    Wire.beginTransmission(ADXL345); // Start communicating with the device 
    Wire.write(0x2D); // Access/ talk to POWER_CTL Register - 0x2D

    // Enable measurement
    Wire.write(8); // (8dec -> 0000 1000 binary) Bit D3 High for measuring enable 
    Wire.endTransmission();
}


void read_adxl() {
    Wire.beginTransmission(ADXL345);
    Wire.write(0x32); // Start with register 0x32 (ACCEL_XOUT_H)
    Wire.endTransmission(false);
    Wire.requestFrom(ADXL345, 6, true); // Read 6 registers total, each axis value is stored in 2 registers
    X_out = ( Wire.read() | Wire.read() << 8); // X-axis value
    X_out = X_out / 256; //For a range of +-2g, we need to divide the raw values by 256, according to the datasheet
    Y_out = ( Wire.read() | Wire.read() << 8); // Y-axis value
    Y_out = Y_out / 256;
    Z_out = ( Wire.read() | Wire.read() << 8); // Z-axis value
    Z_out = Z_out / 256;

    // -- Calculate Roll and Pitch (rotation around X-axis, rotation around Y-axis)
    roll = atan(Y_out / sqrt(pow(X_out, 2) + pow(Z_out, 2))) * 180 / PI;
    pitch = atan(-1 * X_out / sqrt(pow(Y_out, 2) + pow(Z_out, 2))) * 180 / PI;

    // -- Low-pass filter
    rollF = 0.94 * rollF + 0.06 * roll;
    pitchF = 0.94 * pitchF + 0.06 * pitch;
}


void aggregate_adxl() {
    // -- Aggregate roll and pitch
    avg_roll += rollF;
    avg_pitch += pitchF;
    adxl_count++;

    // -- Get the time
    int elapsed_time = millis() - adxl_time;


    // -- Check if we have enough data
    if(elapsed_time >= ADXL345_TIME) {
        // -- Calculate the average
        avg_roll /= adxl_count;
        avg_pitch /= adxl_count;

        // -- Send the data
        int avg_roll_int = (int)floor(avg_roll);
        int avg_pitch_int = (int)floor(avg_pitch);
        
        avg_roll = 0;
        adxl_count = 0;
        adxl_time = millis();
    }
}