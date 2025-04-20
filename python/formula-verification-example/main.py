#!/usr/bin/env python3
from formula_parse import *

def validate_formula(formula):
    try:
        lexer.input(formula)
        # 词法分析校验
        while True:
            tok = lexer.token()
            if not tok:
                break
        # 语法分析校验
        result = parser.parse(formula)
        print("公式校验通过！")
        return result
    except Exception as e:
        print(f"校验失败: {str(e)}")
        return None
    
if __name__ == '__main__':
    # 测试用例
    formulas = [
        "cos(length(name))/sin(age) + (5*pow(score, 2))",  # 有效公式
        "cos(length(name))/sin(age) + (5*pow(score,",      # 括号不匹配
        "invalid_function(age)",                            # 未知函数
        "field.name + 5",                                    # 含点号的字段名（需调整词法规则）
        "5^(2+2) - 3*-5",
        "3*2 + 6*2",
        "avg('abc', 3) * sum(2*bcd)/log(c*cos(bf)^3)"
    ]

    for formula in formulas:
        print(f"\n公式: {formula}")
        print(validate_formula(formula))
