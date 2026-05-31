parimport abc
from abc import abstractclassmethod, abstractmethod
from abc import abstractclassmethod as acm


def other_decorator(func):
    return func


class A:
    @abstractclassmethod
    def f(cls): ...


class B:
    @abc.abstractclassmethod
    def g(cls): ...


class C:
    @acm
    def h(cls): ...


class D:
    @other_decorator
    @abc.abstractclassmethod  # comment
    def i(cls): ...


class E:
    @classmethod
    @abstractmethod
    def j(cls): ...
