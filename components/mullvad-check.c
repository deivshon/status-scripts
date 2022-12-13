#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include "../utils.h"

#define FREQ 60
#define DATA_FILE_PATH "/tmp/countryData"

#define IS_EXIT_IP_KEY      "mullvad_exit_ip"
#define SERVER_HOSTNAME_KEY "mullvad_exit_ip_hostname"
#define COUNTRY_KEY         "country"

// Improper (but task-adequate) json lookup
int json_lookup(char *dest, char *key, char *json) {
    char *buf = strstr(json, key);
    if(buf == NULL) return 0;
    
    buf = strstr(buf, ":") + sizeof(char);

    char *value_end = strstr(buf, ",");
    if(value_end == NULL) return 0;

    (*value_end) = '\0';
    strcpy(dest, buf);
    (*value_end) = ',';

    return 1;
}

char *quote_expand(char *str) {
    char *start;
    char *traverse = str;
    size_t index = 0;
    for(; index < strlen(traverse) && isspace(traverse[index]); index++);

    traverse += sizeof(char) * index;
    start = traverse + sizeof(char);

    if(traverse[0] == '\"') {
        traverse += sizeof(char);
        for(; index < strlen(traverse) && traverse[index] != '\"'; index++);

        traverse[index] = '\0';
    }

    return start;
}

void print_mullvad_exit(char *sep) {
    char *cmd = "curl";
    char *cmd_args[4] = {"curl", "--silent", "https://am.i.mullvad.net/json", NULL};
    char result[2048];
    get_output(result, sizeof(result) - 1, cmd, cmd_args);

    char is_exit[16];
    if(!json_lookup(is_exit, IS_EXIT_IP_KEY, result)) exit(EXIT_FAILURE);

    char output[64];

    if(streq("true", is_exit)) {
        char server_hostname[32];
        if(!json_lookup(server_hostname, SERVER_HOSTNAME_KEY, result)) exit(EXIT_FAILURE);

        char *unquoted_hostname = quote_expand(server_hostname);
        strcpy(output, unquoted_hostname);
    }
    else {
        char country[32];
        if(!json_lookup(country, COUNTRY_KEY, result)) exit(EXIT_FAILURE);
        char *unquoted_country = quote_expand(country);
        sprintf(output, "N/C - %s", unquoted_country);
    }

    printf("%s%s\n", output, sep);
    exit(EXIT_SUCCESS);
}

int main(int argc, char **argv) {
    char *sep = "";
    get_separator(&sep, argc, argv);

    print_mullvad_exit(sep);
}
