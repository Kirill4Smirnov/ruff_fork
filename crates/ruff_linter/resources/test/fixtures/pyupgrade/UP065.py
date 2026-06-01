import unittest
from unittest import TestCase as TC
from unittest.case import TestCase as CaseTestCase


class Base(unittest.TestCase):
    pass


class Direct(unittest.TestCase):
    def test_value(self):
        return 1

    def test_none(self):
        return None

    def test_bare(self):
        return

    def helper(self):
        return 1


class Aliased(TC):
    def runTest(self):
        return "value"


class Indirect(Base):
    def test_indirect(self):
        return object()


class SubmoduleCase(CaseTestCase):
    def test_submodule(self):
        return b"value"


class Async(unittest.IsolatedAsyncioTestCase):
    async def test_async(self):
        return 1


class NotATestCase:
    def test_method(self):
        return 1


def test_top_level():
    return 1


class Nested(unittest.TestCase):
    def test_nested(self):
        def inner():
            return 1

        return inner()
