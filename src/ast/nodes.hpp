#pragma once
#include <string>

class Node {
public:
    virtual ~Node() = default;
};

class NumberNode : public Node {
public:
    int value;
    NumberNode(int v) : value(v) {}
};
