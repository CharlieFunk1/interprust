/*
 * WebSocketClient.ino
 *
 *  Created on: 24.05.2015
 *
 */

#include <Arduino.h>
#include <Adafruit_NeoPixel.h>
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <WiFiUdp.h>
#include <iostream>
#include <WebSocketsClient.h>
#include <SD.h>
#include <Hash.h>
#include <string>
#include <vector>
#include <sstream>

using namespace std;


File myFile;
#define PIN 4
const int chipSelect = 15;
int num_pixels = 0;
unsigned int localUdpPort = 4210;  // local port to listen on
char incomingPacket[450];  // buffer for incoming packets

WiFiUDP EthernetUdp;

Adafruit_NeoPixel ledstrip = Adafruit_NeoPixel(num_pixels, PIN, NEO_GRB + NEO_KHZ800);

ESP8266WiFiMulti WiFiMulti;
WebSocketsClient webSocket;

#define USE_SERIAL Serial


void loop()
{
  int packetSize = EthernetUdp.parsePacket();
  if (packetSize)
  {
    // receive incoming UDP packets
    Serial.printf("Received %d bytes from %s, port %d\n", packetSize, EthernetUdp.remoteIP().toString().c_str(), EthernetUdp.remotePort());
    int len = EthernetUdp.read(incomingPacket, 450);
    if (len > 0)
    {
      incomingPacket[len] = 0;
    }
    //Serial.printf("UDP packet contents: %s\n", incomingPacket);

    int k=0;
    int strip[num_pixels][3];
       
    for (int i = 0; i < num_pixels; i++) {
      for (int j = 0; j < 3; j++) {
                  
        strip[i][j] = incomingPacket[k];
        //Serial.println(k);
        k++;
      }
    }
    
    for (int l = 0; l < (num_pixels); l++) {
      ledstrip.setPixelColor(l, strip[l][0], strip[l][1], strip[l][2]);
    }
    ledstrip.show();
  }
}
    

void setup() {
	// USE_SERIAL.begin(921600);
	USE_SERIAL.begin(115200);

  Serial.print("Initializing SD card...");

  if (!SD.begin(chipSelect)) {
    Serial.println("Initialization failed!");
    while (1);
  }
  Serial.println("initialization done.");

  myFile = SD.open("config.txt");
  //  Serial.println("config.txt:");
  
  string charr;
  string line;               
  while (myFile.available()) {                
      charr = myFile.read();
      line.append(charr);
      //Serial.println(line.c_str());
      //Serial.println("loop");             
  }
   vector<string> result;
   stringstream line_strm(line); //create string stream from the string
   while(line_strm.good()) {
      string substr;
      getline(line_strm, substr, ','); //get first string delimited by comma
      result.push_back(substr);
   }
  //Serial.println(result.at(0).c_str());
  string ssid_ = result.at(0);
  string password_ = result.at(1);
  string num_leds_ = result.at(2);
  string server_ip_ = result.at(3);
  num_pixels = stoi(num_leds_);

  //Serial.println(ssid_.c_str());
  //Serial.println(password_.c_str());
  //Serial.println(num_pixels + 5);
  //USE_SERIAL.printf(server_ip_.c_str());
   
  myFile.close();

  ledstrip.updateLength(num_pixels);
  
	//Serial.setDebugOutput(true);
	USE_SERIAL.setDebugOutput(true);

	USE_SERIAL.println();
	USE_SERIAL.println();
	USE_SERIAL.println();

	for(uint8_t t = 4; t > 0; t--) {
		USE_SERIAL.printf("[SETUP] BOOT WAIT %d...\n", t);
		USE_SERIAL.flush();
		delay(1000);
	}

	WiFiMulti.addAP(ssid_.c_str(), password_.c_str());

	//WiFi.disconnect();
	while(WiFiMulti.run() != WL_CONNECTED) {
		delay(100);
	}
 
  USE_SERIAL.printf("Connecting to ");
  USE_SERIAL.printf(server_ip_.c_str());
  USE_SERIAL.printf("\n");
  
	// server address, port and URL
	EthernetUdp.begin(localUdpPort);
  Serial.printf("Now listening at IP %s, UDP port %d\n", WiFi.localIP().toString().c_str(), localUdpPort);
	// event handler
	

	// use HTTP Basic Authorization this is optional remove if not needed
	//webSocket.setAuthorization("user", "Password");

	// try ever 5000 again if connection has failed
	
  
  // start heartbeat (optional)
  // ping server every 15000 ms
  // expect pong from server within 3000 ms
  // consider connection disconnected if pong is not received 2 times
  //webSocket.enableHeartbeat(15000, 3000, 2);

  ledstrip.begin();
  ledstrip.show();

}
