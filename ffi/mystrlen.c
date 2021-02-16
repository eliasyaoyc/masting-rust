//
// Created by eliasyao on 2021/2/16.
//

unsigned int mystrlen(char *str) {
    unsigned int c;
    for (c = 0; *str != '\0'; c++, *str++);
    return c;
}