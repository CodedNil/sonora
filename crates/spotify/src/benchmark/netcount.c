#define _GNU_SOURCE
#include <arpa/inet.h>
#include <dlfcn.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/uio.h>
#include <unistd.h>

static _Atomic uint64_t *counts;
static int internet(int fd) {
    struct sockaddr_storage addr;
    socklen_t len = sizeof(addr);
    if (getpeername(fd, (struct sockaddr *)&addr, &len) != 0) return 0;
    if (addr.ss_family == AF_INET)
        return (ntohl(((struct sockaddr_in *)&addr)->sin_addr.s_addr) >> 24) != 127;
    if (addr.ss_family == AF_INET6)
        return !IN6_IS_ADDR_LOOPBACK(&((struct sockaddr_in6 *)&addr)->sin6_addr);
    return 0;
}
static void record(int fd, ssize_t len, int outgoing) {
    if (counts && len > 0 && internet(fd))
        atomic_fetch_add_explicit(counts + outgoing, len, memory_order_relaxed);
}
__attribute__((constructor)) static void setup(void) {
    const char *path = getenv("SONORA_PROFILE_NETWORK");
    if (!path) return;
    int fd = open(path, O_RDWR | O_CREAT, 0600);
    if (fd < 0) return;
    if (ftruncate(fd, 16) == 0) {
        void *p = mmap(NULL, 16, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
        if (p != MAP_FAILED) counts = p;
    }
    close(fd);
}
#define WRAP(name, args, params, direction) \
ssize_t name args { \
    static ssize_t (*real) args; \
    if (!real) real = dlsym(RTLD_NEXT, #name); \
    ssize_t result = real params; \
    __typeof__(errno) saved = errno; \
    record(fd, result, direction); \
    errno = saved; \
    return result; \
}
#include <errno.h>
WRAP(read, (int fd, void *buf, size_t bytes), (fd,buf,bytes), 0)
WRAP(write, (int fd, const void *buf, size_t bytes), (fd,buf,bytes), 1)
WRAP(readv, (int fd, const struct iovec *iov, int n), (fd,iov,n), 0)
WRAP(writev, (int fd, const struct iovec *iov, int n), (fd,iov,n), 1)
WRAP(recv, (int fd, void *buf, size_t len, int flags), (fd,buf,len,flags), 0)
WRAP(send, (int fd, const void *buf, size_t len, int flags), (fd,buf,len,flags), 1)
WRAP(recvfrom, (int fd, void *buf, size_t len, int flags, struct sockaddr *addr, socklen_t *alen), (fd,buf,len,flags,addr,alen), 0)
WRAP(sendto, (int fd, const void *buf, size_t len, int flags, const struct sockaddr *addr, socklen_t alen), (fd,buf,len,flags,addr,alen), 1)
WRAP(recvmsg, (int fd, struct msghdr *msg, int flags), (fd,msg,flags), 0)
WRAP(sendmsg, (int fd, const struct msghdr *msg, int flags), (fd,msg,flags), 1)
