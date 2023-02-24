#include "utils.h"
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <ctype.h>

void get_output(char *dest, int dest_size, char *cmd, char **cmd_args) {
    int piped[2];
    if(pipe(piped) == -1) {
        exit(EXIT_FAILURE);
    }

    if(fork() == 0) {
        close(piped[0]);    // close read end of child
        dup2(piped[1], 1);  // stdout to pipe
        dup2(piped[1], 2);  // stderr to pipe
        close(piped[1]);    // close write end of child: no longer needed

        execvp(cmd, cmd_args);
        exit(EXIT_FAILURE);
    }
    else {
        dest[0] = '\0';

        int c = 0; // Bytes read into buffer each time
        char buf[1024];
        buf[0] = '\0';

        close(piped[1]);    // close write end of parent

        while((c = read(piped[0], buf, sizeof(buf) - 1))) {
            // Sets the byte after the last one read to '\0', terminating the string
            buf[c] = '\0';

            strncat(dest, buf, dest_size - strlen(dest) - 1);

            if(strlen(dest) + strlen(buf) >= (size_t) dest_size)
                break;
        }
        close(piped[0]);    // close read end of parent: no longer needed
    }
}

void get_separator(char **dest, int argc, char **argv) {
        for(int i = 0; i < argc; i++) {
        if(streq("--separator", argv[i])) {
            (*dest) = SEPARATOR;
        }
    }
}

int to_formatted_bytes(char *dest, double bytes) {
    char *suffixes[7] = {"B", "K", "M", "G", "T", "P", "E"};

    double approx_bytes = bytes;
    unsigned int divisions = 0;

    while(approx_bytes > 1024 && divisions <= 7) {
        approx_bytes /= 1024;
        divisions++;
    }

    if(divisions == 0)
        sprintf(dest, "%.0lf%s", approx_bytes, suffixes[divisions]);
    else
        sprintf(dest, "%.2lf%s", approx_bytes, suffixes[divisions]);

    return 1;
}

int get_cpu_usage(unsigned long dest[CPU_TIME_FIELDS], char *dest_str, char *file_path) {
    FILE *fs = fopen(file_path, "r");
    if(fs == NULL) return 0;
    char data_line[256];

    data_line[0] = '\0';
    if(!fgets(data_line, 256, fs)) return 0;

    if(dest_str != NULL) strcpy(dest_str, data_line);

    char *buf = strtok(data_line, " ");
    for(int i = 0; i < CPU_TIME_FIELDS; i++) {
        buf = strtok(NULL, " ");
        if(buf == NULL) break;

        dest[i] = strtoul(buf, NULL, 10);
    }
    fclose(fs);

    return 1;
}

void str_to_lower(char str[]) {
    for(int i = 0; str[i]; i++) {
        str[i] = tolower(str[i]);
    }
}

int operstate_up(char *interface_path) {
    char operstate_path[MAX_PATH_LENGTH];
    sprintf(operstate_path, "%s/%s", interface_path, "operstate");

    FILE *fs = fopen(operstate_path, "r");
    if(fs == NULL) return 0;

    char content[16];
    fgets(content, 16, fs);

    fclose(fs);

    strip(content);

    return streq("up", content);
}
