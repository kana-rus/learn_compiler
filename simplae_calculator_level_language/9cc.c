#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
加減算式の定義
- 最初に数字が1つある
- そのあとに0個以上の「項」が続いている
- 項というのは、+の後に数字が来ているものか、-の後に数字が来ているものである
*/

typedef enum {
    TK_RESREVED, // 記号
    TK_NUM,
    TK_EOF,
} TokenKind;
typedef struct Token Token;
struct Token {
    TokenKind kind;
    Token *next;
    int val; // kind が TK_NUM の場合
    char *str;
};

Token *token; // the token current focused

// error reporting func
void error(char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    exit(1);
}
char *user_input;
void error_at(char *loc/* location */, char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);

    int pos = loc - user_input; /* ポインタの差をとって、エラー箇所が先頭から何バイト目かわかる */
    fprintf(stderr, "%s\n", user_input);
    fprintf(stderr, "%*s", pos, " "); // pos個の空白を出力
    fprintf(stderr, "^ ");
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    exit(1);
}

bool consume(char op) {
    if (token->kind != TK_RESREVED || token->str[0] != op) return false;
    token = token->next;
    return true;
}
void expect(char op) {
    if (token->kind != TK_RESREVED || token->str[0] != op)
        // error("Not '%c'", op);
        error_at(token->str, "expected %c", op);
    token = token->next;
}
int expect_number() {
    if (token->kind != TK_NUM)
        // error("This is not a number");
        error_at(token->str, "expected a number");
    int val = token->val;
    token = token->next;
    return val;
}

bool at_eof() {
    return token->kind == TK_EOF;
}

Token *new_token(TokenKind kind, Token *cur/* current token */, char *str) {
    Token *tok = calloc/*メモリ割り当て & ゼロクリア*/(1, sizeof(Token)); /* new token */
    tok->kind = kind;
    tok->str = str;

    cur->next = tok; // なるほど

    return tok;
}


Token *tokenize(/* char *p */) {
    char *p = user_input;
    Token head;
    head.next = NULL;
    Token *cur = &head;

    while (*p) {
        if (isspace(*p)) {
            p++;
            continue;
        }
        if (*p == '+' || *p == '-') {
            cur = new_token(TK_RESREVED, cur, p++);
            continue;
        }
        if (isdigit(*p)) {
            cur = new_token(TK_NUM, cur, p);
            cur->val = strtol(p, &p, 10);
            continue;
        }

        // error("Cannot tokenize");
        error_at(p, "expected a number");
    }
    
    new_token(TK_EOF, cur, p);
    return head.next;
}



int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "only 1 argument has tobe given.");
        return 1;
    }

    user_input = argv[1];
    token = tokenize();
    /*
    strtol ... "str to long" (str, pointer, base)
        文字列 (C では char* という) の最初から読んでいって、
        数値に変換可能な部分までで自動的に打ち切り変換後の数値を返す。
        同時に pointer を文字列の次の文字を指すように書き換える。
    */

    printf(".intel_syntax noprefix\n");
    printf(".global main\n");
    printf("main:\n");
    printf("  mov rax, %d\n", expect_number());

    while (!at_eof()) {
        if (consume('+')) {
            printf("  add rax, %d\n", expect_number());
            continue;
        }
        expect('-');
        printf("  sub rax, %d\n", expect_number());
    }
    
    printf("  ret\n");
    return 0;
}