#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <sys/msg.h>

#define MAX_EVENTS 10
#define PORT 8080
#define BUFFER_SIZE 1024

void handle_event(int epollfd, struct epoll_event *event);

int main()
{
    
    int server_fd, new_socket, epollfd;
    struct sockaddr_in address;
    int addrlen = sizeof(address);
    struct epoll_event event, events[MAX_EVENTS];

    // 创建服务器 socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0)
    {
        perror("socket failed!!!");
        exit(EXIT_FAILURE);
    }
    printf("Socket created server_fd %d\n", server_fd);

    // 绑定服务器 socket
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(PORT);
    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0)
    {
        perror("bind failed");
        exit(EXIT_FAILURE);
    }
    printf("Socket bound to port %d\n", PORT);

    // 监听连接
    if (listen(server_fd, 1) < 0)
    {
        perror("listen failed");
        exit(EXIT_FAILURE);
    }
    printf("Socket is listening\n");

    // 创建 epoll 实例
    if ((epollfd = epoll_create1(0)) < 0)
    {
        perror("epoll_create1 failed");
        exit(EXIT_FAILURE);
    }
    printf("Epoll instance created epollfd %d\n", epollfd);

    // 注册服务器 socket 到 epoll
    event.data.fd = server_fd;
    event.events = EPOLLIN;
    if (epoll_ctl(epollfd, EPOLL_CTL_ADD, server_fd, &event) < 0)
    {
        perror("epoll_ctl: add server_fd failed");
        exit(EXIT_FAILURE);
    }
    printf("Server socket registered to epoll\n");

    printf("Server is listening on port %d\n", PORT);

    while (1)
    {
        int nfds;
        printf("Waiting for events...\n");
        if ((nfds = epoll_wait(epollfd, events, MAX_EVENTS, -1)) < 0)
        {
            perror("epoll_wait failed");
            exit(EXIT_FAILURE);
        }
        printf("epoll_wait returned %d\n", nfds);

        for (int i = 0; i < nfds; i++)
        {
            if (events[i].data.fd == server_fd)
            {
                // 处理新的连接
                if ((new_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t *)&addrlen)) < 0)
                {
                    perror("accept failed");
                    exit(EXIT_FAILURE);
                }
                printf("New connection, socket fd is %d\n", new_socket);

                // 注册新连接的 socket 到 epoll
                event.data.fd = new_socket;
                event.events = EPOLLIN;
                if (epoll_ctl(epollfd, EPOLL_CTL_ADD, new_socket, &event) < 0)
                {
                    perror("epoll_ctl: add new_socket failed");
                    exit(EXIT_FAILURE);
                }
            }
            else
            {
                printf("Socket fd %d has data to read\n", events[i].data.fd);
                // 处理已连接 socket 的事件
                handle_event(epollfd, &events[i]);
            }
        }
    }

    close(server_fd);
    close(epollfd);
    return 0;
}

void handle_event(int epollfd, struct epoll_event *event)
{
    int sockfd = event->data.fd;
    char buffer[BUFFER_SIZE];
    int valread;

    if (event->events & EPOLLIN)
    {
        if ((valread = read(sockfd, buffer, BUFFER_SIZE)) <= 0)
        {
            // 连接关闭或错误
            close(sockfd);
            epoll_ctl(epollfd, EPOLL_CTL_DEL, sockfd, NULL);
            printf("Closed connection, socket fd is %d\n", sockfd);
        }
        else
        {
            // 处理读取的数据
            buffer[valread] = '\0';
            printf("Received data from socket fd %d: %s\n", sockfd, buffer);

            // 回显数据
            write(sockfd, buffer, valread);
        }
    }
}
