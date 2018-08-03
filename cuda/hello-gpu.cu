#include <stdio.h>

/**
 * 第一个gpu程序: Hello GPU.
 * cuda运行方式: nvcc -arch=sm_70 -o hello-gpu hello-gpu.cu -run
 * 然后编译会生成: hello-gpu可执行文件, 之后可直接执行
 */

void helloCPU()
{
  printf("Hello from the CPU.\n");
}

/*
 * Refactor the `helloGPU` definition to be a kernel
 * that can be launched on the GPU. Update its message
 * to read "Hello from the GPU!"
 */
__global__ void helloGPU()
{
  printf("Hello also from the GPU.\n");
}

int main()
{

  helloCPU();

  /*
   * Refactor this call to `helloGPU` so that it launches
   * as a kernel on the GPU.
   */
  helloGPU<<<1, 1>>>();

  /*
   * Add code below to synchronize on the completion of the
   * `helloGPU` kernel completion before continuing the CPU
   * thread.
   */
  cudaDeviceSynchronize();
}
