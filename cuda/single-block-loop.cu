#include <stdio.h>

/*
 * Refactor `loop` to be a CUDA Kernel. The new kernel should
 * only do the work of 1 iteration of the original loop.
 */

/**
 * cpu实现
 */
void loop(int N)
{
  for (int i = 0; i < N; ++i)
  {
    printf("CPU This is iteration number %d\n", i);
  }
}
/**
 * gpu实现
 */
__global__ void loop()
{
    // 这里循环均适用block和线程替代来实现并行计算
    // blockDim表示一个块中的线程数量, 通过blockIdx.x * blockDim * x + threadIdx.x来实现循环区间索引.
    printf("GPU This is iteration number %d\n", blockIdx.x * blockDim.x + threadIdx.x);
}

int main()
{
  /*
   * When refactoring `loop` to launch as a kernel, be sure
   * to use the execution configuration to control how many
   * "iterations" to perform.
   *
   * For this exercise, only use 1 block of threads.
   */
  int N = 10;
  loop(N);
  loop<<<2, 10>>>();
  cudaDeviceSynchronize();
}
