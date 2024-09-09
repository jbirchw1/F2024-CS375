// Short program to generate an unsorted array in the specified format
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main(int argc, char *argv[]) {
    if(argc > 2) {
        printf("ID-10-t Error: Too many arguments (%d) provided.\n", argc);
        return 1;
    }

    srand(time(NULL));

    int length = atoi(argv[1]);
    printf("%d\n", length);

    int temp = 0;
    for(int i = 0; i < length; i++) {
        int max = length*2;
        int min = -(temp) + 1;
        int rd_num = rand() % (max - min + 1) + min;
        temp = temp + rd_num;
        printf("%d\n", temp);
    }

    return 0;
}