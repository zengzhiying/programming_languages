#include<iostream>
#include "half.hpp"

int main() {
    // 9595 -> 0.0214
    // 41834 -> -0.0145
    // 9457 -> 0.0193
    unsigned short v = 9457;
    auto n = halfToFloat(v);
    float f = *(float *)&n;
    std::cout << f << std::endl;

    float f1 = 0.0193;
    auto s = floatToHalf(*(int *)&f1);
    std::cout << s << std::endl;
    
    return 0;
}
