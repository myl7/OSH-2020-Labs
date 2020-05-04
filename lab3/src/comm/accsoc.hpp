#pragma once

#include <stdexcept>

extern "C" {
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
}

constexpr int QUEUED_CON_N = 3;

class AccSoc {
public:
  explicit AccSoc(in_port_t port) {
    if ((fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
      throw std::runtime_error("Can not create a new socket.");
    }

    sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(port),
        .sin_addr = {.s_addr = INADDR_ANY},
    };

    if (bind(fd, (sockaddr *)&addr, sizeof(addr))) {
      throw std::runtime_error("Can not bind the socket to port " + std::to_string(port) + ".");
    }

    if (listen(fd, QUEUED_CON_N)) {
      throw std::runtime_error("Can not listen " + std::to_string(QUEUED_CON_N) + " connections on the socket.");
    }
  }

  ~AccSoc() {
    close(fd);
  }

  int accept() {
    auto serve_fd = ::accept(fd, nullptr, nullptr);

    if (serve_fd == -1) {
      throw std::runtime_error("Can not accept a connection on the socket.");
    }

    return serve_fd;
  }

  int get_fd() {
    return fd;
  }

private:
  int fd;
};
