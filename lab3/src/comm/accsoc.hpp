#pragma once

extern "C" {
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <fcntl.h>
}

class AccSoc {
private:
  in_port_t port;
  int queued_con_n;
  int fd = 0;
  bool nonblock;

public:
  AccSoc(in_port_t port_, int queued_con_n_, bool nonblock_ = false)
      : port(port_), queued_con_n(queued_con_n_), nonblock(nonblock_) {}

  int init() {
    if ((fd = socket(AF_INET, SOCK_STREAM, 0)) == -1) {
      return -1;
    }

    sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(port),
        .sin_addr = {.s_addr = INADDR_ANY},
    };

    if (bind(fd, (sockaddr *)&addr, sizeof(addr))) {
      return -2;
    }

    if (listen(fd, queued_con_n)) {
      return -3;
    }

    if (nonblock) {
      if (fcntl(fd, F_SETFL, O_NONBLOCK) == -1) {
        return -4;
      }
    }

    return 0;
  }

  ~AccSoc() {
    close(fd);
  }

  int accept() {
    auto srv_fd = ::accept(fd, nullptr, nullptr);

    if (srv_fd == -1) {
      return -1;
    }

    return srv_fd;
  }

  int get_fd() {
    return fd;
  }
};
