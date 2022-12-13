#include <stdio.h>
#include "../utils.h"

#define DATA_FILE_PATH "/tmp/cpuData"
#define PROC_FILE_PATH "/proc/stat"

void init_cpu_usage(unsigned long cpu_usage[CPU_TIME_FIELDS]) {
    for(int i = 0; i < CPU_TIME_FIELDS; i++) cpu_usage[i] = 0;
}

double get_cpu_usage_perc(unsigned long cpu_usage[CPU_TIME_FIELDS]) {
    unsigned long total = 0;
    for(int i = 0; i < CPU_TIME_FIELDS; i++) total += cpu_usage[i];

    unsigned long idle = cpu_usage[IDLE_INDEX] + cpu_usage[IOWAIT_INDEX];
    if(total == 0) return 100;
    return 100 - ((double) idle / (double) total) * 100;
}

void get_cpu_usage_diff(unsigned long dest[CPU_TIME_FIELDS], unsigned long old[CPU_TIME_FIELDS], unsigned long new[CPU_TIME_FIELDS]) {
    for(int i = 0; i < CPU_TIME_FIELDS; i++) {
        if(old[i] > new[i]) dest[i] = 0;
        else dest[i] = new[i] - old[i];
    }
}

int main(int argc, char **argv) {
    char *sep = "";
    get_separator(&sep, argc, argv);

    unsigned long old_cpu_usage[CPU_TIME_FIELDS];
    if(!get_cpu_usage(old_cpu_usage, NULL, DATA_FILE_PATH))
        init_cpu_usage(old_cpu_usage);
    
    unsigned long new_cpu_usage[CPU_TIME_FIELDS];
    char new_cpu_usage_str[256];
    init_cpu_usage(new_cpu_usage);
    get_cpu_usage(new_cpu_usage, new_cpu_usage_str, PROC_FILE_PATH);

    FILE *fs = fopen(DATA_FILE_PATH, "w");
    fprintf(fs, "%s\n", new_cpu_usage_str);
    fclose(fs);

    unsigned long result_cpu_usage[CPU_TIME_FIELDS];
    get_cpu_usage_diff(result_cpu_usage, old_cpu_usage, new_cpu_usage);

    double result_perc = get_cpu_usage_perc(result_cpu_usage);

    printf("CPU %.2f%%%s\n", result_perc, sep);
}
