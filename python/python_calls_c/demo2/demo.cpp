#include<iostream>
using namespace std;
#include "/usr/include/python2.7/Python.h"

/**
 * g++ -o libdemo.so -shared -fPIC demo.cpp
 */

class TestFact {
    public:
        TestFact(){};
        ~TestFact(){};
        int fact(int n);
};
int TestFact::fact(int n) {
    if(n <= 1)
        return 1;
    else
        return n * TestFact::fact(n - 1);
}

int test(int n) {
    TestFact *tf = new TestFact();
    int rs = tf->fact(n);
    delete tf;
    tf = NULL;
    return rs;
}

PyObject * wrap_test(PyObject *self, PyObject *args){
    int n, result;
    if(!PyArg_ParseTuple(args, "i:test", &n))
        return NULL;
    result = test(n);
    return Py_BuildValue("i", result);
}

static PyMethodDef demoMethods[] = 
{
    {"test", wrap_test, METH_VARARGS, "Caculate N!"},
    {NULL, NULL}
};

extern "C"
void initlibdemo()
{
    PyObject *m;
    m = Py_InitModule("libdemo", demoMethods);
}

/**
int main() {
    TestFact t;
    cout << t.fact(4) << endl;
    return 0;
}
*/
