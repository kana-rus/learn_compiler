#include <stdio.h>
#include <stdlib.h>

/*
加減算式の定義
- 最初に数字が1つある
- そのあとに0個以上の「項」が続いている
- 項というのは、+の後に数字が来ているものか、-の後に数字が来ているものである
*/

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "only 1 argument has tobe given.");
        return 1;
    }

    char *p = argv[1];
    /*
    strtol ... "str to long" (str, pointer, base)
        文字列 (C では char* という) の最初から読んでいって、
        数値に変換可能な部分までで自動的に打ち切り変換後の数値を返す。
        同時に pointer を文字列の次の文字を指すように書き換える。
    */

    printf(".intel_syntax noprefix\n");
    printf(".global main\n");
    printf("main:\n");
    printf("  mov rax, %ld\n", strtol(p, &p, 10));

    while (*p) {
        if (*p == '+') {
            p++;
            printf("  add rax, %ld\n", strtol(p, &p, 10));
            continue;
        }
        if (*p == '-') {
            p++;
            printf("  sub rax, %ld", strtol(p, &p, 10));;
            continue;
        }

        fprintf(stderr, "unexpected token: '%c'\n", *p);
        return 1;
    }
    
    printf("  ret\n");
    return 0;
}