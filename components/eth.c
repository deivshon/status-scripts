#include <stdio.h>
#include <string.h>
#include <dirent.h>
#include <stdlib.h>
#include "../utils.h"

int main(int argc, char **argv) {
    char *sep = "";
    int isUp = 0;
    
    get_separator(&sep, argc, argv);

    struct dirent *d;
    DIR *interfaces;
    if((interfaces = opendir(NET_INTERFACES_PATH)) == NULL) {
        exit(EXIT_FAILURE);
    }

    while((d = readdir(interfaces)) != NULL) {
        if(starts_with(d->d_name, "eth") || starts_with(d->d_name, "enp")) {
            char current_interface_path[MAX_PATH_LENGTH];
            sprintf(current_interface_path, "%s/%s", NET_INTERFACES_PATH, d->d_name);
            isUp = operstate_up(current_interface_path);
            break;
        }
    }
    closedir(interfaces);

    if(!isUp) exit(EXIT_SUCCESS);
    
    char *res = "ETH: UP";

    printf("%s%s\n", res, sep);
}
