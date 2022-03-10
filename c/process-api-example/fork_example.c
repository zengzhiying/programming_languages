#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>
#include<sys/wait.h>

int main() {
    printf("hello pid: %d \n", (int) getpid());
    int rc = fork();
    if(rc < 0) {
        fprintf(stderr, "fork failed!\n");
        exit(1);
    } else if(rc == 0) {
        sleep(1);
        printf("hello, I am a child (pid: %d)\n", (int) getpid());
    } else {
        // 等待子进程完成再退出
        int wc = wait(NULL);
        printf("hello, I am parent of %d, wc: %d (pid: %d)\n", 
                rc, wc, (int) getpid());
    }
    return 0;
}
