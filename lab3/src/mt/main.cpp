#include <iostream>
#include <vector>
#include <mutex>
#include <condition_variable>
#include <thread>
#include <queue>

#include <comm/accsoc.hpp>
#include <comm/srvsoc.hpp>

constexpr in_port_t port = 8000;
constexpr int CON_N = 32;

int fd_list[CON_N] = {};
std::mutex *mtx_list[CON_N];
std::condition_variable *cv_list[CON_N];
std::thread recv_thds[CON_N];
std::thread send_thds[CON_N];

void init() {
  for (int i = 0; i < CON_N; ++i) {
    mtx_list[i] = new std::mutex();
    cv_list[i] = new std::condition_variable();
  }
}

void clean() {
  for (int i = 0; i < CON_N; ++i) {
    delete mtx_list[i];
    delete cv_list[i];
  }
}

int main() {
  std::ios::sync_with_stdio(false);

  AccSoc accsoc(port);

  init();

  std::queue<std::string> q_list[CON_N] = {{}};

  for (int i = 0; i < CON_N; ++i) {
    fd_list[i] = accsoc.accept();

    std::cerr << "Accept: " << fd_list[i] << std::endl;

    // Send thread.
    send_thds[i] = std::move(std::thread([&, i] {
      while (true) {
        ssize_t len;
        std::string msg{};

        do {
          {
            std::unique_lock lock(*mtx_list[i]);
            cv_list[i]->wait(lock, [&] {return !q_list[i].empty();});
            msg = q_list[i].front();
            q_list[i].pop();
          }

          std::cerr << "Send: " << msg << std::endl;

          len = send_msg(msg + "\n", fd_list[i]);
        } while (len > 0);
      }
    }));

    // Recv thread.
    recv_thds[i] = std::move(std::thread([&, i] {
      while (true) {
        ssize_t len;
        std::string msg;

        while (std::tie(len, msg) = recv_msg(fd_list[i]), len > 0) {
          std::cerr << "Recv: " << msg << std::endl;

          for (int j = 0; j < CON_N; ++j) {
            if (j != i && fd_list[j] != 0) {
              {
                std::lock_guard lock(*mtx_list[j]);
                q_list[j].push(msg);
              }

              cv_list[j]->notify_one();
            }
          }
        }

        close(fd_list[i]);
        fd_list[i] = 0;
        fd_list[i] = accsoc.accept();
      }
    }));
  }

  for (int i = 0; i < CON_N; ++i) {
    recv_thds[i].join();
    send_thds[i].join();
  }

  clean();

  return 0;
}
