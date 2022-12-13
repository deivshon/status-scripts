#include <string.h>

#ifndef CUTILS
#define CUTILS

#define MAX_PATH_LENGTH 4096
#define NET_INTERFACES_PATH "/sys/class/net"

#define SEPARATOR "|"

#define strip(str) str[strcspn(str, "\n")] = '\0';
#define streq(str1, str2) !strcmp(str1, str2)
#define starts_with(str, prefix) !strncmp(str, prefix, strlen(prefix))
#define contains_substr(str, substr) (strstr(str, substr) != NULL)

#define USER_INDEX          0
#define NICE_INDEX          1
#define SYSTEM_INDEX        2
#define IDLE_INDEX          3
#define IOWAIT_INDEX        4
#define IRQ_INDEX           5
#define SOFTIRQ_INDEX       6
#define STEAL_INDEX         7
#define GUEST_INDEX         8
#define GUEST_NICE_INDEX    9

#define CPU_TIME_FIELDS     10

void get_separator(char **dest, int argc, char **argv);
void get_output(char *dest, int dest_size, char *cmd, char **cmd_args);
int to_formatted_bytes(char *dest, double bytes);
int get_cpu_usage(unsigned long cpu_data[CPU_TIME_FIELDS], char *dest_str, char *file_path);
void str_to_lower(char str[]);
int operstate_up(char *interface_path);
#endif
