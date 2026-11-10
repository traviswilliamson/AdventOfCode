#pragma once
#include <string>

struct serverResponse {
    bool success;
    std::string data;
    serverResponse(int code) : data(""), success(code == 0) {};
    serverResponse(std::string d) : data(d), success(true) {};
};

class server {

  public:
    static serverResponse fetchData(unsigned short day);
};
