#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include "../utils.h"

void fetch_line_data(char *data, char *avail, char *mount_point) {
    char *rest;
    char *buf = strtok_r(data, " ", &rest);
    for(int i = 0; buf != NULL; i++) {
        if(i == 3) {
            strcpy(avail, buf);
        }
        else if(i == 5) {
            strcpy(mount_point, buf);
        }
        buf = strtok_r(NULL, " ", &rest);
    }
}

int main(int argc, char **argv) {
    char *sep = "";
    get_separator(&sep, argc, argv);

    char *cmd = "df";
    char *cmd_args[3] = {"df", "-h", NULL};
    char res[2048];
    get_cmd_output(res, 2048, cmd, cmd_args);
    wait(NULL);

    char current_avail[16];
    char current_mount_point[MAX_PATH_LENGTH];

    char *rest;
    char *buf = strtok_r(res, "\n", &rest);
    while(buf != NULL) {
        fetch_line_data(buf, current_avail, current_mount_point);
        if(streq("/", current_mount_point)) {
            char res[32] = "DISK ";
            strcat(res, current_avail);
            strcat(res, sep);
            printf("%s\n", res);
            exit(EXIT_SUCCESS);
        }
        buf = strtok_r(NULL, "\n", &rest);
    }
}
