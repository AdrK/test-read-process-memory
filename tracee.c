#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/prctl.h>
#include <sys/ptrace.h>
#include <sys/reg.h>
#include <sys/types.h>
#include <sys/user.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

int counter = 0;

int main(int argc, char *argv[]) {
  struct timespec ts = {.tv_sec = 0, .tv_nsec = 1e8};
  for (counter = 0;; counter++) {
    printf("value: %d address: %p\n", counter, &counter);
    nanosleep(&ts, NULL);
  }
}
