#include <stdlib.h>
#include <stdio.h>

int main() {
    int ret = system("cp a /home/");
    
    if (ret == 0) {
        printf("Plik 'a' został skopiowany do /home/\n");
    } else {
        printf("Nie udało się skopiować pliku 'a'\n");
    }

    return 0;
}