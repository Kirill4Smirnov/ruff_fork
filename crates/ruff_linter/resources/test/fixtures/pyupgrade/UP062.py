import select
from select import EPOLL_CLOEXEC, epoll
from select import epoll as ep


select.epoll()
select.epoll(10)
select.epoll(10, 0)
select.epoll(flags=0)
select.epoll(sizehint=10, flags=select.EPOLL_CLOEXEC)
epoll(flags=EPOLL_CLOEXEC)
ep(10, EPOLL_CLOEXEC)


class Wrapper:
    def epoll(self, flags=0):
        return None


Wrapper().epoll(flags=0)
