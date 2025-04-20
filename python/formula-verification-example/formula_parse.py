# coding=utf-8

from ply import lex
from ply import yacc

tokens = [
    'NUMBER',        # 数字
    'FIELD',         # 数据库字段名
    'FUNCTION',      # 函数名
    # 'OPERATOR',      # 运算符
    'PLUS', # +
    'MINUS', # -
    'TIMES', # *
    'DIVIDE', # /
    'POWER', # ^
    'REM', # %
    'PAREN_LEFT',    # 左括号
    'PAREN_RIGHT',   # 右括号
    'COMMA',          # 逗号
    'SINGLE_QUOTES'   # 单引号
]


t_PAREN_LEFT  = r'\('
t_PAREN_RIGHT = r'\)'
# t_OPERATOR    = r'[+\-*/%^]'
t_COMMA       = r','
t_PLUS = r'\+'
t_MINUS = r'\-'
t_TIMES = r'\*'
t_DIVIDE = r'/'
t_POWER = r'\^'
t_REM = r'%'
t_SINGLE_QUOTES = r'\''

# 数字匹配（支持整数和浮点数）
def t_NUMBER(t):
    r'\d+\.?\d*[eE]?\d*|\.\d+[eE]?\d*'
    try:
        if t.value.isdigit():
            t.value = int(t.value)
        else:
            t.value = float(t.value)
    except ValueError:
        print(f"非法数字: {t.value}")
        t.value = None
    return t


# 字段名匹配（字母开头，允许字母、数字、下划线）
def t_FIELD(t):
    r'[a-zA-Z_][a-zA-Z0-9_]*'
    # 区分字段名和函数名（函数名需在预定义列表中）
    if t.value.lower() in ['cos', 'sin', 'length', 'pow', 'avg', 'sum', 'log']:
        t.type = 'FUNCTION'
    else:
        t.type = 'FIELD'
    return t

# 忽略空格和制表符
t_ignore  = ' \t'

# 记录行号
def t_newline(t):
    r'\n+'
    t.lexer.lineno += len(t.value)

# 计算列号
def find_column(input, token):
    line_start = input.rfind('\n', 0, token.lexpos) + 1
    return (token.lexpos - line_start) + 1

# 错误处理
def t_error(t):
    # print(f"非法字符: {t.value[0]}")
    # 跳过错误
    # t.lexer.skip(1)
    pass

lexer = lex.lex()


# precedence = (
#     ('left', 'PLUS', 'MINUS'),
#     ('left', 'TIMES', 'DIVIDE', 'REM'),
#     ('left', 'POWER'),
#     ('left', 'PAREN_LEFT', 'PAREN_RIGHT'),
#     ('right', 'UMINUS')  # MINUS term %prec UMINUS
# )


def p_experssion(p):
    '''expression : expression PLUS term
                  | expression MINUS term 
                  | term
       term       : term TIMES power
                  | term DIVIDE power
                  | term REM power
                  | power'''

    if len(p) == 2:
        p[0] = (p[1])
    else:
        p[0] = (p[2], p[1], p[3])

def p_power(p):
    '''power : factor
             | power POWER factor'''
    if len(p) == 2:
        p[0] = p[1]
    else:
        p[0] = f'{p[1]}^{p[3]}'
    

def p_factor(p):
    '''factor : NUMBER
              | FIELD
              | function_call
              | PAREN_LEFT expression PAREN_RIGHT
              | MINUS factor'''
    if p[1] == '(':
        p[0] = p[2]
    elif p[1] == '-':
        p[0] = f'-{p[2]}'
    else:
        p[0] = p[1]

def p_function_call(p):
    '''function_call : FUNCTION PAREN_LEFT arguments PAREN_RIGHT'''
    p[0] = ('function', p[1], p[3])

def p_arguments(p):
    '''arguments : expression
                 | arguments COMMA expression
                 | SINGLE_QUOTES arguments SINGLE_QUOTES'''
    if len(p) == 2:
        p[0] = [p[1]]
    elif p[1] == "'":
        p[0] = p[2]
    else:
        p[0] = p[1] + [p[3]]


def p_error(p):
    if p:
        column = find_column(p.lexer.lexdata, p)
        raise SyntaxError(f"语法错误: 意外的标记 {p.type} 出现在 {p.lineno}: {column}, value: {p.value}")
    else:
        raise SyntaxError("语法错误: 意外的文件结束")

parser = yacc.yacc()

