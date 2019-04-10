#include <stdio.h>
#include <string.h>
#include "video.h"

/**
 * gcc video.c -fPIC -shared -o libvideo.so
 */

void video_proc(char *abc) {
    int size = strlen(abc);
    printf("length: %d\n", size);
}

void video_proc1(void *abc) {
    int size = strlen((char *)abc);
    printf("length: %d\n", size);
}