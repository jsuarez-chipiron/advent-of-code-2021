#include <iostream>
#include <cstdio>

std::string generate_polymer(std::string &pol)
{
    size_t l = pol.size();
    for (int i=0; i<l-1; ++i)
    {
        std::printf("%c", pol[i]);
        std::printf("%c\n", pol[i+1]);
    }
    return "";
}

int main(int argc, char **argv)
{
    std::string init = "NNCB";
    generate_polymer(init);
    return 0;
}
