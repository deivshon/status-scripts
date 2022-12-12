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

void save_country_data(char *output, int n) {
    FILE *fs = fopen(DATA_FILE_PATH, "w");
    if(fs == NULL) exit(EXIT_FAILURE);

    fprintf(fs, "%s\n%d\n", output, n);
    fclose(fs);
}

void update(char *sep) {
    char *cmd = "curl";
    char *cmd_args[4] = {"curl", "--silent", "https://am.i.mullvad.net/json", NULL};
    char result[2048];
    get_cmd_output(result, sizeof(result) - 1, cmd, cmd_args);

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

    save_country_data(output, 0);

    printf("%s%s\n", output, sep);
    exit(EXIT_SUCCESS);
}

void print_old_increase(FILE *fs, char *sep) {
    char output[64];
    fgets(output, sizeof(output) - 1, fs);
    strip(output);

    char n_str[16];
    int n;
    fgets(n_str, sizeof(n_str) - 1, fs);
    n = atoi(n_str);

    fclose(fs);

    if(n < FREQ) save_country_data(output, n + 1);
    else update(sep);

    printf("%s%s\n", output, sep);

    exit(EXIT_SUCCESS);
}

int main(int argc, char **argv) {
    char *sep = "";
    get_separator(&sep, argc, argv);

    FILE *fs = fopen(DATA_FILE_PATH, "r");
    if(fs == NULL) update(sep);
    else print_old_increase(fs, sep);
}
