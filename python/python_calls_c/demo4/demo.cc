#include <iostream>
#include "demo.h"
using namespace std;

/**
 * build: 
 * g++ -o libdemo.so -shared -fPIC demo.cc
 */

TestFact::TestFact() {
    cout << "init." << endl;
    cout << "类初始化操作.." << endl;
}

TestFact::~TestFact() {
    delete this->a;
    this->a = NULL;
    cout << "destroy." << endl;
    cout << "做一些清理操作.." << endl;
}

// 做资源类型的初始化 如: 连接,内存池,显存初始化等耗时操作
void TestFact::inits() {
    cout << "资源初始化." << endl;
    this->a = new int;
    *a = 3;
}

void TestFact::printa() {
    cout << *(this->a) << endl;
}

int TestFact::fact(int n) {
    if(n <= 1) {
        return 1;
    } else {
        return n * TestFact::fact(n - 1);
    }
}

extern "C" {
    // 全局初始化 在python load so的时候就被执行
    // TestFact tf; // 栈实例化
    // 堆实例化
    TestFact *tf = new TestFact();
    // 其他资源初始化 需要在python中手动调用
    void inits() {
        tf->inits();
    }
    int test(int n) {
        return tf->fact(n);
    }
    int test1() {
        cout << "cout." << endl;
    }
    int test2() {
        tf->printa();
    }
    void close() {
        // 清理方法 注意不能直接写在下面会报错
        delete tf;
        tf = NULL;
    }
}
