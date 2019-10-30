#include<stdio.h>

int main() {
    int flag = 1;
    int i;
    int count = 0;
    int s;
    while(flag) {
        scanf("%d", &i);
        if(i == 0) {
            break;
        }
        count++;
        s += i;
    }
    printf("%d,%d\n", s, count);
    return 0;
}
