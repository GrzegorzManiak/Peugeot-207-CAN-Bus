#include "wifi.h"

const char *ap_ssid = APSSID;
const char *ap_password = APPSK;
ESP8266WebServer server(80);

IPAddress ip(192, 168, 0, 75);
IPAddress gateway(192, 168, 0, 1);
IPAddress subnet(255, 255, 255, 0); 

void start_ap() {
    Serial.println("");
    Serial.println("Starting AP");
    WiFi.softAP(ap_ssid, ap_password);
    IPAddress myIP = WiFi.softAPIP();

    Serial.println("");
    Serial.println("AP started");
    Serial.print("AP IP address: ");
    Serial.println(myIP);
}

void setup_wifi() {
    Serial.println("Starting WIFI");

    // -- Try to connect to the saved AP
    //    if that fails, start the AP
    String saved_ssid = read_string_from_eeprom(SAVED_SSID_ADDR_S);
    String saved_pass = read_string_from_eeprom(SAVED_PSK_ADDR_S);
    
    Serial.println("SSID: " + saved_ssid);
    Serial.println("PSK: " + saved_pass);

    if (
        saved_pass.length() > 3 && 
        saved_ssid.length() > 3
    ) {
        int started_at = millis();
        Serial.println("Connecting to saved AP");

        // -- Print all the available networks
        Serial.println("Available networks:");
        int n = WiFi.scanNetworks();
        for (int i = 0; i < n; i++) {
            Serial.println(WiFi.SSID(i));
        }

        // -- Set the mode 
        WiFi.mode(WIFI_STA);
        WiFi.config(ip, gateway, subnet);
        WiFi.hostname(String(HOSTNAME).c_str());
        WiFi.begin(saved_ssid, saved_pass);

        // -- Attempt to connect to the saved AP
        while (WiFi.status() != WL_CONNECTED) {
            delay(500);
            Serial.print(".");

            // -- If we can't connect to the saved AP
            //    after 1 minute, start the AP
            if (millis() - started_at > WIFI_SAVED_TIMEOUT) {
                // -- Stop trying to connect to the saved AP
                WiFi.disconnect();
                start_ap();
                break;
            }
        }


        // -- If we did successfully connect to the saved AP
        if (WiFi.status() == WL_CONNECTED) {
            Serial.println("");
            Serial.println("WiFi connected");
            Serial.print("IP address: ");
            Serial.println(WiFi.localIP());
        }
    } 
    
    else start_ap();

    // -- Start the routes
    Serial.println("Starting routes");
    server.on("/", handle_ap);
    server.on("/ap", handle_ssid_pass);
    server.on("/clear_saved", read_frame);
    server.on("/read", read_frame);

    Serial.println("Starting Server");
    server.begin();
    Serial.println("Server Started");
}

void handle_client() {
    server.handleClient();
}


void clear_saved_wifi() {
    // -- Print out the old ssid and psk
    Serial.println("Old ssid: " + read_string_from_eeprom(SAVED_SSID_ADDR_S));
    Serial.println("Old psk: " + read_string_from_eeprom(SAVED_PSK_ADDR_S));

    // -- Clear the ssid and psk from the EEPROM
    clear_eepprom_at(SAVED_SSID_ADDR_S);
    clear_eepprom_at(SAVED_PSK_ADDR_S);
    
    Serial.println("Cleared saved wifi");
    server.send(200, "text/html", "Cleared saved wifi");
}


// -- This route is called when the server
//    receives a request to /ap?ssid=xxx&psk=xxx
void handle_ssid_pass() {
    // -- Get the ssid and psk from the request
    String ssid = server.arg("ssid");
    String psk = server.arg("psk");


    // -- Make sure the ssid and psk are valid
    if (
        ssid.length() < 3 || psk.length() < 3 ||
        ssid.length() > 32 || psk.length() > 64
    ) {
        server.send(400, "text/html", "Invalid ssid or psk");
        return;
    }


    // -- Make sure the ssid and psk only contain
    //    allowed characters
    for (int i = 0; i < ssid.length(); i++) {
        if (!strchr(ALLOWED_WIFI_CHARS, ssid[i])) {
            server.send(400, "text/html", "Invalid ssid or psk");
            return;
        }
    }

    for (int i = 0; i < psk.length(); i++) {
        if (!strchr(ALLOWED_WIFI_CHARS, psk[i])) {
            server.send(400, "text/html", "Invalid ssid or psk");
            return;
        }
    }
    

    // -- Save the ssid and psk to the EEPROM
    write_string_to_eeprom(SAVED_SSID_ADDR_S, ssid);
    write_string_to_eeprom(SAVED_PSK_ADDR_S, psk);
    delay(1000);
    
    // -- Read the EEPROM 
    Serial.println("New ssid: " + read_string_from_eeprom(SAVED_SSID_ADDR_S));
    Serial.println("New psk: " + read_string_from_eeprom(SAVED_PSK_ADDR_S));

    // -- Commit and restart
    server.send(200, "text/html", "Valid ssid and psk");
    delay(2000);
    ESP.restart();
}



// -- This route actually send out the html
//    form
void handle_ap() {
    Serial.println("Serving AP Screen");
    server.send(200, "text/html", 
        "<form action=\"/ap\" method=\"get\">"
        "SSID: <input type=\"text\" name=\"ssid\"><br>"
        "PSK: <input type=\"text\" name=\"psk\"><br>"
        "<input type=\"submit\" value=\"Submit\">"
        "</form>"
    );
}


void read_frame() {
    // String data = "data=" + String(combined_frames);
    // server.send(200, "text/html", data);
}