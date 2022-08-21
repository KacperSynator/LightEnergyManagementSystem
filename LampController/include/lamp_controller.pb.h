/* Automatically generated nanopb header */
/* Generated by nanopb-0.4.5 */

#ifndef PB_LAMP_CONTROLLER_LAMP_CONTROLLER_PB_H_INCLUDED
#define PB_LAMP_CONTROLLER_LAMP_CONTROLLER_PB_H_INCLUDED
#include <pb.h>

#if PB_PROTO_HEADER_VERSION != 40
#error Regenerate this file with the current version of nanopb generator.
#endif

/* Struct definitions */
typedef struct _lamp_controller_LampData { 
    pb_callback_t name; 
    float illuminance; 
    float voltage; 
    float current; 
    float power; 
    float energy; 
    float frequency; 
    float power_factor; 
} lamp_controller_LampData;


#ifdef __cplusplus
extern "C" {
#endif

/* Initializer values for message structs */
#define lamp_controller_LampData_init_default    {{{NULL}, NULL}, 0, 0, 0, 0, 0, 0, 0}
#define lamp_controller_LampData_init_zero       {{{NULL}, NULL}, 0, 0, 0, 0, 0, 0, 0}

/* Field tags (for use in manual encoding/decoding) */
#define lamp_controller_LampData_name_tag        1
#define lamp_controller_LampData_illuminance_tag 2
#define lamp_controller_LampData_voltage_tag     3
#define lamp_controller_LampData_current_tag     4
#define lamp_controller_LampData_power_tag       5
#define lamp_controller_LampData_energy_tag      6
#define lamp_controller_LampData_frequency_tag   7
#define lamp_controller_LampData_power_factor_tag 8

/* Struct field encoding specification for nanopb */
#define lamp_controller_LampData_FIELDLIST(X, a) \
X(a, CALLBACK, SINGULAR, STRING,   name,              1) \
X(a, STATIC,   SINGULAR, FLOAT,    illuminance,       2) \
X(a, STATIC,   SINGULAR, FLOAT,    voltage,           3) \
X(a, STATIC,   SINGULAR, FLOAT,    current,           4) \
X(a, STATIC,   SINGULAR, FLOAT,    power,             5) \
X(a, STATIC,   SINGULAR, FLOAT,    energy,            6) \
X(a, STATIC,   SINGULAR, FLOAT,    frequency,         7) \
X(a, STATIC,   SINGULAR, FLOAT,    power_factor,      8)
#define lamp_controller_LampData_CALLBACK pb_default_field_callback
#define lamp_controller_LampData_DEFAULT NULL

extern const pb_msgdesc_t lamp_controller_LampData_msg;

/* Defines for backwards compatibility with code written before nanopb-0.4.0 */
#define lamp_controller_LampData_fields &lamp_controller_LampData_msg

/* Maximum encoded size of messages (where known) */
/* lamp_controller_LampData_size depends on runtime parameters */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif
