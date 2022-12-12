#include <stdio.h>
#include <stdlib.h>
#include "../utils.h"

#define MEM_FILE_PATH "/proc/meminfo"

int main(int argc, char **argv) {
    char result[64];
    result[0] = '\0';

    char *sep = "";
    get_separator(&sep, argc, argv);
    
    FILE *fs = fopen(MEM_FILE_PATH, "r");
    if(fs == NULL) exit(EXIT_FAILURE);

    double memTotal = -1;
    double memAvail = -1;
    double inUse = -1;
    double percentageInUse = -1;

    char buf[1024];
    char *buf_line;
    while(fgets(buf, 1024, fs) != NULL && (memTotal == -1 || memAvail == -1)) {
        buf_line = strtok(buf, " ");
        if(streq("MemTotal:", buf_line)) {
            buf_line = strtok(NULL, " ");
            if(buf_line == NULL) exit(EXIT_FAILURE);

            memTotal = strtod(buf_line, NULL);
        }
        else if(streq("MemAvailable:", buf_line)) {
            buf_line = strtok(NULL, " ");
            if(buf_line == NULL) exit(EXIT_FAILURE);

            memAvail = strtod(buf_line, NULL);
        }
    }

    if(memTotal == -1 || memAvail == -1) exit(EXIT_FAILURE);
    memTotal *= 1024;
    memAvail *= 1024;

    char memTotalStr[16];
    char inUseStr[16];

    inUse = memTotal - memAvail;
    percentageInUse = (inUse / memTotal) * 100;

    to_formatted_bytes(memTotalStr, memTotal);
    to_formatted_bytes(inUseStr, inUse);
    sprintf(result, "RAM %s/%s (%.2f%%)", inUseStr, memTotalStr, percentageInUse);
    strcat(result, sep);
    printf("%s\n", result);
}
