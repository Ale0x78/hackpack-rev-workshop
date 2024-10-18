
#include <stdio.h> 
#include <stdlib.h> 
#include <string.h>

int validate(char* flag){ // FnpWZUaK2hVrO48Q
if ((int)flag[0] + (int)flag[1] != 180) return 1;
if ((int)flag[1] + (int)flag[2] != 222) return 1;
if ((int)flag[2] + (int)flag[3] != 199) return 1;
if ((int)flag[3] + (int)flag[4] != 177) return 1;
if ((flag[4] ^ 0x43) != 0x19) return 1;
if ((flag[5] ^ 0x75) != 0x20) return 1;
if ((flag[6] ^ 0x75) != 0x14) return 1;
if ((flag[7] ^ 0x66) != 0x2d) return 1;
if ((flag[8] ^ 0x32) != 0x0) return 1;
if (flag[9] != 'h') return 1;
if (flag[10] != 'V') return 1;
if ((int)flag[11] + (int)flag[12] != 193) return 1;
if ((int)flag[12] + (int)flag[13] != 131) return 1;
if ((int)flag[13] + (int)flag[14] != 108) return 1;
if ((int)flag[14] + (int)flag[15] != 137) return 1;

  return 0;
}

int main(void){
  char *flag = calloc(17, 1);
  bzero(flag, 17);
  fscanf(stdin, "%16s", flag);
  return validate(flag);
}
