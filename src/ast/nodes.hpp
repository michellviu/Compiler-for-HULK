#include <math.h>
#pragma once

class ASTNode {
public:
    virtual ~ASTNode() = default;
    virtual int eval() = 0;
};

class NumberNode : public ASTNode {
    int value;
public:
    NumberNode(int v) : value(v) {}
    int eval() override { return value; }
};

class BinaryOpNode : public ASTNode {
    char op;
    ASTNode* left;
    ASTNode* right;
public:
    BinaryOpNode(char op, ASTNode* l, ASTNode* r) 
        : op(op), left(l), right(r) {}
    
    ~BinaryOpNode() {
        delete left;
        delete right;
    }

    int eval() override {
        int l = left->eval();
        int r = right->eval();
        switch(op) {
            case '+': return l + r;
            case '-': return l - r;
            case '*': return l * r;
            case '/': return l / r;
            case '^': return pow(l, r);
            default: return 0;
        }
    }
};