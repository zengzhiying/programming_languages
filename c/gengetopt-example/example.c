#include <stdio.h>
#include "cmdline.h"

// generate cmdline code: gengetopt < example.ggo --file-name=cmdline
// gcc example.c cmdline.c -o example

// http://www.gnu.org/software/gengetopt/gengetopt.html

int main(int argc, char const *argv[])
{
    struct gengetopt_args_info args_info;

    if(cmdline_parser(argc, argv, &args_info) != 0)
        return 1;

    printf("host: [%d] %s\n", args_info.host_given, args_info.host_arg);
    printf("port: [%d] %d\n", args_info.port_given, args_info.port_arg);
    printf("conf: [%d] %s\n", args_info.conf_given, args_info.conf_arg);
    printf("log: [%d] %d\n", args_info.log_given, args_info.log_flag);
    printf("parallel: [%d] %d\n", args_info.parallel_given, args_info.parallel_arg);

    printf("nums: ");
    for(int i = 0; i < args_info.num_given; i++)
        printf("%d ", args_info.num_arg[i]);
    printf("\n");

    printf("values: ");
    for(int i = 0; i < args_info.value_given; i++)
        printf("%s ", args_info.value_arg[i]);
    printf("\n");

    return 0;
}

// test: ./example --host 192.168.0.31 --port 6371 --log -P 8 --num 100 --num 200 --value abc --value bcd
