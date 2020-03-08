#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <stdlib.h>

int main() {
  pid_t pid1 = fork();
  if (pid1 == 0) {
    execl("/bin/1", NULL);
    exit(1);
  }
  waitpid(pid1, NULL, 0);

  pid_t pid2 = fork();
  if (pid2 == 0) {
    execl("/bin/2", NULL);
    exit(1);
  }
  waitpid(pid2, NULL, 0);

  pid_t pid3 = fork();
  if (pid3 == 0) {
    execl("/bin/3", NULL);
    exit(1);
  }
  waitpid(pid3, NULL, 0);

  while (1) {}

  return 0;
}
