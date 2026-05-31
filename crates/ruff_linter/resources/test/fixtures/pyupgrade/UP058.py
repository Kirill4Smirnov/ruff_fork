import abc
from abc import abstractmethod, abstractstaticmethod
from abc import abstractstaticmethod as asm


def other_decorator(func):
    return func


class A:
    @abstractstaticmethod
    def f(): ...


class B:
    @abc.abstractstaticmethod
    def g(): ...


class C:
    @asm
    def h(): ...


class D:
    @other_decorator
    @abc.abstractstaticmethod  # comment
    def i(): ...


class E:
    @staticmethod
    @abstractmethod
    def j(): ...
