import abc
from abc import abstractmethod, abstractproperty
from abc import abstractproperty as ap


def other_decorator(func):
    return func


class A:
    @abstractproperty
    def f(self): ...


class B:
    @abc.abstractproperty
    def g(self): ...


class C:
    @ap
    def h(self): ...


class D:
    @other_decorator
    @abc.abstractproperty  # comment
    def i(self): ...


class E:
    @property
    @abstractmethod
    def j(self): ...
