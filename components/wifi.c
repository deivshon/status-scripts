#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <dirent.h>
#include "../utils.h"

int main(int argc, char **argv) {
    char *sep = "";
    get_separator(&sep, argc, argv);

    char if_name[64];
    if_name[0] = '\0';

    int wifi_if_found = 0;
    struct dirent *d;
    DIR *interfaces;
    if((interfaces = opendir(NET_INTERFACES_PATH)) == NULL) {
        exit(EXIT_FAILURE);
    }
    while((d = readdir(interfaces)) != NULL) {
        if(starts_with(d->d_name, "wlan") || starts_with(d->d_name, "wlp")) {
            char current_interface_path[MAX_PATH_LENGTH];
            sprintf(current_interface_path, "%s/%s", NET_INTERFACES_PATH, d->d_name);
            if(operstate_up(current_interface_path)) {
                wifi_if_found = 1;
                strcpy(if_name, d->d_name);
                break;
            }
        }
    }
    closedir(interfaces);

    if(!wifi_if_found) exit(EXIT_SUCCESS);

    char iw_output[1024];
    iw_output[0] = '\0';
    char *iw_cmd = "iw";
    char *iw_args[5] = {iw_cmd, "dev", if_name, "link", NULL};

    get_cmd_output(iw_output, sizeof(iw_output) - 1, iw_cmd, iw_args);
    if(!strcmp("", iw_output)) {
        printf("WIFI: UP%s\n", sep);
        exit(EXIT_SUCCESS);
    }

    char *ssid = "";
    int dBm = __INT32_MAX__;
    int quality = 0;

    char *split = strtok(iw_output, "\n");
    while(split != NULL) {
        if(strstr(split, "SSID")) {
            ssid = strstr(split, ":") + sizeof(char) * 2;

            *(split + sizeof(char) * strlen(split)) = '\0';
        }
        else if(strstr(split, "dBm")) {
            split = strstr(split, ":") + sizeof(char) * 2;

            dBm = strtol(split, NULL, 10);
        }
        split = strtok(NULL, "\n");
    }

    int dBm_exists = dBm != __INT32_MAX__;
    int ssid_exists = strcmp("", ssid);

    if(dBm_exists) {
        if(dBm > -50) quality = 100;
        else if(dBm < -100) quality = 0;
        else quality = (dBm + 100) * 2; 
    }

    char result[128];
    char *label = "WIFI ";

    if(!ssid_exists && !dBm_exists)
        sprintf(result, "%sUP", label);
    else if(ssid_exists && !dBm_exists)
        sprintf(result, "%s%s", label, ssid);
    else if(!ssid_exists && dBm_exists)
        sprintf(result, "%s%d%%", label, quality);
    else
        sprintf(result, "%s%d%% %s", label, quality, ssid);

    strcat(result, sep);
    printf("%s\n", result);
}
