#pragma once

#include <Arduino.h>

class PwmHandler {
   public:
    PwmHandler() = delete;
    PwmHandler(const int& led_channel, const int& led_pin, const int& freq, const int& resolution)
        : led_channel_{led_channel}, led_pin_{led_pin}, freq_{freq}, resolution_{resolution} {}
    ~PwmHandler() { ledcDetachPin(led_pin_); }
    
    bool Setup();
    void DutyCycle(const float& duty_cycle) const;
    float DutyCycle() const;
    

   private:
    int freq_{0};
    int led_channel_{0};
    int resolution_{0};
    int led_pin_{0};
};