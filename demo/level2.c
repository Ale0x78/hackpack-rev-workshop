#include <stdio.h>
#include <string.h>

const char* getFlag() {
  char blob[] = "\x29\x20\x20\x3d\x26\x27\x3d\x23\x23\x21\x20\x00";
  for(char* p = blob; *p != '\x0'; p++)
  {
    *p = (*p^0x10);
  }
  return blob;
}

void passcheck(char* buffer) {
  if(strcmp(buffer, getFlag()) == 0) {
    printf("Correct!");
  } else {
    printf("Bztssss");
  }
 }

int main(int argc, char** argv)
{
  if(argc <= 1) {
    printf("Please provide Kiwi's SSN as the second parameter!");
    return 1;
  }

  passcheck(argv[1]);
 
  return 0;
}
