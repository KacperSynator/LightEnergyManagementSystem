#include "pwm_handler.h"


bool PwmHandler::Setup() {
        if (ledcSetup(led_channel_, freq_, resolution_) == 0) {
            return false;
        }

        ledcAttachPin(led_pin_, led_channel_);

        return true;
    }

void PwmHandler::DutyCycle(const float& duty_cycle) const {
    if (duty_cycle < 0.0 || duty_cycle > 1.0) {
        Serial.println("Duty cycle is not in [0, 1] range!");
        return;
    }

    uint32_t scaled_duty = duty_cycle * (std::pow(2, resolution_) - 1);
    ledcWrite(led_channel_, scaled_duty);
}

float PwmHandler::DutyCycle() const {
    return static_cast<float>(ledcRead(led_channel_)) / (std::pow(2, resolution_) - 1);
}