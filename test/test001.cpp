#include <iostream>

typedef char char_t;

struct Temp {
    void blah(int a, int b);
private:
    int m_a;
};

const double the_const_d = 666.42;
constexpr unsigned the_const_unsigned = 44;
static const char* the_const_string = "really???";
const double& the_const_ref_d = the_const_d;
static const char& the_const_ref_char = the_const_string[0];

int main()
{
    char c = 'a';
    char_t b = 'b';
    char_t&& bb = 'b';
    char_t *d = nullptr;
    const char* blah = "blah";
    float f = 11;
    float& g = f;
    float* h = &g;
    float*& i = h;
    int x = 10;
    std::cout << x << std::endl;
    return 0;
}
