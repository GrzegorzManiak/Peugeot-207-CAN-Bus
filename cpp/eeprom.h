#ifndef EEPROM_H
#define EEPROM_H

#include <EEPROM.h>
#include <Arduino.h>

String read_string_from_eeprom(int addrOffset);
void write_string_to_eeprom(int addrOffset, String str);
void clear_eepprom_at(int addrOffset);

#endif  // EEPROM_H