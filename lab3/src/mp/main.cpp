#include <iostream>

#include <comm/accsoc.hpp>
#include <mp/srvsoc.hpp>

extern "C" {
#include <sys/epoll.h>
}

constexpr in_port_t port = 8000;
constexpr int CON_N = 32;

int main() {
  std::ios::sync_with_stdio(false);

  AccSoc accsoc(port, CON_N, true);
  if (accsoc.init() != 0) {
    exit(-1);
  }
  const auto accfd = accsoc.get_fd();

  std::vector<SrvSoc> srv_list{};

  auto in_epfd = epoll_create(CON_N + 1);
  epoll_event acc_event = {.events = EPOLLIN, .data = {.fd = accfd}};
  epoll_ctl(in_epfd, EPOLL_CTL_ADD, accfd, &acc_event);

  auto out_epfd = epoll_create(CON_N);

  epoll_event event = {};

  while (true) {
    bool to_send = false;
    for (const auto &srv : srv_list) {
      if (srv.to_send()) {
        to_send = true;
        break;
      }
    }

    int has_in;
    if (to_send) {
      has_in = epoll_wait(in_epfd, &event, 1, 0);
    } else {
      has_in = epoll_wait(in_epfd, &event, 1, -1);
    }

    if (has_in > 0) {
      if (event.data.fd == accfd) {
        int fd = accsoc.accept();

        if (fd > 0) {
          if (srv_list.size() < CON_N) {
            srv_list.emplace_back(fd);

            epoll_event add{};

            add = {.events = EPOLLIN, .data = {.fd = fd}};
            epoll_ctl(in_epfd, EPOLL_CTL_ADD, fd, &add);

            add = {.events = EPOLLOUT, .data = {.fd = fd}};
            epoll_ctl(out_epfd, EPOLL_CTL_ADD, fd, &add);
          } else {
            close(fd);
          }
        }
      } else {
        for (auto i = srv_list.begin(); i != srv_list.end(); ++i) {
          auto fd = (*i).get_fd();

          if (fd == event.data.fd) {
            auto len = (*i).recv([&, fd](const std::string &msg){
              for (auto &srv : srv_list) {
                if (srv.get_fd() != fd) {
                  srv.msg_q += msg;
                }
              }
            });

            if (len <= 0 && errno != EAGAIN) {
              srv_list.erase(i);
              epoll_ctl(in_epfd, EPOLL_CTL_DEL, fd, nullptr);
              epoll_ctl(out_epfd, EPOLL_CTL_DEL, fd, nullptr);
            }

            break;
          }
        }
      }
    }

    if (to_send) {
      for (auto &srv : srv_list) {
        if (srv.to_send()) {
          auto len = srv.send();

          if (len <= 0 && errno != EAGAIN) {
            srv.msg_q = "";
          }
        }
      }
    }
  }
}
