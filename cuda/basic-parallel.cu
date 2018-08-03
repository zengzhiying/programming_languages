#include <stdio.h>

/*
 * 最简单的基于cuda在gpu并行运行的函数
 */
__global__ void firstParallel()
{
  printf("This should be running in parallel.\n");
}

int main()
{
  firstParallel<<<1, 10>>>();
  cudaDeviceSynchronize();

  return 0;
}
