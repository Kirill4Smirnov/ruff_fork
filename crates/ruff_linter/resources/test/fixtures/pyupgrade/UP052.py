import codecs
import codecs as c
from codecs import open
from codecs import open as codecs_open

codecs.open("file.txt")
c.open("file.txt")
open("file.txt")
codecs_open("file.txt")


def open(path):
    return path


open("shadowed.txt")


class codecs:
    @staticmethod
    def open(path):
        return path


codecs.open("shadowed.txt")


class Custom:
    def open(self, path):
        return path


custom = Custom()
custom.open("shadowed.txt")
