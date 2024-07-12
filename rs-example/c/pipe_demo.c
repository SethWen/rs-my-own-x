#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>

int main()
{
    int pipefd[2]; // 用于存储管道的读和写文件描述符
    pid_t pid;
    char buffer[256];

    // 创建管道
    if (pipe(pipefd) == -1)
    {
        perror("pipe");
        exit(EXIT_FAILURE);
    }

    // 创建子进程
    pid = fork();
    if (pid == -1)
    {
        perror("fork");
        exit(EXIT_FAILURE);
    }

    if (pid == 0)
    {
        // 子进程
        close(pipefd[0]); // 关闭子进程的读端

        char *message = "Hello from child process!";
        write(pipefd[1], message, strlen(message) + 1);
        close(pipefd[1]); // 关闭子进程的写端

        sleep(20);

        exit(EXIT_SUCCESS);
    }
    else
    {
        // 父进程
        close(pipefd[1]); // 关闭父进程的写端

        read(pipefd[0], buffer, sizeof(buffer));
        printf("Parent process received message: %s\n", buffer);
        close(pipefd[0]); // 关闭父进程的读端

        sleep(30);
        exit(EXIT_SUCCESS);
    }
}
