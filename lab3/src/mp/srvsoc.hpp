#pragma once

#include <queue>
#include <string>
#include <functional>

extern "C" {
#include <sys/epoll.h>
#include <sys/socket.h>
}

class SrvSoc {
private:
  int fd;

public:
  std::string msg{};
  std::string msg_q{};

  explicit SrvSoc(int fd_) : fd(fd_) {}

  ssize_t recv(const std::function<void(const std::string &)> &callback) {
    ssize_t len;
    char c;

    while ((len = ::recv(fd, &c, 1, MSG_DONTWAIT)) > 0) {
      msg.push_back(c);

      if (c == '\n') {
        callback(msg);
        msg.clear();
      }
    }

    return len;
  }

  ssize_t send() {
    ssize_t len = ::send(fd, &msg_q[0], msg_q.size(), MSG_DONTWAIT);

    if (len > 0) {
      msg_q = msg_q.substr(len);
    }

    return len;
  }

  [[nodiscard]] bool to_send() const {
    return !msg_q.empty();
  }

  int get_fd() {
    return fd;
  }
};
