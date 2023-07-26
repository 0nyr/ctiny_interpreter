#include <stdio.h>

int main() { 
    int a; a = 0; 
    int y; y = 10;

    while (y > 0) { 
        if (a < 100) { a = a + y; } 
        y = y - 1; 
    } 
    
    printf("%d\n", a);
    return a; 
}