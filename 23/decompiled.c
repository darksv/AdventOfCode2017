#include <stdbool.h>

int main() {
  int a = 1;
  int b = 67;
  int c = b;
  int h = 0;
  int muls =0;
  if (a != 0) {
   	b = b * 100 + 100000;
   	muls++;
  	c = b + 17000;
  }


  for (; ; b += 17) {
  	bool flag = true;
  	
  	for (int d = 2; d != b; d++) {
  	  if (b % d == 0) {
  	    flag = false;
  	  }
  	  
  	 // for (int e = 2; e != b; e++) {
  	 //   muls++;
    //   	if (d * e == b) {
    //   	  flag = false;
    //   	}
      	
      	
  	 // }
  	}
  	
  	if (!flag) {
  	 	h += 1;
  	}
  	
  	if (b == c) {
  		break;
  	}
  }
  printf("%d\n", h);
  printf("%d\n", muls);
}