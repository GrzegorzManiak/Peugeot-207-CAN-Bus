#ifndef WIFI_H
#define WIFI_H

#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <ESP8266WebServer.h>
#include "eeprom.h"

// -- Wifi
#define APSSID "Peugeot-207"
#define APPSK "gregsWifiPassword"
#define HOSTNAME "Peugeot-207"
#define WIFI_SAVED_TIMEOUT 60000 // 1 minute
#define ALLOWED_WIFI_CHARS "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,<.>/?"

// -- EEPROM
#define SAVED_SSID_ADDR_S 0
#define SAVED_PSK_ADDR_S 32

void setup_wifi();
void handle_ap();
void handle_ssid_pass();
void handle_client();
void clear_saved_wifi();
void read_frame();

#endif  // WIFI_H