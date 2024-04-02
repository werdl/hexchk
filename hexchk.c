// hexchk.c - the same as src/hexchk.rs, with the same logic, but written in C
// for comparison purposes.

#include <stdio.h>

typedef unsigned char uint8_t;

char * hex_to_color(uint8_t hex, uint8_t min, uint8_t max) {
    if (hex < min || hex > max) {
        return "\x1b[0m";
    }

    int rgb = hex * (16581375 / (max - min));

    int r = (rgb >> 16) & 0xFF;
    int g = (rgb >> 8) & 0xFF;
    int b = rgb & 0xFF;

    char * color = (char *)malloc(16);
    sprintf(color, "\x1b[38;2;%d;%d;%dm", r, g, b);
    return color;
}

int main(int argc, char ** argv) {
    char * file = argc > 1 ? argv[1] : "/dev/stdin";
    FILE * fp = fopen(file, "r");

    if (fp == NULL) {
        printf("Error: could not open file %s\n", file);
        return 1;
    }

    int mode = argc > 2 ? 0 : 1;

    int c;

    uint8_t buffer[16];
    int offset = 0;

    while (1) {
        size_t bytes_read = fread(buffer, sizeof(uint8_t), 16, fp);

        if (bytes_read == 0) {
            printf("|");
            break;
        } else if (bytes_read > 0) {
            printf("|\x1b[0m\n%08x  ", offset);
            offset += bytes_read;
            for (int i = 0; i < 16; i++) {
                if (i < bytes_read) {
                    printf("%s%02x ", mode?"":hex_to_color(buffer[i], 0x20, 0x7e), buffer[i]);
                } else {
                    printf("   ");
                }
            } 

            printf(" |");

            for (int i = 0; i < 16; i++) {
                if (i < bytes_read) {
                    uint8_t c = buffer[i];
                    printf("%s%c", mode?"":hex_to_color(buffer[i], 0x20, 0x7e), c>=0x20&&c<=0x7e?c:'.');
                } else {
                    printf(" ");
                }
            }

        } else {
            printf("Error: could not read file\n");
            return 1;
        }
    }

    printf("\x1b[0m\n");
}