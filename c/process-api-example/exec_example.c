#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>
#include<string.h>
#include<sys/wait.h>

int main(int argc, char *argv[]) {
    printf("hello pid: %d \n", (int) getpid());
    int rc = fork();
    if(rc < 0) {
        fprintf(stderr, "fork failed!\n");
        exit(1);
    } else if(rc == 0) {
        printf("hello, I am a child (pid: %d)\n", (int) getpid());
        char *myargs[3];
        myargs[0] = strdup("wc");
        myargs[1] = strdup("exec_example.c");
        myargs[2] = NULL;
        execvp(myargs[0], myargs);
        // exec替换当前进程 下面代码不会执行
        printf("this shouldn't print out"); 
    } else {
        // 等待子进程完成再退出
        int wc = wait(NULL);
        printf("hello, I am parent of %d, wc: %d (pid: %d)\n", 
                rc, wc, (int) getpid());
    }
    return 0;
}
