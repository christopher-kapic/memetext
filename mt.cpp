// #include <iostream>

// char *finalString(char *s)
// {
// }

// int main(int argc, char *argv[])
// {
//     if (!argv[1])
//     {
//         fprintf(stderr, "Please provide text for\n");
//         return 1;
//     }

//     return 0;
// }

#include <iostream>
#include <stdlib.h>
using namespace std;

char* memeText(char* text){
    char* text_ = text;
    for (; *text_ != "\0"; text_++) {
        if (rand() % 2 == 0){
            *text_ = "e";
        } else {
            *text_ = "i";
        }
    }
    return (*text_);
}

int main(int argc, char *argv[]){
    if (!argv[1]){
        std::cerr << "Please provide an argument for memetext.\n";
    }

    return memeText(argv[1]);
}
