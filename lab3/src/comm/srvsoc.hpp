#pragma once

#include <string>
#include <tuple>

extern "C" {
#include <sys/socket.h>
#include <netinet/in.h>
}

static std::tuple<ssize_t, std::string> recv_msg(int fd) {
  ssize_t len;
  char c;
  std::string msg{};

  while ((len = recv(fd, &c, 1, 0)) > 0) {
    if (c == '\n') {
      return std::make_tuple(len, msg);
    } else {
      msg.push_back(c);
    }
  }

  return std::make_tuple(len, msg);
}

constexpr int SEND_PKG_LEN = 1000;

static ssize_t send_msg(const std::string &msg, int fd) {
  int msglen = msg.size();
  int i = 0;
  ssize_t len;

  while (msglen - i >= SEND_PKG_LEN) {
    if ((len = send(fd, &msg[0] + i, SEND_PKG_LEN, 0)) <= 0) {
      return len;
    }
    i += SEND_PKG_LEN;
  }

  if ((len = send(fd, &msg[0] + i, msglen - i, 0)) <= 0) {
    return len;
  } else {
    return 1;
  }
}
