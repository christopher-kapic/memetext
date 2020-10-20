#include <iostream>
#include <stdlib.h>
using namespace std;

char* memeText(char* text){
    char* text_ = text;
    size_t size = sizeof(text_) / sizeof(text_[0]);

    for (int i = 0; i < size; i++) {
        if (rand() % 2 == 0){
            std::cout << "Random a\n";
            text_[i] = "a";
        } else {
            text_[i] = "b";
        }

    }

    return (text_);
}


int main(int argc, char *argv[]){
    if (!argv[1]){
        std::cerr << "Please provide an argument for memetext.\n";
    }
    
    std::cout << memeText(argv[1]);
    std::cout << "\n";
    return 0;
}
