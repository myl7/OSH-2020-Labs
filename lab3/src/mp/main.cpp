#include <iostream>
#include <queue>
#include <string>
#include <vector>
#include <tuple>

#include <comm/accsoc.hpp>
#include <comm/srvsoc.hpp>

extern "C" {
#include <sys/epoll.h>
}

constexpr in_port_t port = 8000;
constexpr int CON_N = 32;

int main() {
  std::ios::sync_with_stdio(false);

  AccSoc accsoc(port);
  const auto accfd = accsoc.get_fd();

  std::vector<std::tuple <int, std::queue<std::string>>> fd_list(CON_N);

  auto in_epfd = epoll_create(CON_N + 1);
  epoll_event acc_event = {.events = EPOLLIN, .data = {.fd = accfd}};
  epoll_ctl(in_epfd, EPOLL_CTL_ADD, accfd, &acc_event);

  auto out_epfd = epoll_create(CON_N);

  epoll_event event[1] = {{}};
  while (epoll_wait(in_epfd, event, 1, -1)) {
    if (event->data.fd == accfd) {
      if (fd_list.size() < CON_N) {
        int fd = accsoc.accept();
        fd_list.emplace_back(fd, std::queue<std::string>{});

        epoll_event add{};

        add = {.events = EPOLLIN, .data = {.fd = fd}};
        epoll_ctl(in_epfd, EPOLL_CTL_ADD, fd, &add);

        add = {.events = EPOLLOUT, .data = {.fd = fd}};
        epoll_ctl(out_epfd, EPOLL_CTL_ADD, fd, &add);
      }
    } else {
      ssize_t len;
      std::string msg;
      std::tie(len, msg) = recv_msg(event->data.fd);

      for (auto i = fd_list.begin(); i != fd_list.end(); ++i) {
        int fd;
        std::queue<std::string> q;
        std::tie(fd, q) = (*i);

        if (fd == event->data.fd) {
          if (len <= 0) {
            epoll_ctl(in_epfd, EPOLL_CTL_DEL, event->data.fd, nullptr);
            epoll_ctl(out_epfd, EPOLL_CTL_DEL, event->data.fd, nullptr);
            fd_list.erase(i);
          } else {
            q.push(msg);
          }

          break;
        }
      }
    }
  }

  return 0;
}
