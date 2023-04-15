#include "eeprom.h"

String read_string_from_eeprom(int addrOffset)
{
    // -- Begin the EEPROM transaction
    EEPROM.begin(512);
        
    int newStrLen = EEPROM.read(addrOffset);
    Serial.println("Reading from EEPROM at " + String(addrOffset) + " with length " + String(newStrLen));
    char data[newStrLen + 1];

    for (int i = 0; i < newStrLen; i++) {
        data[i] = EEPROM.read(addrOffset + 1 + i); }

    data[newStrLen] = '\0';
    return String(data);
}

void write_string_to_eeprom(int addrOffset, String str)
{
    int newStrLen = str.length();
    Serial.println("Writing to EEPROM: " + str + " at " + String(addrOffset) + " with length " + String(newStrLen));
    EEPROM.write(addrOffset, newStrLen);

    for (int i = 0; i < newStrLen; i++) {
        EEPROM.write(addrOffset + 1 + i, str[i]); }

    EEPROM.commit();
}

void clear_eepprom_at(int addrOffset)
{
    int len = EEPROM.read(addrOffset);
    Serial.println("Clearing EEPROM at " + String(addrOffset) + " with length " + String(len));
    for (int i = 0; i < len; i++) {
        EEPROM.write(addrOffset + 1 + i, 0); }

    EEPROM.write(addrOffset, 0);
    EEPROM.commit();
}