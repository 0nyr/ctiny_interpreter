// import print
#include <stdio.h>


int function_a(int a[10], int b) {
    return a[9] + b;
}

int main() {
    int a[10], i;

    i = 0;

    while(i < 10) {
        a[i] = i;
        printf("a[i]: %d \n", a[i]);
        i = i + 1;
    }

    // print(a[9]);
    printf("a[9]: %d \n", a[9]);

    int b = 2;
    int c = function_a(a, b);
    printf("c: %d \n", c);

    return 0;
}