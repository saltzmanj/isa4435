#include <stdio.h>


int main() {
	char strbuf[64];
	
	do {

		printf("> ");
		scanf("%s", strbuf);
		printf("%s", strbuf);

	} while(1);
}