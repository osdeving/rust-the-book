#include <stdio.h>
#include <string.h>

int main() {
    return 0;
}

char *pig_latin(char *s) {
    int new_size = 0;
    while(*s) {
        if(*s == ' ')
            new_size += 3;
    }

    char* new_s = (char*)malloc(strlen(s) + new_size);

    while(*s) {
        *new_s++ = *s;
        char letter = '\0';
        
        if(isalpha(*s)) {
            letter = *s;
        }


        if(letter == ' ') {

        }

        if(*letter == 'a' || *letter == 'e' || *letter == 'i' || *letter == 'o' || *letter == 'u') {
            
        }
    }
}