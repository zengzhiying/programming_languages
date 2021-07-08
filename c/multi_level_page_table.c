#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
// #include <unistd.h>

// 多级页表简单实现示例

int main(int argc, char const *argv[])
{
    uint32_t v_addr = 0x01020f3c;
    uint32_t p_addr = 0xc57ef000;

    // 一级页表
    int ****mpt = (int ****)malloc(32 * sizeof(int ***));
    for(int i = 0; i < 32; i++) {
        // 二级页表
        int ***pt2 = (int ***)malloc(32 * sizeof(int **));
        mpt[i] = pt2;
        for(int j = 0; j < 32; j++) {
            // 三级页表
            int **pt3 = (int **)malloc(32 * sizeof(uint32_t *));
            pt2[j] = pt3;
            for(int k = 0; k < 32; k++) {
                // 四级页表
                uint32_t *pt4 = (uint32_t *)malloc(32 * sizeof(uint32_t));
                pt3[k] = pt4;
            }
        }
    }

    // set mapping
    uint8_t i1 = (v_addr & 0xf8000000) >> 27;
    uint8_t i2 = (v_addr & 0x07c00000) >> 22;
    uint8_t i3 = (v_addr & 0x003e0000) >> 17;
    uint8_t i4 = (v_addr & 0x0001f000) >> 12;
    printf("i1: %d, i2: %d, i3: %d, i4: %d\n", i1, i2, i3, i4);
    mpt[i1][i2][i3][i4] = p_addr;
    // 3313434428
    printf("p: %lu\n", mpt[i1][i2][i3][i4] + (v_addr & 0xfff));

    // 统计内存占用
    // sleep(100);

    for(int i = 0; i < 32; i++) {
        for(int j = 0; j < 32; j++) {
            for(int k = 0; k < 32; k++) {
                free(mpt[i][j][k]);
            }
            free(mpt[i][j]);
        }
        free(mpt[i]);
    }
    free(mpt);
    return 0;
}
