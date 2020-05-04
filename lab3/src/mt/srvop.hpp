#pragma once

#include <string>

extern "C" {
#include <sys/socket.h>
}

static std::string recv_msg(int fd) {
  char c;
  std::string msg{};

  while (recv(fd, &c, 1, 0) > 0) {
    msg.push_back(c);

    if (c == '\n') {
      return msg;
    }
  }

  return std::string{};
}

static ssize_t send_msg(const std::string &msg, int fd) {
  return send(fd, &msg[0], msg.size(), 0);
}
