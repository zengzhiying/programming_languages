#include <stdio.h>
#include <unistd.h>
#include <time.h>
#include <sys/time.h>

int main()
{
    // 1. cpu计时函数, 为cpu真正运行的时钟, 其中sleep不算, 结果接近于0
    clock_t start, end;
    start = clock();
    // 操作...
    sleep(3);
    end = clock();
    double time_cost = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Time cost: %lf\n", time_cost);

    // 2. 实际等待时间计时, 程序中间实际经过的时间
    // struct timeval {
    //     long tv_sec;  // 秒数
    //     long tv_usec; // 微妙数
    // };
    struct timeval time1, time2;
    gettimeofday(&time1, NULL);
    // 操作...
    sleep(3);
    gettimeofday(&time2, NULL);
    double time_cost2 = (time2.tv_sec - time1.tv_sec) + (time2.tv_usec - time1.tv_usec) / 1e6;
    printf("Time cost2: %lf\n", time_cost2);
    return 0;
}
