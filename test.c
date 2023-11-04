#include <stdio.h>
// This will print any argument it parses just for testing purposes.
// You can see this as a gist at https://gist.github.com/drHyperion451/61cf0f736c1bb2acae605e24351f9fa0
int main(int argc, char *argv[]) {
    for (int i = 1; i < argc; i++) {
        printf("Argument %d: %s\n", i, argv[i]);
    }
    return 0;
}
