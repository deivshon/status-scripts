#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>
#include "../utils.h"

#define is_time_char(c) (c >= 48 && c <= 58) // 58 -> ':'

int main(int argc, char **argv) {
    char *sep = "";
    get_separator(&sep, argc, argv);

    char acpi_output[128];
    char *acpi_command = "acpi";
    char *acpi_args[3] = {acpi_command, "-b", NULL};

    get_output(acpi_output, sizeof(acpi_output) - 1, acpi_command, acpi_args);
    str_to_lower(acpi_output);

    if(contains_substr(acpi_output, "unavailable")  ||
       contains_substr(acpi_output, "no support")   ||
       !strcmp("", acpi_output))
    {
        exit(EXIT_SUCCESS);
    }

    char *charging = "";
    char *charge = "";
    char *remaining = "";
    char *to_end = NULL;
    if((contains_substr(acpi_output, "charging") && !contains_substr(acpi_output, "discharging"))   ||
       (contains_substr(acpi_output, "full") && !contains_substr(acpi_output, "discharging")))
    {
        charging = "CHR";
    }
    if(contains_substr(acpi_output, "%")) {
        char *traverse = strstr(acpi_output, "%");
        to_end = (traverse + sizeof(char));

        while(!isspace(*traverse)) {
            traverse -= sizeof(char);
        }
        traverse += sizeof(char);

        charge = traverse;
    }

    char *split = strtok(acpi_output, ",");
    while(split != NULL) {
        if(contains_substr(split, "remaining") || contains_substr(split, "until")) {
            for(int i = 0; isspace(split[i]); i++) split += sizeof(char);

            remaining = split;

            int colons = 0;
            while(is_time_char(*split) && colons < 2) {
                if((*split) == ':') colons++;
                split += sizeof(char);
            }
            split -= sizeof(char);

            (*split) = '\0';
            break;
        }

        split = strtok(NULL, ",");
    }

    int charging_exists = strcmp("", charging);
    int charge_exists = strcmp("", charge);
    int remaining_exists = strcmp("", remaining);

    int space_after_charge = remaining_exists && !(charging_exists && !charge_exists);

    if(to_end != NULL) (*to_end) = '\0';

    printf("BAT %s%*s%s%*s%s%s\n", charging, charging_exists ? 1 : 0, "", charge, space_after_charge ? 1 : 0, "", remaining, sep);
}
