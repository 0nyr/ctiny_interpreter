// Import necessary libraries
#include <iostream>
#include <array>

int function_a(std::array<int, 10>& a, int b) {
    return a[9] + b;
}

int main() {
    std::array<int, 10> a;
    int i = 0;

    // Loop to populate and print the array
    while(i < 10) {
        a[i] = i;
        std::cout << "a[i]: " << a[i] << std::endl;
        i = i + 1;
    }

    std::cout << "a[9]: " << a[9] << std::endl;

    int b = 2;
    int c = function_a(a, b);
    std::cout << "c: " << c << std::endl;

    return 0;
}
